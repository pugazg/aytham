# Experiment 026 — Semantic Graph Classes v0.1

## Objective

Define the first executable object model for Aytham's canonical semantic graph.

This experiment does not define Aytham syntax. It defines internal objects used by the semantic engine.

## Initial Classes

### Entity

Represents a meaningful object.

Fields:

- id
- type
- attributes
- claims
- relations

Examples:

- Account
- Customer
- Poem
- Dataset

---

### Claim

Represents a statement about an entity.

Fields:

- subject
- property
- value
- confidence
- evidence
- authority

Confidence values:

- established
- derived
- inferred
- unknown
- disputed

---

### Action

Represents a semantic operation.

Fields:

- id
- participants
- requirements
- effects
- produced_claims

An Action should answer:

- Who participates?
- What conditions are required?
- What meaning changes after execution?

---

### Transformation

Represents change of semantic state.

Fields:

- input
- operation
- output
- lineage

Example:

RawCustomer → VerifyIdentity → VerifiedCustomer

---

### SemanticGraph

Container for all semantic objects.

Responsibilities:

- add entities
- add claims
- add actions
- resolve references
- provide graph traversal

---

## First Validation Goal

Create a graph:

Customer

has claim:

identity_verified = true

Action:

ApproveLoan

requires:

identity_verified

Expected:

Action can execute.

---

## Design Principle

The object model should preserve meaning, provenance and explanation capability before introducing any surface programming syntax.
