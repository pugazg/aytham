# Aytham Semantic Model v0.1 (Research Draft)

## Status

Research draft. This document defines the current semantic model hypothesis. It is not a language specification and does not define syntax.

## Objective

Aytham is exploring whether programming can treat meaning, relationships, evidence, and context as first-class concepts.

The goal is not to create Tamil keywords for existing programming languages. The goal is to investigate a semantic programming model inspired by Tamil grammatical and literary analysis while being validated against modern programming language research.

---

# 1. Core Semantic Objects

An Aytham semantic object is not only a value. It may carry:

- Identity
- Representations
- Observations
- Relations
- Claims
- Evidence
- Transformations
- Provenance

Conceptual model:

```
SemanticObject
 |
 +-- Identity
 |
 +-- Observations
 |
 +-- Relations
 |
 +-- Claims
 |
 +-- Evidence
 |
 +-- Transformations
 |
 +-- Provenance
```

---

# 2. Observation Layer

A system should distinguish observation from interpretation.

Example:

```
Observation:
    text contains "rain"

Interpretations:
    literal weather event
    poetic imagery
    seasonal reference
```

The compiler or runtime should preserve this distinction.

---

# 3. Claim and Evidence Model

A claim should not only contain a value.

Conceptual model:

```
Claim
 |
 +-- value
 +-- authority
 +-- evidence
 +-- confidence
 +-- validity
```

Possible states:

- explicit
- derived
- inferred
- uncertain
- contested

---

# 4. ActionFrame Model

Functions traditionally describe input and output.

Aytham research explores describing actions through semantic roles.

```
ActionFrame
 |
 +-- Action
 +-- Participants
 +-- Context
 +-- Time
 +-- Requirements
 +-- Established claims
 +-- Effects
 +-- Result relations
```

Example:

```
Transfer
 |
 +-- source
 +-- destination
 +-- amount
 |
 +-- requires
 |     ownership_verified
 |
 +-- establishes
       transfer_completed
```

---

# 5. Composition Model

Composition should consider meaning, not only type compatibility.

Conceptual rule:

```
Produced meaning
        |
 Composition rule
        |
Required meaning
```

A failure should explain missing semantic requirements.

Example:

```
SendSensitiveMessage requires:
    ownership_verified

Available:
    email_format_valid

Suggested bridge:
    VerifyOwnership
```

---

# 6. Relationship to Tamil Sources

The sources are inspiration and research evidence, not direct programming definitions.

## Tolkappiyam

Provides grammatical categories and relationships.

## Nannul and commentaries

Provide comparative interpretation and grammatical explanation.

## Sangam literature

Provides usage, context, inference and provenance challenges.

---

# 7. Current Research Questions

1. Can ActionFrame provide advantages over ordinary function signatures?

2. Can evidence-aware values improve software correctness?

3. Can context-aware interpretation be made explicit in programming?

4. Can semantic diagnostics be more useful than traditional type errors?

---

# 8. Non-goals

Aytham is not currently attempting to:

- replace existing programming languages
- translate Tamil grammar directly into syntax
- claim ancient Tamil grammar invented modern programming concepts

---

# 9. Next Validation Steps

The model must be tested against:

- existing programming language concepts
- real software workflows
- data and AI pipelines
- literary interpretation workflows

Only after validation should syntax or implementation be considered.
