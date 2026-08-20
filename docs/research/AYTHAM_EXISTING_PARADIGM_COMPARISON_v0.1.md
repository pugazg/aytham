# Aytham Existing Paradigm Comparison v0.1

## Purpose

This document compares the current Aytham research hypotheses against existing programming-language concepts. The purpose is not to claim novelty prematurely, but to identify where Aytham may provide a different integration or developer experience.

## Core principle

Aytham is not attempting to rename existing concepts with Tamil terminology. Each proposed concept must demonstrate a useful computational difference.

---

# 1. Semantic Object

## Aytham hypothesis

A computational entity may carry:

- identity
- observations
- relations
- claims
- evidence
- transformations
- provenance

## Related existing concepts

- Objects/classes
- Knowledge graphs
- Entity-relationship models
- Semantic data models

## Research question

Can a programming entity naturally combine identity, meaning, evidence and history instead of requiring separate systems?

---

# 2. ActionFrame

## Aytham hypothesis

An action is represented through:

- actor
- participants
- context
- time
- requirements
- established claims
- effects

## Related existing concepts

- Functions
- Events
- Workflow systems
- Frame semantics
- Event sourcing
- Actor models

## Research question

Can actions become primary semantic units rather than only callable procedures?

---

# 3. Claims and Evidence

## Aytham hypothesis

A fact may contain:

- value
- evidence
- authority
- confidence
- validity

## Related existing concepts

- Refinement types
- Assertions
- Proof-carrying code
- Provenance systems
- Contracts

## Research question

Can evidence-aware computation become usable in everyday programming rather than remaining an advanced verification technique?

---

# 4. Validated Data Flow

## Aytham hypothesis

Values accumulate meaning through transformations.

Example:

Raw input
→ Parsed value
→ Validated value
→ Trusted value

## Related existing concepts

- Typestate
- Dependent types
- Refinement types
- Data lineage
- ETL validation pipelines

## Research question

Can validation history become part of normal programming semantics?

---

# 5. Semantic Composition

## Aytham hypothesis

Composition should consider whether produced meaning satisfies required meaning.

## Related existing concepts

- Type checking
- Effect systems
- Capability systems
- Protocol/session types

## Research question

Can compiler diagnostics explain missing meaning rather than only reporting incompatible structures?

---

# Current possible differentiation

The individual ideas have prior art. The possible research contribution is their integration:

```
Entity
  +
Relations
  +
Actions
  +
Claims
  +
Evidence
  +
Context
  +
Lineage
```

as one programming model.

---

# Current conclusion

Aytham should not attempt to prove:

"Tamil grammar invented a new programming concept."

The stronger and more defensible goal is:

"Can a semantic programming model inspired by Tamil grammatical and literary structures provide a clearer way to express meaning, relationships and validated transformations?"

---

# Next validation step

Create small experimental programs and compare Aytham-style modelling against conventional approaches:

1. Banking workflow
2. AI/data pipeline
3. Knowledge management system
4. Literary interpretation system

The model should demonstrate measurable advantages before language design begins.
