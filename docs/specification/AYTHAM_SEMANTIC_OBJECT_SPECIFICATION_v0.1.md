# Aytham Semantic Object Specification v0.1

## Purpose

This document defines the first formal structure for the Aytham semantic model. It does not define programming syntax. It defines the meaning units that a future language implementation may express.

## Core model

The provisional core contains:

- Entity
- Relation
- Action
- Claim
- Transformation

Supporting semantic information:

- Evidence
- Context
- Authority

---

## Entity

An Entity represents a computational object with identity and evolving meaning.

```text
Entity {
    identity
    observations
    relations
    claims
    provenance
}
```

An entity is not only a stored value. It can accumulate validated knowledge through transformations.

---

## Relation

A Relation represents a meaningful connection between entities.

```text
Relation {
    source
    target
    relationship_type
    context
    evidence
}
```

Relations may be explicit, derived, or inferred.

---

## Action

An Action represents a meaningful change.

```text
Action {
    identity
    participants
    requirements
    context
    effects
    produced_claims
}
```

Actions are evaluated by meaning and consequences, not only by input and output types.

---

## Claim

A Claim represents a statement about an entity or relation.

```text
Claim {
    subject
    property
    value
    evidence
    authority
    confidence
    validity
}
```

Claims may be:

- established
- derived
- inferred
- unknown
- disputed

---

## Transformation

A Transformation changes the state or meaning of an entity.

```text
Transformation {
    input
    operation
    output
    evidence
    lineage
}
```

Example:

```text
RawEmail
   |
   ParseEmail
   |
EmailAddress
   |
   VerifyOwnership
   |
TrustedEmail
```

---

## Evidence, Context and Authority

These are supporting semantic dimensions.

Evidence explains why a claim exists.

Context explains the situation in which meaning is evaluated.

Authority explains the trust level of a source or evidence.

---

## Design principle

Aytham should preserve the distinction between:

```text
Observation
    -> Interpretation
    -> Claim
    -> Validated knowledge
```

A future compiler should not only check structure. It should reason about semantic requirements and transformations.
