# Benchmark 001 — Comparative Modeling Exercise

## Purpose

Compare three approaches using the same semantic problem:

1. Proof-token API
2. Extensible/open fact record
3. Aytham claim environment

This is not a performance benchmark. It is a modelling benchmark.

The question is:

> Which model makes semantic requirements, invalidation, preservation, and diagnostics easiest to express?

---

## Scenario

An email identity is used by multiple actions.

Facts:

```
Email
 ├── syntax_valid
 ├── ownership_verified
 ├── marketing_consent
 ├── mfa_verified
 └── jurisdiction_allowed
```

Actions:

```
SendSecurityAlert
    requires ownership_verified + mfa_verified

SendMarketingMessage
    requires ownership_verified + marketing_consent

SendRegulatedNotice
    requires ownership_verified + jurisdiction_allowed
```

Transformation:

```
RevokeMarketingConsent

removes:
    marketing_consent

must preserve:
    ownership_verified
    mfa_verified
    jurisdiction_allowed
```

---

## Evaluation criteria

Each approach will be evaluated on:

### Adding a new fact

How much code changes are required?

### Adding a new action

Can the action declare requirements directly?

### Invalidation

Can one fact become invalid without affecting unrelated facts?

### Diagnostics

Can the system explain:

- missing fact
- wrong subject
- stale evidence
- invalidated lineage

### Evidence burden

How much manual annotation is needed?

---

## Current hypothesis

Aytham should not compete by merely storing facts.

Existing systems already support:

- proof tokens
- capabilities
- generic state markers
- extensible records

The possible Aytham contribution is a unified semantic layer:

```
Claim
 + subject
 + scope
 + provenance
 + validity
 + preservation rules
```

combined with action requirements and effects.

---

## Status

Pending implementation of the three equivalent models.
