#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawEmail(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmailSyntaxError {
    pub input: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxValidEmail {
    value: String,
}

impl SyntaxValidEmail {
    pub fn parse(raw: RawEmail) -> Result<Self, EmailSyntaxError> {
        let value = raw.0.trim().to_owned();

        // Deliberately modest syntax rule: this benchmark is about semantic
        // state and guarantees, not RFC-complete email parsing.
        let mut parts = value.split('@');
        let local = parts.next().unwrap_or_default();
        let domain = parts.next().unwrap_or_default();
        let has_extra_at = parts.next().is_some();
        let valid = !local.is_empty()
            && !domain.is_empty()
            && domain.contains('.')
            && !value.chars().any(char::is_whitespace)
            && !has_extra_at;

        if !valid {
            return Err(EmailSyntaxError { input: value });
        }

        Ok(Self { value })
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Identity-relevant transformation.
    ///
    /// The changed address remains syntax-valid but does not retain ownership
    /// verification. Re-verification is required.
    pub fn replace_domain(&self, new_domain: &str) -> Result<Self, EmailSyntaxError> {
        let local = self.value.split('@').next().unwrap_or_default();
        Self::parse(RawEmail(format!("{local}@{new_domain}")))
    }
}

pub type AccountId = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnershipVerification {
    account_id: AccountId,
    subject_value: String,
    established_at_ms: u64,
    expires_at_ms: u64,
    evidence_id: String,
}

impl OwnershipVerification {
    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    pub fn established_at_ms(&self) -> u64 {
        self.established_at_ms
    }

    pub fn expires_at_ms(&self) -> u64 {
        self.expires_at_ms
    }

    pub fn evidence_id(&self) -> &str {
        &self.evidence_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedEmail {
    email: SyntaxValidEmail,
    verification: OwnershipVerification,
}

impl VerifiedEmail {
    pub fn verify_ownership(
        email: SyntaxValidEmail,
        account_id: impl Into<AccountId>,
        established_at_ms: u64,
        ttl_ms: u64,
        evidence_id: impl Into<String>,
    ) -> Self {
        let account_id = account_id.into();
        let subject_value = email.value.clone();

        Self {
            email,
            verification: OwnershipVerification {
                account_id,
                subject_value,
                established_at_ms,
                expires_at_ms: established_at_ms.saturating_add(ttl_ms),
                evidence_id: evidence_id.into(),
            },
        }
    }

    pub fn email(&self) -> &SyntaxValidEmail {
        &self.email
    }

    pub fn verification(&self) -> &OwnershipVerification {
        &self.verification
    }
}

/// Explicit token representing permission to perform the benchmark's
/// `network_send` effect.
///
/// The benchmark does not claim this token is a security boundary. It makes
/// the effect requirement visible at the API boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkSendCapability {
    issued_to: String,
}

impl NetworkSendCapability {
    pub fn issue(issued_to: impl Into<String>) -> Self {
        Self {
            issued_to: issued_to.into(),
        }
    }

    pub fn issued_to(&self) -> &str {
        &self.issued_to
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeliveryReceipt {
    pub recipient: String,
    pub account_id: AccountId,
    pub evidence_id: String,
    pub sent_at_ms: u64,
    pub effect: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SendError {
    SubjectMismatch {
        message: String,
    },
    ScopeMismatch {
        expected_account_id: AccountId,
        verification_account_id: AccountId,
        message: String,
    },
    VerificationStale {
        expired_at_ms: u64,
        now_ms: u64,
        message: String,
    },
}

/// Send a sensitive message using an ownership-verified address.
///
/// The function signature makes syntax/ownership state and the network effect
/// capability explicit. Freshness and account scope remain runtime-dependent
/// checks.
///
/// I1 — raw input is not accepted by the type system:
///
/// ```compile_fail
/// use aytham_benchmark_verified_email::{
///     send_sensitive_message, NetworkSendCapability, RawEmail,
/// };
///
/// let raw = RawEmail("person@example.org".to_owned());
/// let network = NetworkSendCapability::issue("example");
/// let _ = send_sensitive_message(&raw, "user-42", 0, &network);
/// ```
///
/// I2 — syntax-valid but unverified input is not accepted:
///
/// ```compile_fail
/// use aytham_benchmark_verified_email::{
///     send_sensitive_message, NetworkSendCapability, RawEmail, SyntaxValidEmail,
/// };
///
/// let parsed = SyntaxValidEmail::parse(
///     RawEmail("person@example.org".to_owned())
/// ).unwrap();
/// let network = NetworkSendCapability::issue("example");
/// let _ = send_sensitive_message(&parsed, "user-42", 0, &network);
/// ```
///
/// I7 — the network capability is required:
///
/// ```compile_fail
/// use aytham_benchmark_verified_email::{
///     send_sensitive_message, RawEmail, SyntaxValidEmail, VerifiedEmail,
/// };
///
/// let parsed = SyntaxValidEmail::parse(
///     RawEmail("person@example.org".to_owned())
/// ).unwrap();
/// let verified = VerifiedEmail::verify_ownership(
///     parsed, "user-42", 0, 60_000, "challenge-1"
/// );
/// let _ = send_sensitive_message(&verified, "user-42", 1_000);
/// ```
pub fn send_sensitive_message(
    recipient: &VerifiedEmail,
    required_account_id: &str,
    now_ms: u64,
    _network_send: &NetworkSendCapability,
) -> Result<DeliveryReceipt, SendError> {
    let verification = &recipient.verification;

    // Defensive check. Normal safe construction binds evidence to this exact
    // address and does not provide a public way to detach/reapply it.
    if verification.subject_value != recipient.email.value {
        return Err(SendError::SubjectMismatch {
            message: "Ownership evidence belongs to a different email value than the recipient."
                .to_owned(),
        });
    }

    if verification.account_id != required_account_id {
        return Err(SendError::ScopeMismatch {
            expected_account_id: required_account_id.to_owned(),
            verification_account_id: verification.account_id.clone(),
            message: format!(
                "Ownership was verified for account {}, not {}.",
                verification.account_id, required_account_id
            ),
        });
    }

    if now_ms > verification.expires_at_ms {
        return Err(SendError::VerificationStale {
            expired_at_ms: verification.expires_at_ms,
            now_ms,
            message: format!(
                "Ownership verification expired at {}; current time is {}.",
                verification.expires_at_ms, now_ms
            ),
        });
    }

    Ok(DeliveryReceipt {
        recipient: recipient.email.value.clone(),
        account_id: required_account_id.to_owned(),
        evidence_id: verification.evidence_id.clone(),
        sent_at_ms: now_ms,
        effect: "network_send",
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const T0: u64 = 1_000_000;
    const TTL: u64 = 60_000;

    fn verified_email() -> VerifiedEmail {
        let parsed = SyntaxValidEmail::parse(RawEmail(
            "person@example.org".to_owned(),
        ))
        .expect("baseline email should parse");

        VerifiedEmail::verify_ownership(
            parsed,
            "user-42",
            T0,
            TTL,
            "challenge-001",
        )
    }

    #[test]
    fn valid_send_succeeds() {
        let verified = verified_email();
        let network = NetworkSendCapability::issue("benchmark-test");

        let receipt = send_sensitive_message(
            &verified,
            "user-42",
            T0 + 1_000,
            &network,
        )
        .expect("valid verified send should succeed");

        assert_eq!(receipt.effect, "network_send");
        assert_eq!(receipt.recipient, "person@example.org");
    }

    #[test]
    fn verification_cannot_be_detached_to_another_public_value() {
        let verified_a = verified_email();
        let parsed_b = SyntaxValidEmail::parse(RawEmail(
            "other@example.org".to_owned(),
        ))
        .expect("second email should parse");

        assert_ne!(verified_a.email().as_str(), parsed_b.as_str());

        // I3: there is intentionally no public constructor that combines
        // `parsed_b` with `verified_a.verification()`.
        // `parsed_b` remains SyntaxValidEmail and cannot be sent until a new
        // `VerifiedEmail::verify_ownership` operation establishes ownership.
    }

    #[test]
    fn identity_relevant_mutation_drops_verification_state() {
        let verified = verified_email();
        let changed = verified
            .email()
            .replace_domain("attacker.example")
            .expect("changed email should remain syntax-valid");

        assert_eq!(changed.as_str(), "person@attacker.example");

        // I4: `changed` is SyntaxValidEmail, not VerifiedEmail. There is no
        // safe API that preserves the old ownership proof across this change.
    }

    #[test]
    fn stale_verification_is_rejected() {
        let verified = verified_email();
        let network = NetworkSendCapability::issue("benchmark-test");

        let error = send_sensitive_message(
            &verified,
            "user-42",
            T0 + TTL + 1,
            &network,
        )
        .expect_err("stale verification should fail");

        assert!(matches!(error, SendError::VerificationStale { .. }));
    }

    #[test]
    fn wrong_scope_is_rejected() {
        let verified = verified_email();
        let network = NetworkSendCapability::issue("benchmark-test");

        let error = send_sensitive_message(
            &verified,
            "user-77",
            T0 + 1_000,
            &network,
        )
        .expect_err("wrong-scope verification should fail");

        assert!(matches!(error, SendError::ScopeMismatch { .. }));
    }
}
