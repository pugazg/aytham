# Benchmark 001 Extension — Orthogonal Facts Execution Evidence

Status: **PARTIAL / TypeScript executed, Rust execution pending, Aytham paper-only**  
Date: 2026-08-23

This file records only evidence actually obtained for `ORTHOGONAL_FACTS.md`.

---

## TypeScript

Repository artifacts:

```text
typescript/orthogonal.ts
typescript/orthogonal.test.ts
typescript/tsconfig.json
```

The extension uses one generic state representation:

```text
EmailFacts<O, M, F, J>
```

where the four Boolean dimensions represent presence/absence of:

```text
O = ownership_verified
M = marketing_consent
F = mfa_verified
J = jurisdiction_allowed
```

### Local execution environment

```text
Node.js v22.16.0
TypeScript compiler 5.8.3
strict TypeScript settings
```

The repository benchmark code was tested together with the new orthogonal extension.

Command pattern:

```text
tsc -p tsconfig.json
node dist/benchmark.test.js
node dist/orthogonal.test.js
```

Observed output:

```text
TypeScript Benchmark 001 runtime checks passed.
TypeScript Benchmark 001 orthogonal-facts checks passed.
```

### Static checks demonstrated by `@ts-expect-error`

Strict compilation succeeded while verifying that the following calls remain compile-time errors:

```text
O1 ownership + MFA -> SendMarketingMessage
   rejected because marketing_consent is absent

O2 ownership + jurisdiction -> SendMarketingMessage
   rejected because jurisdiction_allowed is not marketing_consent

O2 ownership + jurisdiction -> SendSecurityAlert
   rejected because mfa_verified is absent

O3 state after revokeMarketingConsent -> SendMarketingMessage
   rejected because the marketing dimension changed from true to false

O6 a second parsed email with no facts -> SendSecurityAlert
   rejected because ownership/MFA dimensions are absent
```

If any of these expected type errors disappears, `tsc` will report an unused `@ts-expect-error` directive and fail the benchmark compile.

### Runtime checks demonstrated

The extension also executed these dynamic cases:

```text
ownership + MFA -> SendSecurityAlert
PASS

all four facts -> SendMarketingMessage
PASS

all four facts -> SendRegulatedNotice(IN)
PASS

revoke marketing consent -> SendSecurityAlert
PASS

revoke marketing consent -> SendRegulatedNotice(IN)
PASS

MFA expired while ownership remains fresh -> SendSecurityAlert
REJECT: mfa_stale

same stale-MFA state -> SendMarketingMessage
PASS because marketing send does not require MFA

jurisdiction_allowed(IN) -> SendRegulatedNotice(EU)
REJECT: jurisdiction_mismatch
```

### What this proves

For this four-fact benchmark, ordinary strict TypeScript can:

1. avoid `2^4` named wrapper classes;
2. encode four independent static fact-presence dimensions in one generic type;
3. require different fact subsets in different action signatures;
4. revoke only marketing consent while retaining static ownership/MFA/jurisdiction knowledge;
5. keep freshness/value checks runtime-dependent where appropriate;
6. keep an irrelevant stale fact from blocking an action that does not require it.

This falsifies the simple argument:

> Conventional typestate necessarily creates one named wrapper class for every fact combination.

It does not.

### TypeScript cost observed

The solution is not free.

Each new independent fact currently requires another generic state dimension in:

```text
EmailFacts<O, M, F, J>
```

Transitions must preserve unrelated dimensions explicitly:

```text
verifyOwnership -> EmailFacts<true, M, F, J>
revokeMarketing -> EmailFacts<O, false, F, J>
```

Actions that care about only two facts still quantify over unrelated dimensions so those dimensions remain unconstrained:

```text
SendSecurityAlert<M, J>(EmailFacts<true, M, true, J>)
```

This is a real maintenance/ergonomics pressure point, but it is not combinatorial named-state explosion.

---

## Rust

Repository artifacts:

```text
rust-orthogonal/Cargo.toml
rust-orthogonal/src/lib.rs
```

The Rust extension deliberately uses the analogous strong conventional design:

```text
EmailFacts<O, M, F, J>
```

with:

```text
Present
Absent
```

marker types.

It includes:

- independent transition methods;
- `revoke_marketing_consent` preserving O/F/J markers;
- action signatures requiring different marker subsets;
- runtime MFA freshness and jurisdiction-value checks;
- unit tests;
- `compile_fail` doctests for missing marketing consent and post-revocation marketing send.

### Execution status

```text
IMPLEMENTED / NOT LOCALLY EXECUTED
```

The current local execution environment has no `rustc` or `cargo`.

The repository workflow now runs both:

```text
rust/Cargo.toml
rust-orthogonal/Cargo.toml
```

on GitHub Actions.

Until an Actions result is directly inspected, the Rust extension must not be reported as passing.

---

## Aytham

Repository artifact:

```text
aytham/orthogonal-model.md
```

Status:

```text
PAPER SEMANTICS ONLY
```

No Aytham checker currently proves requirement presence, freshness, subject matching, independent invalidation, or generic diagnostics for this extension.

---

## Evidence state

```text
TypeScript orthogonal baseline  COMPILED + EXECUTED LOCALLY
Rust orthogonal baseline        IMPLEMENTED / EXECUTION PENDING
Aytham orthogonal model         PAPER MODEL ONLY
```

No Aytham PASS is justified yet.