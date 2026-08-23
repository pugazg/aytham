import {
  AccountId,
  NetworkSendCapability,
  Result,
  SyntaxValidEmail,
} from "./benchmark";

/**
 * Orthogonal-facts extension for Benchmark 001.
 *
 * This deliberately gives TypeScript a strong ordinary baseline: fact presence
 * is tracked with four independent generic state dimensions rather than one
 * named wrapper type for every possible combination.
 */

interface BoundFact {
  readonly subjectValue: string;
  readonly accountId: AccountId;
  readonly evidenceId: string;
}

interface OwnershipFact extends BoundFact {
  readonly establishedAtMs: number;
  readonly expiresAtMs: number;
}

interface MarketingConsentFact extends BoundFact {
  readonly grantedAtMs: number;
}

interface MfaFact extends BoundFact {
  readonly establishedAtMs: number;
  readonly expiresAtMs: number;
}

interface JurisdictionFact extends BoundFact {
  readonly jurisdiction: string;
  readonly establishedAtMs: number;
  readonly expiresAtMs: number;
}

/**
 * O = ownership_verified present
 * M = marketing_consent present
 * F = mfa_verified present
 * J = jurisdiction_allowed present
 */
export class EmailFacts<
  O extends boolean,
  M extends boolean,
  F extends boolean,
  J extends boolean,
> {
  // Makes the generic state dimensions part of nominal compatibility.
  private readonly __factState!: [O, M, F, J];

  private constructor(
    public readonly email: SyntaxValidEmail,
    private readonly ownership?: OwnershipFact,
    private readonly marketingConsent?: MarketingConsentFact,
    private readonly mfa?: MfaFact,
    private readonly jurisdiction?: JurisdictionFact,
  ) {}

  static from(
    email: SyntaxValidEmail,
  ): EmailFacts<false, false, false, false> {
    return new EmailFacts<false, false, false, false>(email);
  }

  verifyOwnership(
    accountId: AccountId,
    establishedAtMs: number,
    ttlMs: number,
    evidenceId: string,
  ): EmailFacts<true, M, F, J> {
    return new EmailFacts<true, M, F, J>(
      this.email,
      {
        subjectValue: this.email.value,
        accountId,
        establishedAtMs,
        expiresAtMs: establishedAtMs + ttlMs,
        evidenceId,
      },
      this.marketingConsent,
      this.mfa,
      this.jurisdiction,
    );
  }

  grantMarketingConsent(
    accountId: AccountId,
    grantedAtMs: number,
    evidenceId: string,
  ): EmailFacts<O, true, F, J> {
    return new EmailFacts<O, true, F, J>(
      this.email,
      this.ownership,
      {
        subjectValue: this.email.value,
        accountId,
        grantedAtMs,
        evidenceId,
      },
      this.mfa,
      this.jurisdiction,
    );
  }

  verifyMfa(
    accountId: AccountId,
    establishedAtMs: number,
    ttlMs: number,
    evidenceId: string,
  ): EmailFacts<O, M, true, J> {
    return new EmailFacts<O, M, true, J>(
      this.email,
      this.ownership,
      this.marketingConsent,
      {
        subjectValue: this.email.value,
        accountId,
        establishedAtMs,
        expiresAtMs: establishedAtMs + ttlMs,
        evidenceId,
      },
      this.jurisdiction,
    );
  }

  allowJurisdiction(
    accountId: AccountId,
    jurisdiction: string,
    establishedAtMs: number,
    ttlMs: number,
    evidenceId: string,
  ): EmailFacts<O, M, F, true> {
    return new EmailFacts<O, M, F, true>(
      this.email,
      this.ownership,
      this.marketingConsent,
      this.mfa,
      {
        subjectValue: this.email.value,
        accountId,
        jurisdiction,
        establishedAtMs,
        expiresAtMs: establishedAtMs + ttlMs,
        evidenceId,
      },
    );
  }

  /**
   * Independent invalidation transition.
   *
   * Marketing consent is removed while the exact generic knowledge of
   * ownership, MFA and jurisdiction is preserved.
   */
  revokeMarketingConsent(
    this: EmailFacts<O, true, F, J>,
  ): EmailFacts<O, false, F, J> {
    return new EmailFacts<O, false, F, J>(
      this.email,
      this.ownership,
      undefined,
      this.mfa,
      this.jurisdiction,
    );
  }

  ownershipFact(
    this: EmailFacts<true, M, F, J>,
  ): OwnershipFact {
    return this.ownership!;
  }

  marketingConsentFact(
    this: EmailFacts<O, true, F, J>,
  ): MarketingConsentFact {
    return this.marketingConsent!;
  }

  mfaFact(
    this: EmailFacts<O, M, true, J>,
  ): MfaFact {
    return this.mfa!;
  }

  jurisdictionFact(
    this: EmailFacts<O, M, F, true>,
  ): JurisdictionFact {
    return this.jurisdiction!;
  }
}

export type OrthogonalSendError =
  | {
      kind: "subject_mismatch";
      fact: "ownership_verified" | "marketing_consent" | "mfa_verified" | "jurisdiction_allowed";
      message: string;
    }
  | {
      kind: "scope_mismatch";
      fact: "ownership_verified" | "marketing_consent" | "mfa_verified" | "jurisdiction_allowed";
      expectedAccountId: AccountId;
      actualAccountId: AccountId;
      message: string;
    }
  | {
      kind: "ownership_stale";
      expiredAtMs: number;
      nowMs: number;
      message: string;
    }
  | {
      kind: "mfa_stale";
      expiredAtMs: number;
      nowMs: number;
      message: string;
    }
  | {
      kind: "jurisdiction_stale";
      expiredAtMs: number;
      nowMs: number;
      message: string;
    }
  | {
      kind: "jurisdiction_mismatch";
      required: string;
      actual: string;
      message: string;
    };

export interface OrthogonalDeliveryReceipt {
  readonly action:
    | "security_alert"
    | "marketing_message"
    | "regulated_notice";
  readonly recipient: string;
  readonly accountId: AccountId;
  readonly sentAtMs: number;
  readonly effect: "network_send";
}

function validateBoundFact(
  email: SyntaxValidEmail,
  factName: OrthogonalSendError extends { kind: "subject_mismatch"; fact: infer N }
    ? N
    : never,
  fact: BoundFact,
  requiredAccountId: AccountId,
): OrthogonalSendError | null {
  if (fact.subjectValue !== email.value) {
    return {
      kind: "subject_mismatch",
      fact: factName,
      message: `${factName} belongs to a different email value.`,
    };
  }

  if (fact.accountId !== requiredAccountId) {
    return {
      kind: "scope_mismatch",
      fact: factName,
      expectedAccountId: requiredAccountId,
      actualAccountId: fact.accountId,
      message: `${factName} applies to account ${fact.accountId}, not ${requiredAccountId}.`,
    };
  }

  return null;
}

function validateOwnership(
  email: SyntaxValidEmail,
  fact: OwnershipFact,
  requiredAccountId: AccountId,
  nowMs: number,
): OrthogonalSendError | null {
  const bound = validateBoundFact(
    email,
    "ownership_verified",
    fact,
    requiredAccountId,
  );
  if (bound) return bound;

  if (nowMs > fact.expiresAtMs) {
    return {
      kind: "ownership_stale",
      expiredAtMs: fact.expiresAtMs,
      nowMs,
      message: "ownership_verified exists but is stale.",
    };
  }

  return null;
}

export function sendSecurityAlert<
  M extends boolean,
  J extends boolean,
>(
  recipient: EmailFacts<true, M, true, J>,
  requiredAccountId: AccountId,
  nowMs: number,
  _networkSend: NetworkSendCapability,
): Result<OrthogonalDeliveryReceipt, OrthogonalSendError> {
  const ownershipError = validateOwnership(
    recipient.email,
    recipient.ownershipFact(),
    requiredAccountId,
    nowMs,
  );
  if (ownershipError) return { ok: false, error: ownershipError };

  const mfa = recipient.mfaFact();
  const mfaBound = validateBoundFact(
    recipient.email,
    "mfa_verified",
    mfa,
    requiredAccountId,
  );
  if (mfaBound) return { ok: false, error: mfaBound };

  if (nowMs > mfa.expiresAtMs) {
    return {
      ok: false,
      error: {
        kind: "mfa_stale",
        expiredAtMs: mfa.expiresAtMs,
        nowMs,
        message: "mfa_verified exists but is stale.",
      },
    };
  }

  return {
    ok: true,
    value: {
      action: "security_alert",
      recipient: recipient.email.value,
      accountId: requiredAccountId,
      sentAtMs: nowMs,
      effect: "network_send",
    },
  };
}

export function sendMarketingMessage<
  F extends boolean,
  J extends boolean,
>(
  recipient: EmailFacts<true, true, F, J>,
  requiredAccountId: AccountId,
  nowMs: number,
  _networkSend: NetworkSendCapability,
): Result<OrthogonalDeliveryReceipt, OrthogonalSendError> {
  const ownershipError = validateOwnership(
    recipient.email,
    recipient.ownershipFact(),
    requiredAccountId,
    nowMs,
  );
  if (ownershipError) return { ok: false, error: ownershipError };

  const consent = recipient.marketingConsentFact();
  const consentError = validateBoundFact(
    recipient.email,
    "marketing_consent",
    consent,
    requiredAccountId,
  );
  if (consentError) return { ok: false, error: consentError };

  return {
    ok: true,
    value: {
      action: "marketing_message",
      recipient: recipient.email.value,
      accountId: requiredAccountId,
      sentAtMs: nowMs,
      effect: "network_send",
    },
  };
}

export function sendRegulatedNotice<
  M extends boolean,
  F extends boolean,
>(
  recipient: EmailFacts<true, M, F, true>,
  requiredAccountId: AccountId,
  requiredJurisdiction: string,
  nowMs: number,
  _networkSend: NetworkSendCapability,
): Result<OrthogonalDeliveryReceipt, OrthogonalSendError> {
  const ownershipError = validateOwnership(
    recipient.email,
    recipient.ownershipFact(),
    requiredAccountId,
    nowMs,
  );
  if (ownershipError) return { ok: false, error: ownershipError };

  const jurisdiction = recipient.jurisdictionFact();
  const jurisdictionBound = validateBoundFact(
    recipient.email,
    "jurisdiction_allowed",
    jurisdiction,
    requiredAccountId,
  );
  if (jurisdictionBound) return { ok: false, error: jurisdictionBound };

  if (nowMs > jurisdiction.expiresAtMs) {
    return {
      ok: false,
      error: {
        kind: "jurisdiction_stale",
        expiredAtMs: jurisdiction.expiresAtMs,
        nowMs,
        message: "jurisdiction_allowed exists but is stale.",
      },
    };
  }

  if (jurisdiction.jurisdiction !== requiredJurisdiction) {
    return {
      ok: false,
      error: {
        kind: "jurisdiction_mismatch",
        required: requiredJurisdiction,
        actual: jurisdiction.jurisdiction,
        message: `jurisdiction_allowed is ${jurisdiction.jurisdiction}, not ${requiredJurisdiction}.`,
      },
    };
  }

  return {
    ok: true,
    value: {
      action: "regulated_notice",
      recipient: recipient.email.value,
      accountId: requiredAccountId,
      sentAtMs: nowMs,
      effect: "network_send",
    },
  };
}
