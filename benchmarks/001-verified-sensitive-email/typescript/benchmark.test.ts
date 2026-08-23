import {
  NetworkSendCapability,
  RawEmail,
  SyntaxValidEmail,
  VerifiedEmail,
  sendSensitiveMessage,
} from "./benchmark";

function assert(condition: unknown, message: string): asserts condition {
  if (!condition) {
    throw new Error(message);
  }
}

const T0 = 1_000_000;
const TTL = 60_000;
const ACCOUNT_42 = "user-42";
const ACCOUNT_77 = "user-77";

const parsed = SyntaxValidEmail.parse(new RawEmail("person@example.org"));
assert(parsed.ok, "baseline email should parse");

const verified = VerifiedEmail.verifyOwnership(
  parsed.value,
  ACCOUNT_42,
  T0,
  TTL,
  "challenge-001",
);

const network = NetworkSendCapability.issue("benchmark-test");

// Valid case.
const valid = sendSensitiveMessage(
  verified,
  ACCOUNT_42,
  T0 + 1_000,
  network,
);
assert(valid.ok, "valid verified send should succeed");
assert(valid.value.effect === "network_send", "effect should be explicit");

// I1 — Raw text cannot be passed directly.
const raw = new RawEmail("person@example.org");
// @ts-expect-error RawEmail is not VerifiedEmail.
sendSensitiveMessage(raw, ACCOUNT_42, T0, network);

// I2 — Syntax-valid but ownership-unverified cannot be passed directly.
// @ts-expect-error SyntaxValidEmail is not VerifiedEmail.
sendSensitiveMessage(parsed.value, ACCOUNT_42, T0, network);

// I3 — Verification for A cannot be detached and applied to B through the
// public API. A second value remains syntax-valid only until separately
// verified.
const parsedB = SyntaxValidEmail.parse(new RawEmail("other@example.org"));
assert(parsedB.ok, "second email should parse");
// @ts-expect-error SyntaxValidEmail B is not ownership-verified.
sendSensitiveMessage(parsedB.value, ACCOUNT_42, T0, network);

// I4 — Identity-relevant mutation deliberately downgrades to SyntaxValidEmail.
const changed = verified.email.replaceDomain("attacker.example");
assert(changed.ok, "changed email should remain syntactically valid");
// @ts-expect-error changed value has lost ownership-verification state.
sendSensitiveMessage(changed.value, ACCOUNT_42, T0, network);

// I5 — Stale verification is rejected at runtime because freshness depends on
// the current clock value.
const stale = sendSensitiveMessage(
  verified,
  ACCOUNT_42,
  T0 + TTL + 1,
  network,
);
assert(!stale.ok, "stale verification should fail");
assert(
  stale.error.kind === "verification_stale",
  "stale failure should explain freshness",
);

// I6 — Wrong account/scope is rejected at runtime.
const wrongScope = sendSensitiveMessage(
  verified,
  ACCOUNT_77,
  T0 + 1_000,
  network,
);
assert(!wrongScope.ok, "wrong-scope verification should fail");
assert(
  wrongScope.error.kind === "scope_mismatch",
  "scope failure should identify scope mismatch",
);

// I7 — The network capability is a required API argument.
// @ts-expect-error Missing NetworkSendCapability argument.
sendSensitiveMessage(verified, ACCOUNT_42, T0 + 1_000);

console.log("TypeScript Benchmark 001 runtime checks passed.");
