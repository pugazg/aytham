# Experiment 004 — Minimal Semantic Core v0.1

## Objective

Discover the smallest expressive core required by Aytham before considering syntax or implementation.

This experiment intentionally avoids designing a programming language. It tests whether the semantic model can be represented with a small set of primitives.

## Research question

Can the following concepts form a minimal computational foundation?

- Entity
- Relation
- Action
- Claim
- Evidence
- Transformation
- Composition

## Candidate semantic primitives

### Entity

A distinguishable thing with identity and history.

Possible attributes:

- identity
- representations
- observations
- relations
- claims
- provenance

---

### Relation

A named connection between entities.

Examples:

- owns
- belongs_to
- verified_by
- produced_from

Relations should preserve whether they are:

- explicit
- inferred
- derived
- uncertain

---

### Action

A meaningful transformation involving participants and context.

Structure:

```
Action
  participants
  context
  requirements
  transformations
  effects
  established_claims
```

---

### Claim

A statement about an entity or action.

Structure:

```
Claim
  subject
  property
  value
  authority
  evidence
  confidence
  validity
```

---

### Evidence

Information supporting a claim.

Evidence answers:

- where did this come from?
- how was it produced?
- can it be trusted?

---

### Transformation

A change that may alter the meaning or trusted state of an entity.

Example:

```
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

## Initial conclusion

The smallest promising core appears to be:

```
Entity
Relation
Action
Claim
Evidence
Transformation
```

Composition appears to be a rule operating on these primitives rather than a primitive itself.

## Open questions

1. Is Relation fundamental or can all relations be represented as Actions?
2. Is Claim separate from Entity state or part of Entity?
3. Is Evidence a property of Claim or a first-class object?
4. Can this model express ordinary programs without excessive complexity?

## Next step

Test the minimal core against concrete examples before introducing syntax.