# Benchmark 001 — Verified Sensitive Email Workflow

Status: **ACTIVE comparative benchmark / orthogonal-facts gate completed provisionally**  
Date started: 2026-08-23

This is the first evidence gate after the Aytham direction reset.

It compares the same semantic problem in:

1. TypeScript;
2. Rust;
3. the Aytham Semantic Kernel Candidate.

The benchmark exists to **falsify weak Aytham claims**, not to make Aytham look different.

---

## 1. Base scenario

An application receives an email string and may send a sensitive message only after:

1. syntax validation;
2. ownership verification;
3. correct verification account/scope;
4. fresh verification;
5. subject/value lineage has not invalidated the verification;
6. a `network_send` capability is available.

Required operations:

```text
RawEmail
   ↓ ParseEmail
SyntaxValidEmail
   ↓ VerifyOwnership
ownership-verified email
   ↓ SendSensitiveMessage
DeliveryReceipt
```

`ReplaceDomain` is intentionally identity-relevant and must not silently retain ownership verification.

---

## 2. Base invalid cases

Every implementation must account for:

```text
I1 raw email sent directly
I2 syntax-valid but unverified email sent
I3 verification from email A used for email B
I4 verified email changed afterward
I5 stale verification
I6 wrong account/scope
I7 missing network_send capability
```

Compile-time rejection and runtime rejection are recorded separately.

---

## 3. Base benchmark result so far

The executed TypeScript baseline demonstrated that ordinary strict TypeScript can already make several important invalid states unavailable through normal typed API use:

```text
RawEmail
SyntaxValidEmail
VerifiedEmail
```

and can deliberately provide domain-specific runtime diagnostics for freshness/scope.

Therefore Aytham may **not** claim that conventional languages can only provide generic type errors for this workflow.

See:

```text
EVIDENCE.md
COMPARISON.md
```

Rust has an implementation with unit/compile-fail tests, but local Rust execution remains unavailable; repository CI is configured to execute it.

The Aytham base model remains paper semantics only.

---

## 4. Orthogonal-facts extension

The first comparison revealed a sharper question: what happens when several facts are independent rather than one linear typestate?

The extension is specified in:

```text
ORTHOGONAL_FACTS.md
```

Four facts are tracked for one email subject:

```text
ownership_verified
marketing_consent
mfa_verified
jurisdiction_allowed
```

Three actions require different subsets:

```text
SendSecurityAlert
    ownership_verified + mfa_verified

SendMarketingMessage
    ownership_verified + marketing_consent

SendRegulatedNotice
    ownership_verified + jurisdiction_allowed
```

The extension also tests:

```text
RevokeMarketingConsent
```

which must remove only marketing consent while preserving the other independent facts.

---

## 5. Strong conventional baseline rule

The benchmark does not force TypeScript/Rust into one named wrapper for every possible state combination.

The conventional extensions deliberately use strong ordinary generic/marker models:

```text
TypeScript
EmailFacts<O, M, F, J>

Rust
EmailFacts<O, M, F, J>
```

This avoids a straw-man `2^N` family of named types.

---

## 6. Orthogonal extension result so far

### TypeScript — executed

Strict compilation and runtime execution succeeded.

The implementation demonstrated:

- different actions can require different static fact subsets;
- fact kinds are not interchangeable;
- revoking marketing consent preserves ownership/MFA/jurisdiction dimensions;
- stale MFA blocks the security action but not the marketing action;
- jurisdiction mismatch is diagnosed independently;
- a second email starts with no copied fact state.

Most importantly:

> **Conventional typestate does not inherently require combinatorial named wrapper classes.**

The generic-state TypeScript baseline falsifies that weak argument.

### Rust — implementation complete, execution pending

A corresponding marker-generic crate exists under:

```text
rust-orthogonal/
```

with unit tests and `compile_fail` doctests.

Do not report it as passing until CI/local Rust execution is directly inspected.

### Aytham — paper model only

The Aytham extension is documented in:

```text
aytham/orthogonal-model.md
```

It represents the facts as an open claim environment rather than a closed Boolean/marker tuple.

No executable Aytham guarantee is demonstrated yet.

See:

```text
ORTHOGONAL_EVIDENCE.md
ORTHOGONAL_COMPARISON.md
```

---

## 7. What the benchmark has falsified

Do not justify Aytham with:

> Conventional languages require one wrapper type for every combination of independent facts.

The benchmark shows that generic/marker state dimensions avoid this.

Do not justify Aytham merely with:

> A value can carry validated facts.

Refinements, typestate, proof/evidence values and ordinary application structures already do this.

---

## 8. Surviving Aytham hypothesis

The comparison has narrowed the research target to two properties.

### Open claim environment

Can new semantic claims be introduced without editing a central state tuple and without threading unrelated generic parameters through existing APIs?

Conceptually:

```text
subject
  + claim A
  + claim B
  + claim C
  + ...
```

with actions declaring only the claims they require.

### Shared semantic matcher and explanation

Can one bounded semantic checker derive from declarations:

```text
missing claim
wrong subject
wrong scope
stale claim
wrong claim value
invalidated claim history
missing capability
```

without each action implementing equivalent custom validation/diagnostic logic?

These are hypotheses, not demonstrated wins.

---

## 9. New comparison obligation

The open-claim idea now needs a stronger conventional comparison than the Boolean/marker tuple alone.

Before implementing the Aytham validator, compare against:

```text
independent proof-token APIs
row-polymorphic / extensible-record approaches
effect-row/open-set techniques where relevant
```

A conventional API can pass only the proofs an action requires, avoiding unrelated generic dimensions entirely.

Aytham must survive that comparison before its open claim environment is treated as a language contribution.

---

## 10. Benchmark integrity rules

1. Conventional baselines must be strong, ordinary designs—not straw men.
2. Aytham receives no extra problem information.
3. Static and runtime guarantees are reported separately.
4. Handwritten TypeScript/Rust diagnostics count as real baseline capability.
5. Aytham pays for metadata, matching rules, lineage and provenance machinery.
6. More semantic information in an IR is not itself a win.
7. No test is reported as passing until actually executed.
8. A falsified Aytham argument must be removed rather than defended.
9. Path planning remains outside this benchmark.

---

## 11. Current artifact map

```text
benchmarks/001-verified-sensitive-email/
  README.md
  EVIDENCE.md
  COMPARISON.md

  ORTHOGONAL_FACTS.md
  ORTHOGONAL_EVIDENCE.md
  ORTHOGONAL_COMPARISON.md

  typescript/
    benchmark.ts
    benchmark.test.ts
    orthogonal.ts
    orthogonal.test.ts
    tsconfig.json

  rust/
    Cargo.toml
    src/lib.rs

  rust-orthogonal/
    Cargo.toml
    src/lib.rs

  aytham/
    semantic-model.md
    orthogonal-model.md
```

CI:

```text
.github/workflows/benchmark-001.yml
```

---

## 12. Current evidence state

```text
TypeScript base               COMPILED + EXECUTED
TypeScript orthogonal         COMPILED + EXECUTED
Rust base                     IMPLEMENTED / EXECUTION PENDING
Rust orthogonal               IMPLEMENTED / EXECUTION PENDING
Aytham base                   PAPER MODEL
Aytham orthogonal             PAPER MODEL
```

---

## 13. Next activity

Do **not** implement the Aytham validator yet.

The next evidence gate is:

> **Open Claim Environment Comparison**

Compare the surviving Aytham claim-environment hypothesis against:

1. an ordinary independent proof-token design;
2. row-polymorphic/extensible-record prior art and representative modelling;
3. effect-row/open-set techniques where they overlap.

Focus on:

```text
adding new claim kinds
requiring only local subsets
subject identity
scope/freshness
independent invalidation
provenance/history
generated diagnostics
```

Only after that comparison should the project decide whether a minimal executable Aytham requirement matcher has earned implementation.