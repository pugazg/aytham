# Open Claim Environment Comparison

Status: research gate in progress
Date: 2026-08-23

## Question

The orthogonal-facts benchmark removed the weak argument that conventional typestate requires a separate wrapper for every combination of facts.

The surviving Aytham hypothesis is:

> An open semantic claim environment may allow actions to request only the claims they need, while a shared matcher provides consistent validation and explanations.

This document compares that hypothesis against stronger conventional alternatives.

---

# 1. Independent proof-token APIs

## Model

A conventional design can pass only the evidence/action proofs required:

```text
sendSecurityAlert(email, OwnershipProof, MfaProof, NetworkCapability)

sendMarketing(email, OwnershipProof, ConsentProof, NetworkCapability)
```

## Strengths

- no central state tuple;
- unrelated facts do not appear in unrelated APIs;
- proof ownership can be explicit;
- works naturally in existing languages.

## Weaknesses

- every API defines its own proof vocabulary;
- cross-cutting diagnostics are usually application-specific;
- transformations need conventions for preservation/invalidation.

## Aytham obligation

Aytham must show that one semantic requirement system provides meaningful reuse beyond manually composing proof objects.

---

# 2. Extensible records / row-polymorphic approaches

## Model

A value can carry an open set of fields/facts:

```text
Email {
  syntax_valid,
  ownership_verified,
  mfa_verified,
  ...additional facts
}
```

## Strengths

- open extension is natural;
- functions can require only selected fields;
- avoids fixed Boolean dimensions.

## Weaknesses

- field presence does not automatically define provenance, freshness, or invalidation;
- semantic relationships between facts require additional modelling.

## Aytham obligation

Aytham must demonstrate that claims are more than named fields: subject identity, evidence lineage, scope and lifecycle must provide practical value.

---

# 3. Effect rows / capability approaches

## Model

Effects can be tracked separately:

```text
requires network_send
```

## Strengths

- explicit effect requirements;
- compositional reasoning about capabilities.

## Weaknesses

- effects answer "what may happen";
- they do not by themselves answer "which evidence is valid for this value".

## Aytham obligation

The claim system must combine naturally with effects without replacing mature capability ideas.

---

# 4. What Aytham must prove

Aytham should not claim novelty for:

- claims;
- provenance;
- capabilities;
- refinement-like conditions;
- contracts.

The possible contribution is integration:

```text
value identity
+
semantic claims
+
lineage
+
preservation/invalidation
+
requirements
+
shared explanations
```

in one bounded everyday programming model.

---

# 5. Required experiment before implementation

Before writing an Aytham validator, model the same scenario three ways:

1. proof-token API;
2. extensible/open record model;
3. Aytham claim environment.

Measure:

- adding a new claim kind;
- adding a new action requiring old/new claims;
- invalidating one claim while preserving others;
- explaining failures;
- keeping simple code simple.

Only a demonstrated advantage justifies a new language mechanism.
