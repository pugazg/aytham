import {
  NetworkSendCapability,
  RawEmail,
  SyntaxValidEmail,
} from "./benchmark";
import {
  EmailFacts,
  sendMarketingMessage,
  sendRegulatedNotice,
  sendSecurityAlert,
} from "./orthogonal";

function assert(condition: unknown, message: string): asserts condition {
  if (!condition) throw new Error(message);
}

const T0 = 2_000_000;
const OWNERSHIP_TTL = 120_000;
const MFA_TTL = 30_000;
const JURISDICTION_TTL = 120_000;
const ACCOUNT_42 = "user-42";
const network = NetworkSendCapability.issue("orthogonal-benchmark");

const parsed = SyntaxValidEmail.parse(new RawEmail("person@example.org"));
assert(parsed.ok, "orthogonal benchmark email should parse");

// O1 — ownership + MFA is enough for security, but not marketing.
const ownershipAndMfa = EmailFacts.from(parsed.value)
  .verifyOwnership(
    ACCOUNT_42,
    T0,
    OWNERSHIP_TTL,
    "ownership-challenge-001",
  )
  .verifyMfa(ACCOUNT_42, T0, MFA_TTL, "mfa-challenge-001");

const security = sendSecurityAlert(
  ownershipAndMfa,
  ACCOUNT_42,
  T0 + 1_000,
  network,
);
assert(security.ok, "ownership + MFA should allow security alert");

if (false) {
  // @ts-expect-error O1: marketing_consent is absent.
  sendMarketingMessage(ownershipAndMfa, ACCOUNT_42, T0 + 1_000, network);
}

// Build all four independent facts without introducing a named combined state.
const allFacts = ownershipAndMfa
  .grantMarketingConsent(ACCOUNT_42, T0, "consent-001")
  .allowJurisdiction(
    ACCOUNT_42,
    "IN",
    T0,
    JURISDICTION_TTL,
    "jurisdiction-policy-001",
  );

const marketing = sendMarketingMessage(
  allFacts,
  ACCOUNT_42,
  T0 + 1_000,
  network,
);
assert(marketing.ok, "all-facts state should allow marketing send");

const regulated = sendRegulatedNotice(
  allFacts,
  ACCOUNT_42,
  "IN",
  T0 + 1_000,
  network,
);
assert(regulated.ok, "all-facts state should allow regulated notice");

// O2 — fact kinds are not interchangeable. A state with jurisdiction but no
// marketing consent still cannot satisfy marketing send.
const ownershipAndJurisdiction = EmailFacts.from(parsed.value)
  .verifyOwnership(
    ACCOUNT_42,
    T0,
    OWNERSHIP_TTL,
    "ownership-challenge-002",
  )
  .allowJurisdiction(
    ACCOUNT_42,
    "IN",
    T0,
    JURISDICTION_TTL,
    "jurisdiction-policy-002",
  );

if (false) {
  // @ts-expect-error O2: jurisdiction_allowed is not marketing_consent.
  sendMarketingMessage(
    ownershipAndJurisdiction,
    ACCOUNT_42,
    T0 + 1_000,
    network,
  );

  // @ts-expect-error O2: marketing/jurisdiction state does not imply MFA.
  sendSecurityAlert(
    ownershipAndJurisdiction,
    ACCOUNT_42,
    T0 + 1_000,
    network,
  );
}

// O3 — revoke exactly one fact and preserve the exact static knowledge of the
// other dimensions.
const marketingRevoked = allFacts.revokeMarketingConsent();

const securityAfterRevocation = sendSecurityAlert(
  marketingRevoked,
  ACCOUNT_42,
  T0 + 1_000,
  network,
);
assert(
  securityAfterRevocation.ok,
  "revoking marketing consent must preserve ownership + MFA",
);

const regulatedAfterRevocation = sendRegulatedNotice(
  marketingRevoked,
  ACCOUNT_42,
  "IN",
  T0 + 1_000,
  network,
);
assert(
  regulatedAfterRevocation.ok,
  "revoking marketing consent must preserve jurisdiction",
);

if (false) {
  // @ts-expect-error O3: marketing_consent was removed by the transition.
  sendMarketingMessage(
    marketingRevoked,
    ACCOUNT_42,
    T0 + 1_000,
    network,
  );
}

// O4 — MFA freshness is independent. Marketing does not require MFA.
const shortMfa = EmailFacts.from(parsed.value)
  .verifyOwnership(
    ACCOUNT_42,
    T0,
    OWNERSHIP_TTL,
    "ownership-challenge-003",
  )
  .verifyMfa(ACCOUNT_42, T0, 500, "mfa-short")
  .grantMarketingConsent(ACCOUNT_42, T0, "consent-002");

const staleMfaSecurity = sendSecurityAlert(
  shortMfa,
  ACCOUNT_42,
  T0 + 1_000,
  network,
);
assert(!staleMfaSecurity.ok, "stale MFA should block security alert");
assert(
  staleMfaSecurity.error.kind === "mfa_stale",
  "security failure should identify only mfa_stale",
);

const marketingWithStaleMfa = sendMarketingMessage(
  shortMfa,
  ACCOUNT_42,
  T0 + 1_000,
  network,
);
assert(
  marketingWithStaleMfa.ok,
  "stale MFA must not block an action that does not require MFA",
);

// O5 — jurisdiction mismatch is independent and domain-specific.
const wrongJurisdiction = sendRegulatedNotice(
  allFacts,
  ACCOUNT_42,
  "EU",
  T0 + 1_000,
  network,
);
assert(!wrongJurisdiction.ok, "wrong jurisdiction should fail");
assert(
  wrongJurisdiction.error.kind === "jurisdiction_mismatch",
  "regulated notice should identify jurisdiction mismatch",
);

// O6 — another email starts with no facts. There is no public API for copying
// the hidden fact payload from `allFacts` into this new value.
const parsedB = SyntaxValidEmail.parse(new RawEmail("other@example.org"));
assert(parsedB.ok, "second email should parse");
const otherEmail = EmailFacts.from(parsedB.value);

if (false) {
  // @ts-expect-error O6: facts for email A do not make email B verified.
  sendSecurityAlert(otherEmail, ACCOUNT_42, T0 + 1_000, network);
}

console.log("TypeScript Benchmark 001 orthogonal-facts checks passed.");
