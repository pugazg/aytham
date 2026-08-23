# Benchmark 001 — Comparative Review

Status: **PROVISIONAL — Rust execution and Aytham implementation still pending**  
Date: 2026-08-23

This review compares the current artifacts without claiming more evidence than exists.

Execution status is recorded separately in `EVIDENCE.md`.

---

# 1. Current artifacts

## TypeScript

Real implementation:

```text
typescript/benchmark.ts
typescript/benchmark.test.ts
```

Execution status:

```text
COMPILED + EXECUTED LOCALLY
```

## Rust

Real implementation:

```text
rust/src/lib.rs
```

Execution status:

```text
IMPLEMENTED / EXECUTION PENDING
```

## Aytham

Current artifact:

```text
aytham/semantic-model.md
```

Execution status:

```text
PAPER SEMANTICS ONLY
```

No Aytham victory can be declared from this benchmark yet.

---

# 2. Important result already visible from the TypeScript baseline

The benchmark immediately disproves any weak claim that Aytham is needed simply to make the following impossible:

```text
raw email -> sensitive send
syntax-valid but unverified email -> sensitive send
changed email retaining verification
missing explicit network-send token
```

Strict TypeScript with private/nominal state wrappers can already make these ordinary API misuses compile-time errors.

It can also provide deliberate runtime diagnostics for:

```text
stale verification
wrong account scope
```

Therefore Aytham cannot justify itself through the statement:

> Conventional languages can only report generic type errors and cannot model these guarantees.

That statement would be false for this benchmark.

---

# 3. TypeScript baseline strengths

## Compile-time state separation

The baseline uses:

```text
RawEmail
SyntaxValidEmail
VerifiedEmail
```

with private nominal markers/constructors.

This makes several invalid transitions unavailable through normal typed API use.

## Explicit invalidation through return type

`replaceDomain` returns:

```text
SyntaxValidEmail
```

rather than:

```text
VerifiedEmail
```

so ownership verification is dropped naturally through the type transition.

This is already a clear and effective solution to value-lineage invalidation for this simple linear state flow.

## Good custom runtime explanations

The baseline can deliberately explain:

```text
verification_stale
scope_mismatch
```

with domain-specific data.

Aytham must compare against these designed diagnostics, not against intentionally poor messages.

## Explicit effect argument

`NetworkSendCapability` makes the network-send requirement visible at the function boundary.

Again, Aytham cannot claim that effect/capability visibility is unavailable in a conventional language.

---

# 4. TypeScript baseline weaknesses relevant to Aytham

## Wrapper/state proliferation risk

For the current linear workflow, three wrappers are manageable:

```text
RawEmail
SyntaxValidEmail
VerifiedEmail
```

The harder question appears when independent facts multiply.

Suppose email values may independently carry:

```text
syntax_valid
ownership_verified
marketing_consent
mfa_verified
age_verified
jurisdiction_allowed
```

A pure wrapper-state approach must choose among several strategies:

1. create many combined wrapper states;
2. encode properties in generic type parameters/brands;
3. use intersection types;
4. move some facts back to runtime metadata;
5. build a more elaborate type-state framework.

This is the first place where Aytham's **independent claim set** may offer a genuine ergonomics advantage.

That advantage is not proven by the current benchmark because Benchmark 001 currently uses only a few facts.

## Provenance is manually embedded in the wrapper

TypeScript can store:

```text
subjectValue
accountId
establishedAtMs
expiresAtMs
evidenceId
```

but the language does not automatically give this metadata semantic meaning.

The application function must manually interpret scope/freshness/subject requirements.

Aytham's possible advantage is not storing the data; TypeScript stores it easily.

The possible advantage is **making claim semantics and requirement matching reusable language/tooling behaviour rather than handwritten checks in each API**.

That remains unproven.

## Compiler diagnostic versus domain diagnostic split

For I1/I2/I4, TypeScript's compiler rejects the call by nominal state type.

The error is primarily structural/type-oriented.

For I5/I6, application code produces domain messages manually.

Aytham's hypothesis is that one semantic model could explain both kinds of failure using the same domain vocabulary.

Whether that is worth the semantic machinery remains an open benchmark question.

---

# 5. Rust baseline — expected comparison, pending execution

The Rust implementation uses a similarly strong conventional model:

```text
RawEmail
SyntaxValidEmail
VerifiedEmail
```

with private construction and explicit state transitions.

Rust additionally has language-level ownership/move semantics that may make value/state lineage especially natural.

Expected strengths, subject to actual execution:

- strong construction boundaries;
- clear state transitions through distinct types;
- safe public API preventing evidence detachment;
- compile-fail tests for invalid call shapes;
- explicit capability token;
- runtime freshness/scope checks.

Potential Aytham comparison pressure:

Rust may make the `changed value loses verified state` rule very clear without any separate semantic graph.

If so, Aytham must demonstrate value elsewhere—probably independent fact composition and explanation—not merely lineage safety.

Do not treat these expectations as test results until `cargo test` has executed successfully.

---

# 6. Aytham candidate strengths — conceptual only

## Independent claims rather than monolithic state wrappers

Aytham can conceptually represent:

```text
email#v1
  + syntax_valid
  + ownership_verified(scope=user-42, expires=T1)
  + marketing_consent
  + jurisdiction_allowed
```

without requiring a distinct wrapper type for every combination.

This may be Aytham's strongest practical hypothesis.

But to count as an advantage, the future checker must:

- remain sound;
- remain understandable;
- avoid theorem-prover-level annotation;
- handle path-sensitive runtime-established claims;
- make invalidation/preservation predictable;
- keep ordinary code compact.

## Unified explanation model

Aytham can conceptually explain:

```text
missing ownership claim
wrong claim subject
wrong scope
stale validity
claim invalidated by transformation
missing network capability
```

through one semantic requirement system.

The TypeScript baseline currently splits these across:

- compile-time state-type rejection;
- explicit handwritten runtime checks.

Aytham's value would be substantial only if the language/runtime can generate these explanations from declarations rather than requiring equally manual custom code.

## Explicit preservation/invalidation vocabulary

The Aytham candidate says:

```text
ReplaceDomain
  preserves   syntax_valid
  invalidates ownership_verified
```

This can be easier to inspect than discovering the same rule indirectly from return types.

However, explicit declarations also create annotation and soundness obligations.

Aytham must prove that `preserves/invalidates` does not become repetitive bookkeeping.

---

# 7. Aytham candidate weaknesses exposed by the benchmark

## No executable semantics yet

The current Aytham model does not prove that any invalid case is actually rejected.

Every Aytham advantage in this document is therefore still a hypothesis.

## Runtime-established facts need a real path-sensitive model

Ownership verification happens at runtime and only on success.

A future Aytham checker must represent:

```text
before success:
    ownership_verified absent

after successful branch:
    ownership_verified established

failure branch:
    ownership_verified absent
```

This is not solved merely by having a Claim record.

The project must eventually define flow/path sensitivity or an equivalent state-transition semantics.

## `preserves` may hide proof obligations

If code declares:

```text
preserves ownership_verified
```

how is that trusted?

Possible policies include:

- compiler proves it;
- operation is defined in terms of identity-preserving primitives;
- trusted/unsafe assertion;
- runtime re-check;
- external proof/attestation.

Until this is defined, `preserves` is descriptive rather than a guarantee.

## General claim matching may recreate a refinement/proof system

Scope/freshness requirements such as:

```text
scope == account(user-42)
now <= expires_at
subject == recipient
```

need actual comparison/evaluation rules.

If Aytham allows arbitrary predicates, it rapidly approaches refinement/dependent verification complexity.

The language needs a deliberately bounded everyday claim model.

## Semantic IDs may be implementation overhead

The paper model uses:

```text
email#v0
email#v1
email#v2
```

A user should not need to manually manage these IDs.

If identity/lineage cannot be inferred naturally from ordinary program values, the semantic model will be too heavy.

---

# 8. Invalid-case comparison

Current evidence/expectation:

| Case | TypeScript | Rust | Aytham candidate |
|---|---|---|---|
| I1 raw send | static rejection demonstrated | designed as static rejection; execution pending | should reject; not implemented |
| I2 unverified send | static rejection demonstrated | designed as static rejection; execution pending | should reject missing claim; not implemented |
| I3 evidence for another value | public API prevents ordinary misuse | private construction designed to prevent misuse | explicit subject mismatch model; not implemented |
| I4 mutation after verification | return type drops verified state; demonstrated by compile-time check | changed value returns syntax-valid state | explicit invalidation model; not implemented |
| I5 stale verification | runtime rejection demonstrated | runtime rejection designed; pending | runtime validity requirement; not implemented |
| I6 wrong scope | runtime rejection demonstrated | runtime rejection designed; pending | scope mismatch model; not implemented |
| I7 missing network capability | static missing-argument rejection demonstrated | compile-fail test designed; pending | missing capability model; not implemented |

This table is deliberately asymmetric because implementation maturity is asymmetric.

---

# 9. First benchmark conclusion

Aytham has **not yet earned a PASS**.

The TypeScript baseline is stronger than the earlier Aytham research documents implicitly assumed conventional code would be.

For this small linear workflow, ordinary state wrappers plus runtime validation are already:

- safe under normal API use;
- understandable;
- compact enough;
- capable of good domain-specific runtime errors.

Therefore Aytham should **not** pursue a full semantic framework merely to solve this exact simple pipeline.

The benchmark does, however, identify a sharper question worth testing:

> **Does Aytham handle multiple independent, provenance-bearing, scope/freshness-sensitive claims more compositionally than wrapper/typestate designs while generating useful diagnostics from shared semantic declarations?**

That is a substantially stronger and more falsifiable research target.

---

# 10. Recommended benchmark extension before kernel freeze

Add a bounded **orthogonal-facts stress case** to Benchmark 001.

For the same email subject, introduce independent facts:

```text
ownership_verified
marketing_consent
mfa_verified
jurisdiction_allowed
```

Then define actions that require different subsets:

```text
SendSecurityAlert
    requires ownership_verified + mfa_verified

SendMarketingMessage
    requires ownership_verified + marketing_consent

SendRegulatedNotice
    requires ownership_verified + jurisdiction_allowed
```

Questions:

1. How many wrapper/state types or type parameters are needed in TypeScript?
2. How naturally does Rust model the combinations?
3. Can Aytham keep the facts independent without losing static safety?
4. Can a transformation invalidate one claim while preserving the others?
5. Can diagnostics identify exactly one missing/invalidated fact?

This extension targets the place where the Aytham claim-set model could genuinely outperform monolithic typestate.

---

# 11. Decision

## Do not freeze the Semantic Kernel yet.

Retain:

```text
Relation / Role
Claim
Action / Transformation
requires
establishes
preserves
invalidates
effects/capabilities
```

for further testing.

But narrow the immediate research target from:

> build a unified semantic graph

into:

> test whether independent semantic claims + lineage-aware preservation/invalidation + shared diagnostics provide practical advantages over conventional typed state encodings.

The canonical graph remains an internal representation candidate, not a demonstrated user-facing language contribution.

---

# 12. Next activity

1. Inspect GitHub Actions results for both conventional baselines when available.
2. Fix any real baseline defects before comparing Aytham.
3. Extend Benchmark 001 with the orthogonal-facts stress case.
4. Only then decide whether to implement a minimal Aytham benchmark validator.

No path-planning work should resume.
