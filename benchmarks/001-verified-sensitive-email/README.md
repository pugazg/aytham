# Benchmark 001 — Verified Sensitive Email Workflow

Status: **ACTIVE comparative benchmark**  
Date started: 2026-08-23

This benchmark is the first evidence gate after the Aytham direction reset.

It compares the same programming problem in:

1. TypeScript;
2. Rust;
3. Aytham Semantic Kernel Candidate.

The purpose is **not** to make Aytham look different. The purpose is to determine whether its semantic contract model earns its complexity.

---

# 1. Scenario

An application receives an email string and may send a sensitive message only after:

1. the email syntax has been validated;
2. ownership has been verified;
3. the ownership verification applies to the correct account/scope;
4. the verification is still fresh;
5. the exact email value being sent is the value that was verified, or a transformation has explicitly preserved the verification;
6. the caller possesses permission/capability to perform the network-send effect.

The benchmark must make invalid cases difficult or impossible to perform accidentally.

---

# 2. Required semantic states

## Raw input

```text
RawEmail
```

No email-specific semantic claim is established.

## Syntax-valid email

```text
email_syntax_valid
```

This means only that the string satisfies the benchmark's email syntax rule.

It does **not** imply ownership.

## Ownership verification

```text
ownership_verified
```

The verification must carry at least:

```text
subject email value
account/scope
established_at
expires_at
evidence identifier
```

## Network-send capability

Sensitive sending requires an explicit permission/capability representing:

```text
network_send
```

The capability model does not need to be cryptographically secure for this benchmark. It exists to compare whether the effect requirement is visible and hard to bypass accidentally.

---

# 3. Required operations

## ParseEmail

Input:

```text
RawEmail
```

Success establishes:

```text
email_syntax_valid
```

Failure returns a syntax error.

## VerifyOwnership

Input requires:

```text
email_syntax_valid
```

Inputs include:

```text
account_id
verification time
evidence id
```

Success establishes ownership verification scoped to that account and exact email value.

## ReplaceDomain

Changes:

```text
person@example.org
```

into another syntactically valid address.

This transformation is intentionally identity-relevant and therefore must **not silently preserve** ownership verification.

Its output should require re-verification.

## NormalizeDisplay

An optional benchmark extension may perform a transformation that is defined to preserve semantic email identity.

If included, the implementation must make the preservation rule explicit enough to compare with Aytham's future `preserves` concept.

## SendSensitiveMessage

Requires:

```text
email_syntax_valid
ownership_verified
verification scope == target account
verification still fresh
network_send capability
```

Produces:

```text
DeliveryReceipt
```

---

# 4. Mandatory invalid cases

Every implementation must show how it handles these seven cases.

## I1 — Raw text sent directly

```text
RawEmail -> SendSensitiveMessage
```

Expected: rejected before sensitive send.

## I2 — Syntax-valid but ownership-unverified

```text
ParseEmail -> SendSensitiveMessage
```

Expected: rejected because ownership has not been established.

## I3 — Verification belongs to another email value

```text
A verified
B unverified
send B using A's verification
```

Expected: rejected or unrepresentable without an explicit unsafe bypass.

## I4 — Verified value mutated afterward

```text
verify person@example.org
replace domain -> person@attacker.example
send
```

Expected: ownership verification must not silently survive.

## I5 — Stale verification

```text
now > expires_at
```

Expected: rejected with a freshness/expiry reason.

## I6 — Wrong account/scope

```text
verification scope = user-42
send requires       user-77
```

Expected: rejected with a scope mismatch reason.

## I7 — Missing network-send capability

The address is valid and verified, but the calling context has no network-send permission.

Expected: rejected before effect execution.

---

# 5. Valid case

```text
raw input
  ↓ ParseEmail
syntax-valid email
  ↓ VerifyOwnership(account=user-42)
verified email for user-42
  ↓ SendSensitiveMessage(
        account=user-42,
        now <= expires_at,
        network_send capability)
DeliveryReceipt
```

---

# 6. Baseline constraints

## TypeScript

Use ordinary TypeScript mechanisms only.

Permitted techniques include:

- branded types;
- discriminated unions;
- classes/interfaces;
- opaque construction through module/private fields where practical;
- explicit runtime checks;
- capability tokens.

Do not invent an Aytham-like graph framework inside the TypeScript baseline merely to make comparison symmetrical.

## Rust

Use ordinary stable Rust mechanisms only.

Permitted techniques include:

- newtypes;
- private fields/constructors;
- ownership/moves;
- enums/Result;
- typestate-like structs;
- capability tokens.

Do not add a theorem prover, macro framework, or external verification library.

## Aytham candidate

Only after the two conventional baselines exist, model the same requirements using the current candidate:

```text
Subject / Value Identity
Relation / Role
Claim
Action / Transformation
Composition Judgment
```

with optional provenance/scope/freshness/effects.

Do not change the benchmark requirements to fit Aytham.

---

# 7. Measurement rubric

For each implementation record:

## Safety

- Which invalid cases are compile-time impossible?
- Which require runtime checks?
- Which can be bypassed accidentally through ordinary API use?

## Ceremony

Count/describe:

- wrapper/newtype declarations;
- state-specific types;
- annotations;
- explicit metadata required at normal call sites;
- helper functions required to maintain invariants.

Raw line count may be reported but is not the primary metric.

## Independent facts

Assess how naturally the implementation scales if additional orthogonal claims are added, such as:

```text
marketing_consent
mfa_verified
age_verified
jurisdiction_allowed
```

Does the design require combinatorial wrapper/state types?

## Lineage / invalidation

How clearly does the implementation prevent an established fact from migrating to a changed value?

## Provenance / scope / freshness

Can the program explain:

```text
who/what established this?
for which value?
for which account?
when?
until when?
```

without turning every simple property into heavyweight metadata?

## Effects

How visible is the `network_send` requirement at the API boundary?

## Diagnostics

Evaluate failures for:

- missing ownership verification;
- stale verification;
- wrong scope;
- verification invalidated by changed value;
- missing effect capability.

Distinguish compiler diagnostics from deliberately designed application/runtime diagnostics.

## Progressive disclosure

Does ordinary syntax validation remain simple, or does the model force full provenance/effect machinery everywhere?

---

# 8. Benchmark integrity rules

1. The TypeScript and Rust versions are real baselines, not straw men.
2. Aytham does not receive extra semantic information that the baseline problem statement did not provide.
3. Compile-time impossibility and runtime validation must be reported separately.
4. A runtime error message written manually in TypeScript/Rust counts as a deliberate diagnostic capability and must not be dismissed.
5. Aytham must pay for its metadata and concepts in the ceremony evaluation.
6. Aytham does not pass merely because its IR contains more information.
7. If conventional code is clearer overall, the Aytham mechanism must be revised.
8. No implementation is reported as tested until it is actually executed.

---

# 9. Planned artifacts

```text
benchmarks/001-verified-sensitive-email/
  README.md

  typescript/
    benchmark.ts
    tsconfig.json

  rust/
    Cargo.toml
    src/lib.rs

  aytham/
    semantic-model.md
    graph.json          # only if the candidate actually needs it

  COMPARISON.md         # written after all three models exist
```

The file plan is provisional except for this benchmark specification.

---

# 10. Current activity

Implement the TypeScript and Rust baselines without changing these benchmark requirements.

Only then model the Aytham candidate and perform the comparison.
