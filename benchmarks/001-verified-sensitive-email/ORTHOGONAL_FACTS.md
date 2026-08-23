# Benchmark 001 Extension — Orthogonal Facts Stress Case

Status: **ACTIVE**  
Date: 2026-08-23

## Purpose

The original Benchmark 001 showed that a simple linear state flow can be modelled well in ordinary TypeScript and Rust:

```text
RawEmail -> SyntaxValidEmail -> VerifiedEmail
```

This extension tests a harder and more relevant Aytham hypothesis:

> Can several independent facts about the same semantic subject remain compositional without creating awkward combined states, while allowing one fact to be invalidated without disturbing the others?

The benchmark must not force conventional languages into one-class-per-combination typestate. TypeScript and Rust may use ordinary generic/marker techniques to represent independent state dimensions.

---

## 1. Facts

After email syntax validation, the subject may independently acquire:

```text
ownership_verified
marketing_consent
mfa_verified
jurisdiction_allowed
```

The benchmark uses the same email subject for all four facts so that state-combination pressure is explicit.

Each fact remains semantically distinct. Establishing one must not imply another.

### ownership_verified

Carries:

```text
subject email value
account_id
established_at
expires_at
evidence_id
```

### marketing_consent

Carries:

```text
subject email value
account_id
evidence_id
```

### mfa_verified

Carries:

```text
subject email value
account_id
established_at
expires_at
evidence_id
```

### jurisdiction_allowed

Carries:

```text
subject email value
account_id
jurisdiction
established_at
expires_at
evidence_id
```

---

## 2. Actions requiring different subsets

### SendSecurityAlert

Requires:

```text
ownership_verified
mfa_verified
network_send capability
```

It must not require marketing consent or jurisdiction approval.

### SendMarketingMessage

Requires:

```text
ownership_verified
marketing_consent
network_send capability
```

It must not require MFA or jurisdiction approval.

### SendRegulatedNotice

Requires:

```text
ownership_verified
jurisdiction_allowed(required_jurisdiction)
network_send capability
```

It must not require marketing consent or MFA.

---

## 3. Independent invalidation test

The extension includes:

```text
RevokeMarketingConsent
```

Given a subject that currently has all four facts:

```text
ownership_verified
marketing_consent
mfa_verified
jurisdiction_allowed
```

revocation must produce a state where:

```text
marketing_consent = absent/invalid
```

while the other three facts remain usable.

After revocation:

```text
SendSecurityAlert     -> still allowed if freshness/scope checks pass
SendRegulatedNotice   -> still allowed if freshness/scope/jurisdiction checks pass
SendMarketingMessage  -> rejected because marketing_consent is absent
```

This is the key independence test.

---

## 4. Mandatory extension cases

### O1 — Missing exactly one required fact

A subject with ownership + MFA but no marketing consent:

```text
SendSecurityAlert    -> allowed
SendMarketingMessage -> rejected
```

### O2 — Fact kinds are not interchangeable

`jurisdiction_allowed` must not satisfy `marketing_consent`; `marketing_consent` must not satisfy `mfa_verified`.

Expected: static rejection where the baseline encoding makes fact presence part of the type/state.

### O3 — Revoke one fact, preserve the others

After `RevokeMarketingConsent`, security and regulated actions remain eligible while marketing send is statically unavailable in the typed baselines.

### O4 — MFA can be stale independently

Ownership can still be fresh while MFA has expired.

Expected:

```text
SendSecurityAlert -> runtime rejection: mfa_stale
```

Marketing send must not fail merely because MFA is stale, because it does not require MFA.

### O5 — Jurisdiction mismatch is independent

A `jurisdiction_allowed(IN)` fact cannot satisfy an action requiring `EU`.

Expected runtime/domain rejection identifying the jurisdiction mismatch.

### O6 — Wrong subject remains invalid

Facts established for email A cannot authorize email B.

Expected: rejected or unrepresentable without an explicit unsafe bypass.

---

## 5. Conventional baseline integrity

### TypeScript

Use ordinary language features only. A strong baseline may use a generic fact-state value such as:

```text
EmailFacts<Ownership, Marketing, MFA, Jurisdiction>
```

where each dimension is present/absent at the type level.

This avoids a straw-man `2^N` family of named wrapper classes. The cost of extra type parameters and transition methods must be counted honestly.

### Rust

Use stable Rust marker/typestate generics such as:

```text
EmailFacts<O, M, F, J>
```

with marker types for present/absent facts.

Again, do not create one named struct for every combination merely to make Aytham look better.

### Aytham

Use the reduced kernel candidate only:

```text
subject identity
independent claims
requires / establishes / invalidates / preserves
scope / freshness / provenance when required
capability/effect requirement
```

Do not add path planning or general theorem proving.

---

## 6. Measurement questions

1. Does either conventional baseline require combinatorial named states?
2. How much generic/marker machinery is required as facts increase?
3. Can one fact be removed while preserving the exact static knowledge of the others?
4. Are subject/scope/freshness checks reusable or handwritten per action?
5. Can diagnostics identify the one missing/stale/mismatched fact?
6. Does Aytham's independent claim model reduce ceremony materially, or merely replace type parameters with graph metadata?
7. Does Aytham provide a shared validation/explanation rule that conventional code lacks without itself becoming a generic rule engine?

---

## 7. Pass/fail consequence for Aytham

Aytham does **not** pass merely because its paper notation lists four independent claims.

The extension strengthens Aytham only if the eventual checker can show at least one meaningful advantage such as:

- less state-encoding ceremony as independent facts grow;
- generic requirement matching across actions;
- generic preservation/invalidation handling;
- better domain explanations generated from shared declarations rather than handwritten branches;
- provenance/scope/freshness integration without forcing them into every simple fact.

If TypeScript/Rust marker-state encodings remain comparably clear and the Aytham model requires similar or greater machinery, the kernel must be narrowed again.