use std::marker::PhantomData;

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
}

pub type AccountId = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Absent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Present;

#[derive(Debug, Clone, PartialEq, Eq)]
struct OwnershipFact {
    subject_value: String,
    account_id: AccountId,
    established_at_ms: u64,
    expires_at_ms: u64,
    evidence_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MarketingConsentFact {
    subject_value: String,
    account_id: AccountId,
    granted_at_ms: u64,
    evidence_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MfaFact {
    subject_value: String,
    account_id: AccountId,
    established_at_ms: u64,
    expires_at_ms: u64,
    evidence_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct JurisdictionFact {
    subject_value: String,
    account_id: AccountId,
    jurisdiction: String,
    established_at_ms: u64,
    expires_at_ms: u64,
    evidence_id: String,
}

/// Strong ordinary Rust baseline for independent facts.
///
/// O = ownership_verified
/// M = marketing_consent
/// F = mfa_verified
/// J = jurisdiction_allowed
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmailFacts<O, M, F, J> {
    email: SyntaxValidEmail,
    ownership: Option<OwnershipFact>,
    marketing_consent: Option<MarketingConsentFact>,
    mfa: Option<MfaFact>,
    jurisdiction: Option<JurisdictionFact>,
    _state: PhantomData<(O, M, F, J)>,
}

impl EmailFacts<Absent, Absent, Absent, Absent> {
    pub fn from(email: SyntaxValidEmail) -> Self {
        Self {
            email,
            ownership: None,
            marketing_consent: None,
            mfa: None,
            jurisdiction: None,
            _state: PhantomData,
        }
    }
}

impl<O, M, F, J> EmailFacts<O, M, F, J> {
    pub fn email(&self) -> &SyntaxValidEmail {
        &self.email
    }

    pub fn verify_ownership(
        self,
        account_id: impl Into<AccountId>,
        established_at_ms: u64,
        ttl_ms: u64,
        evidence_id: impl Into<String>,
    ) -> EmailFacts<Present, M, F, J> {
        let Self {
            email,
            marketing_consent,
            mfa,
            jurisdiction,
            ..
        } = self;
        let account_id = account_id.into();
        let subject_value = email.value.clone();

        EmailFacts {
            email,
            ownership: Some(OwnershipFact {
                subject_value,
                account_id,
                established_at_ms,
                expires_at_ms: established_at_ms.saturating_add(ttl_ms),
                evidence_id: evidence_id.into(),
            }),
            marketing_consent,
            mfa,
            jurisdiction,
            _state: PhantomData,
        }
    }

    pub fn grant_marketing_consent(
        self,
        account_id: impl Into<AccountId>,
        granted_at_ms: u64,
        evidence_id: impl Into<String>,
    ) -> EmailFacts<O, Present, F, J> {
        let Self {
            email,
            ownership,
            mfa,
            jurisdiction,
            ..
        } = self;
        let account_id = account_id.into();
        let subject_value = email.value.clone();

        EmailFacts {
            email,
            ownership,
            marketing_consent: Some(MarketingConsentFact {
                subject_value,
                account_id,
                granted_at_ms,
                evidence_id: evidence_id.into(),
            }),
            mfa,
            jurisdiction,
            _state: PhantomData,
        }
    }

    pub fn verify_mfa(
        self,
        account_id: impl Into<AccountId>,
        established_at_ms: u64,
        ttl_ms: u64,
        evidence_id: impl Into<String>,
    ) -> EmailFacts<O, M, Present, J> {
        let Self {
            email,
            ownership,
            marketing_consent,
            jurisdiction,
            ..
        } = self;
        let account_id = account_id.into();
        let subject_value = email.value.clone();

        EmailFacts {
            email,
            ownership,
            marketing_consent,
            mfa: Some(MfaFact {
                subject_value,
                account_id,
                established_at_ms,
                expires_at_ms: established_at_ms.saturating_add(ttl_ms),
                evidence_id: evidence_id.into(),
            }),
            jurisdiction,
            _state: PhantomData,
        }
    }

    pub fn allow_jurisdiction(
        self,
        account_id: impl Into<AccountId>,
        jurisdiction: impl Into<String>,
        established_at_ms: u64,
        ttl_ms: u64,
        evidence_id: impl Into<String>,
    ) -> EmailFacts<O, M, F, Present> {
        let Self {
            email,
            ownership,
            marketing_consent,
            mfa,
            ..
        } = self;
        let account_id = account_id.into();
        let subject_value = email.value.clone();

        EmailFacts {
            email,
            ownership,
            marketing_consent,
            mfa,
            jurisdiction: Some(JurisdictionFact {
                subject_value,
                account_id,
                jurisdiction: jurisdiction.into(),
                established_at_ms,
                expires_at_ms: established_at_ms.saturating_add(ttl_ms),
                evidence_id: evidence_id.into(),
            }),
            _state: PhantomData,
        }
    }
}

impl<O, F, J> EmailFacts<O, Present, F, J> {
    /// Remove exactly the marketing-consent fact while retaining the static
    /// presence/absence knowledge of the other three dimensions.
    pub fn revoke_marketing_consent(self) -> EmailFacts<O, Absent, F, J> {
        EmailFacts {
            email: self.email,
            ownership: self.ownership,
            marketing_consent: None,
            mfa: self.mfa,
            jurisdiction: self.jurisdiction,
            _state: PhantomData,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkSendCapability {
    _issued_to: String,
}

impl NetworkSendCapability {
    pub fn issue(issued_to: impl Into<String>) -> Self {
        Self {
            _issued_to: issued_to.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrthogonalSendError {
    SubjectMismatch {
        fact: &'static str,
    },
    ScopeMismatch {
        fact: &'static str,
        expected_account_id: AccountId,
        actual_account_id: AccountId,
    },
    OwnershipStale {
        expired_at_ms: u64,
        now_ms: u64,
    },
    MfaStale {
        expired_at_ms: u64,
        now_ms: u64,
    },
    JurisdictionStale {
        expired_at_ms: u64,
        now_ms: u64,
    },
    JurisdictionMismatch {
        required: String,
        actual: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrthogonalDeliveryReceipt {
    pub action: &'static str,
    pub recipient: String,
    pub account_id: AccountId,
    pub sent_at_ms: u64,
    pub effect: &'static str,
}

trait BoundFact {
    fn subject_value(&self) -> &str;
    fn account_id(&self) -> &str;
}

impl BoundFact for OwnershipFact {
    fn subject_value(&self) -> &str {
        &self.subject_value
    }
    fn account_id(&self) -> &str {
        &self.account_id
    }
}

impl BoundFact for MarketingConsentFact {
    fn subject_value(&self) -> &str {
        &self.subject_value
    }
    fn account_id(&self) -> &str {
        &self.account_id
    }
}

impl BoundFact for MfaFact {
    fn subject_value(&self) -> &str {
        &self.subject_value
    }
    fn account_id(&self) -> &str {
        &self.account_id
    }
}

impl BoundFact for JurisdictionFact {
    fn subject_value(&self) -> &str {
        &self.subject_value
    }
    fn account_id(&self) -> &str {
        &self.account_id
    }
}

fn validate_bound_fact<T: BoundFact>(
    email: &SyntaxValidEmail,
    fact_name: &'static str,
    fact: &T,
    required_account_id: &str,
) -> Result<(), OrthogonalSendError> {
    if fact.subject_value() != email.value {
        return Err(OrthogonalSendError::SubjectMismatch { fact: fact_name });
    }

    if fact.account_id() != required_account_id {
        return Err(OrthogonalSendError::ScopeMismatch {
            fact: fact_name,
            expected_account_id: required_account_id.to_owned(),
            actual_account_id: fact.account_id().to_owned(),
        });
    }

    Ok(())
}

fn validate_ownership(
    email: &SyntaxValidEmail,
    fact: &OwnershipFact,
    required_account_id: &str,
    now_ms: u64,
) -> Result<(), OrthogonalSendError> {
    validate_bound_fact(email, "ownership_verified", fact, required_account_id)?;

    if now_ms > fact.expires_at_ms {
        return Err(OrthogonalSendError::OwnershipStale {
            expired_at_ms: fact.expires_at_ms,
            now_ms,
        });
    }

    Ok(())
}

/// Requires ownership + MFA, but not marketing consent or jurisdiction.
pub fn send_security_alert<M, J>(
    recipient: &EmailFacts<Present, M, Present, J>,
    required_account_id: &str,
    now_ms: u64,
    _network_send: &NetworkSendCapability,
) -> Result<OrthogonalDeliveryReceipt, OrthogonalSendError> {
    let ownership = recipient
        .ownership
        .as_ref()
        .expect("Present ownership marker must carry OwnershipFact");
    validate_ownership(&recipient.email, ownership, required_account_id, now_ms)?;

    let mfa = recipient
        .mfa
        .as_ref()
        .expect("Present MFA marker must carry MfaFact");
    validate_bound_fact(&recipient.email, "mfa_verified", mfa, required_account_id)?;

    if now_ms > mfa.expires_at_ms {
        return Err(OrthogonalSendError::MfaStale {
            expired_at_ms: mfa.expires_at_ms,
            now_ms,
        });
    }

    Ok(OrthogonalDeliveryReceipt {
        action: "security_alert",
        recipient: recipient.email.value.clone(),
        account_id: required_account_id.to_owned(),
        sent_at_ms: now_ms,
        effect: "network_send",
    })
}

/// Requires ownership + marketing consent.
///
/// O1 / O2 — ownership + MFA or ownership + jurisdiction does not satisfy the
/// marketing requirement:
///
/// ```compile_fail
/// use aytham_benchmark_orthogonal_facts::{
///     EmailFacts, NetworkSendCapability, RawEmail, SyntaxValidEmail,
///     send_marketing_message,
/// };
///
/// let email = SyntaxValidEmail::parse(RawEmail("person@example.org".into())).unwrap();
/// let state = EmailFacts::from(email)
///     .verify_ownership("user-42", 0, 10_000, "own")
///     .verify_mfa("user-42", 0, 10_000, "mfa");
/// let net = NetworkSendCapability::issue("test");
/// let _ = send_marketing_message(&state, "user-42", 1, &net);
/// ```
pub fn send_marketing_message<F, J>(
    recipient: &EmailFacts<Present, Present, F, J>,
    required_account_id: &str,
    now_ms: u64,
    _network_send: &NetworkSendCapability,
) -> Result<OrthogonalDeliveryReceipt, OrthogonalSendError> {
    let ownership = recipient
        .ownership
        .as_ref()
        .expect("Present ownership marker must carry OwnershipFact");
    validate_ownership(&recipient.email, ownership, required_account_id, now_ms)?;

    let consent = recipient
        .marketing_consent
        .as_ref()
        .expect("Present marketing marker must carry MarketingConsentFact");
    validate_bound_fact(
        &recipient.email,
        "marketing_consent",
        consent,
        required_account_id,
    )?;

    Ok(OrthogonalDeliveryReceipt {
        action: "marketing_message",
        recipient: recipient.email.value.clone(),
        account_id: required_account_id.to_owned(),
        sent_at_ms: now_ms,
        effect: "network_send",
    })
}

/// Requires ownership + jurisdiction approval.
pub fn send_regulated_notice<M, F>(
    recipient: &EmailFacts<Present, M, F, Present>,
    required_account_id: &str,
    required_jurisdiction: &str,
    now_ms: u64,
    _network_send: &NetworkSendCapability,
) -> Result<OrthogonalDeliveryReceipt, OrthogonalSendError> {
    let ownership = recipient
        .ownership
        .as_ref()
        .expect("Present ownership marker must carry OwnershipFact");
    validate_ownership(&recipient.email, ownership, required_account_id, now_ms)?;

    let jurisdiction = recipient
        .jurisdiction
        .as_ref()
        .expect("Present jurisdiction marker must carry JurisdictionFact");
    validate_bound_fact(
        &recipient.email,
        "jurisdiction_allowed",
        jurisdiction,
        required_account_id,
    )?;

    if now_ms > jurisdiction.expires_at_ms {
        return Err(OrthogonalSendError::JurisdictionStale {
            expired_at_ms: jurisdiction.expires_at_ms,
            now_ms,
        });
    }

    if jurisdiction.jurisdiction != required_jurisdiction {
        return Err(OrthogonalSendError::JurisdictionMismatch {
            required: required_jurisdiction.to_owned(),
            actual: jurisdiction.jurisdiction.clone(),
        });
    }

    Ok(OrthogonalDeliveryReceipt {
        action: "regulated_notice",
        recipient: recipient.email.value.clone(),
        account_id: required_account_id.to_owned(),
        sent_at_ms: now_ms,
        effect: "network_send",
    })
}

/// O3 — after marketing consent is revoked, the resulting state cannot satisfy
/// `send_marketing_message` even though ownership/MFA/jurisdiction remain.
///
/// ```compile_fail
/// use aytham_benchmark_orthogonal_facts::{
///     EmailFacts, NetworkSendCapability, RawEmail, SyntaxValidEmail,
///     send_marketing_message,
/// };
///
/// let email = SyntaxValidEmail::parse(RawEmail("person@example.org".into())).unwrap();
/// let state = EmailFacts::from(email)
///     .verify_ownership("user-42", 0, 10_000, "own")
///     .verify_mfa("user-42", 0, 10_000, "mfa")
///     .grant_marketing_consent("user-42", 0, "consent")
///     .allow_jurisdiction("user-42", "IN", 0, 10_000, "jur")
///     .revoke_marketing_consent();
/// let net = NetworkSendCapability::issue("test");
/// let _ = send_marketing_message(&state, "user-42", 1, &net);
/// ```
pub fn orthogonal_compile_fail_marker() {}

#[cfg(test)]
mod tests {
    use super::*;

    const T0: u64 = 2_000_000;
    const OWNERSHIP_TTL: u64 = 120_000;
    const MFA_TTL: u64 = 30_000;
    const JURISDICTION_TTL: u64 = 120_000;

    fn parsed_email() -> SyntaxValidEmail {
        SyntaxValidEmail::parse(RawEmail("person@example.org".to_owned()))
            .expect("benchmark email should parse")
    }

    fn all_facts() -> EmailFacts<Present, Present, Present, Present> {
        EmailFacts::from(parsed_email())
            .verify_ownership("user-42", T0, OWNERSHIP_TTL, "ownership-001")
            .verify_mfa("user-42", T0, MFA_TTL, "mfa-001")
            .grant_marketing_consent("user-42", T0, "consent-001")
            .allow_jurisdiction(
                "user-42",
                "IN",
                T0,
                JURISDICTION_TTL,
                "jurisdiction-001",
            )
    }

    #[test]
    fn ownership_and_mfa_are_enough_for_security_only() {
        let state = EmailFacts::from(parsed_email())
            .verify_ownership("user-42", T0, OWNERSHIP_TTL, "ownership-001")
            .verify_mfa("user-42", T0, MFA_TTL, "mfa-001");
        let net = NetworkSendCapability::issue("test");

        let result = send_security_alert(&state, "user-42", T0 + 1_000, &net);
        assert!(result.is_ok());
    }

    #[test]
    fn revoking_marketing_preserves_other_static_dimensions() {
        let state = all_facts().revoke_marketing_consent();
        let net = NetworkSendCapability::issue("test");

        assert!(send_security_alert(&state, "user-42", T0 + 1_000, &net).is_ok());
        assert!(send_regulated_notice(
            &state,
            "user-42",
            "IN",
            T0 + 1_000,
            &net,
        )
        .is_ok());
    }

    #[test]
    fn stale_mfa_blocks_security_but_not_marketing() {
        let state = EmailFacts::from(parsed_email())
            .verify_ownership("user-42", T0, OWNERSHIP_TTL, "ownership-001")
            .verify_mfa("user-42", T0, 500, "mfa-short")
            .grant_marketing_consent("user-42", T0, "consent-001");
        let net = NetworkSendCapability::issue("test");

        let security = send_security_alert(&state, "user-42", T0 + 1_000, &net)
            .expect_err("stale MFA should block security");
        assert!(matches!(security, OrthogonalSendError::MfaStale { .. }));

        assert!(send_marketing_message(&state, "user-42", T0 + 1_000, &net).is_ok());
    }

    #[test]
    fn jurisdiction_mismatch_is_independent() {
        let state = all_facts();
        let net = NetworkSendCapability::issue("test");

        let error = send_regulated_notice(
            &state,
            "user-42",
            "EU",
            T0 + 1_000,
            &net,
        )
        .expect_err("wrong jurisdiction should fail");

        assert!(matches!(
            error,
            OrthogonalSendError::JurisdictionMismatch { .. }
        ));
    }

    #[test]
    fn second_email_starts_without_any_fact_state() {
        let other = SyntaxValidEmail::parse(RawEmail("other@example.org".to_owned()))
            .expect("second email should parse");
        let state = EmailFacts::from(other);

        assert_eq!(state.email().as_str(), "other@example.org");
        // The type is EmailFacts<Absent, Absent, Absent, Absent>; there is no
        // public operation that copies the private fact payload from another
        // EmailFacts value into it.
    }
}
