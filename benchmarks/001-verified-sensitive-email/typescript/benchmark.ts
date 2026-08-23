export type Result<T, E> =
  | { ok: true; value: T }
  | { ok: false; error: E };

export class RawEmail {
  constructor(public readonly value: string) {}
}

export class SyntaxValidEmail {
  // Private nominal marker prevents ordinary structural substitution.
  private readonly __syntaxValid!: void;

  private constructor(public readonly value: string) {}

  static parse(raw: RawEmail): Result<SyntaxValidEmail, EmailSyntaxError> {
    const value = raw.value.trim();

    // Deliberately modest syntax rule: the benchmark is about semantic state,
    // not RFC-complete email parsing.
    const looksLikeEmail = /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value);

    if (!looksLikeEmail) {
      return {
        ok: false,
        error: new EmailSyntaxError(value),
      };
    }

    return {
      ok: true,
      value: new SyntaxValidEmail(value),
    };
  }

  /**
   * Identity-relevant transformation.
   *
   * The result is syntax-valid only. Ownership verification is intentionally
   * absent and must be re-established for the changed address.
   */
  replaceDomain(newDomain: string): Result<SyntaxValidEmail, EmailSyntaxError> {
    const [local] = this.value.split("@", 2);
    return SyntaxValidEmail.parse(new RawEmail(`${local}@${newDomain}`));
  }
}

export class EmailSyntaxError extends Error {
  constructor(public readonly input: string) {
    super(`Email syntax is invalid: ${input}`);
    this.name = "EmailSyntaxError";
  }
}

export type AccountId = string;

export interface OwnershipVerification {
  readonly accountId: AccountId;
  readonly subjectValue: string;
  readonly establishedAtMs: number;
  readonly expiresAtMs: number;
  readonly evidenceId: string;
}

export class VerifiedEmail {
  private readonly __ownershipVerified!: void;

  private constructor(
    public readonly email: SyntaxValidEmail,
    public readonly verification: OwnershipVerification,
  ) {}

  static verifyOwnership(
    email: SyntaxValidEmail,
    accountId: AccountId,
    establishedAtMs: number,
    ttlMs: number,
    evidenceId: string,
  ): VerifiedEmail {
    return new VerifiedEmail(email, {
      accountId,
      subjectValue: email.value,
      establishedAtMs,
      expiresAtMs: establishedAtMs + ttlMs,
      evidenceId,
    });
  }
}

/**
 * Explicit effect capability.
 *
 * This benchmark does not claim cryptographic or security isolation. The
 * purpose is to make the effect requirement visible at the API boundary.
 */
export class NetworkSendCapability {
  private readonly __networkSend!: void;

  private constructor(public readonly issuedTo: string) {}

  static issue(issuedTo: string): NetworkSendCapability {
    return new NetworkSendCapability(issuedTo);
  }
}

export interface DeliveryReceipt {
  readonly recipient: string;
  readonly accountId: AccountId;
  readonly evidenceId: string;
  readonly sentAtMs: number;
  readonly effect: "network_send";
}

export type SendError =
  | {
      kind: "subject_mismatch";
      message: string;
    }
  | {
      kind: "scope_mismatch";
      expectedAccountId: AccountId;
      verificationAccountId: AccountId;
      message: string;
    }
  | {
      kind: "verification_stale";
      expiredAtMs: number;
      nowMs: number;
      message: string;
    };

export function sendSensitiveMessage(
  recipient: VerifiedEmail,
  requiredAccountId: AccountId,
  nowMs: number,
  _networkSend: NetworkSendCapability,
): Result<DeliveryReceipt, SendError> {
  const verification = recipient.verification;

  // Defensive runtime check even though normal construction binds the
  // verification to this exact value.
  if (verification.subjectValue !== recipient.email.value) {
    return {
      ok: false,
      error: {
        kind: "subject_mismatch",
        message:
          "Ownership evidence belongs to a different email value than the recipient.",
      },
    };
  }

  if (verification.accountId !== requiredAccountId) {
    return {
      ok: false,
      error: {
        kind: "scope_mismatch",
        expectedAccountId: requiredAccountId,
        verificationAccountId: verification.accountId,
        message: `Ownership was verified for account ${verification.accountId}, not ${requiredAccountId}.`,
      },
    };
  }

  if (nowMs > verification.expiresAtMs) {
    return {
      ok: false,
      error: {
        kind: "verification_stale",
        expiredAtMs: verification.expiresAtMs,
        nowMs,
        message: `Ownership verification expired at ${verification.expiresAtMs}; current time is ${nowMs}.`,
      },
    };
  }

  return {
    ok: true,
    value: {
      recipient: recipient.email.value,
      accountId: requiredAccountId,
      evidenceId: verification.evidenceId,
      sentAtMs: nowMs,
      effect: "network_send",
    },
  };
}
