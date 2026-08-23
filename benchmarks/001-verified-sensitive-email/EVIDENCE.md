# Benchmark 001 — Execution Evidence

Status: **PARTIAL / evidence in progress**  
Date: 2026-08-23

This file records only work that has actually been executed or directly verified.

It must not be used to infer that unlisted implementations or tests have passed.

---

## TypeScript baseline

Files:

```text
typescript/benchmark.ts
typescript/benchmark.test.ts
typescript/tsconfig.json
```

### Local execution environment

Available tools in the working execution environment:

```text
Node.js v22.16.0
TypeScript compiler (`tsc`)
```

### First run — failed for a test-harness reason

The initial `benchmark.test.ts` placed deliberately invalid `@ts-expect-error` calls directly in executable top-level code.

Result:

- strict TypeScript compilation succeeded, confirming the expected compile-time errors;
- the generated JavaScript then executed the first deliberately invalid call;
- runtime crashed because a `RawEmail` was intentionally passed where `VerifiedEmail` was required.

This was a **benchmark test-harness defect**, not a semantic baseline failure.

The negative compile-time cases were then moved into an unreachable `if (false)` block. TypeScript still checks them and `@ts-expect-error` still fails compilation if the expected static rejection disappears, but the invalid operations are no longer executed.

### Corrected run

Command pattern:

```text
tsc -p tsconfig.json
node dist/benchmark.test.js
```

Result:

```text
TypeScript Benchmark 001 runtime checks passed.
```

### What this actually proves

For the current TypeScript baseline and public API:

- I1 raw email → send is statically rejected;
- I2 syntax-valid but ownership-unverified email → send is statically rejected;
- I3 a second syntax-valid email cannot be passed as verified without separately creating a `VerifiedEmail`;
- I4 `replaceDomain` returns `SyntaxValidEmail`, so the changed value cannot be sent as ownership-verified without re-verification;
- I5 stale verification is rejected by runtime validation;
- I6 wrong account/scope is rejected by runtime validation;
- I7 omission of the explicit `NetworkSendCapability` argument is statically rejected;
- the valid case succeeds;
- runtime failure variants identify stale verification and scope mismatch explicitly.

### Important limitation

This does **not** prove that TypeScript provides unforgeable security guarantees.

The benchmark measures safe ordinary API use under strict TypeScript, not hostile JavaScript interop, `any`, type assertions, reflection-like escape hatches, or malicious construction.

---

## Rust baseline

Files:

```text
rust/Cargo.toml
rust/src/lib.rs
```

The Rust baseline includes:

- private state constructors/fields;
- `RawEmail`, `SyntaxValidEmail`, and `VerifiedEmail` state separation;
- ownership verification bound to the exact email value;
- identity-relevant mutation returning only `SyntaxValidEmail`;
- explicit `NetworkSendCapability` argument;
- runtime freshness and scope checks;
- unit tests;
- `compile_fail` doctests for raw/unverified/missing-capability calls.

### Execution status

**NOT YET EXECUTED in the local working environment.**

Reason:

```text
rustc: unavailable
cargo: unavailable
```

A repository workflow has been added at:

```text
.github/workflows/benchmark-001.yml
```

so the Rust and TypeScript baselines can be executed on GitHub Actions.

Until a workflow result is inspected, do **not** report the Rust tests as passing.

---

## Aytham candidate

File:

```text
aytham/semantic-model.md
```

Status:

```text
PAPER SEMANTICS ONLY
```

No executable Aytham validator currently implements the benchmark-specific model.

Therefore no static/runtime Aytham guarantee in the benchmark may yet be reported as demonstrated.

---

## Evidence discipline

Current evidence state:

```text
TypeScript baseline    COMPILED + EXECUTED LOCALLY
Rust baseline          IMPLEMENTED, EXECUTION PENDING
Aytham candidate       PAPER MODEL ONLY
```

The benchmark comparison must keep these three maturity levels separate.
