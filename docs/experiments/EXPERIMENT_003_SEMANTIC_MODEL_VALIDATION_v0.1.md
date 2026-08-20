# Experiment 003 — Semantic Model Validation v0.1

## Purpose

Validate whether the Aytham Semantic Model provides a useful abstraction before any language syntax or compiler work begins.

This experiment does not attempt to prove that Aytham is a replacement for existing programming languages. It tests whether the semantic model improves representation, explanation, and reasoning.

---

## Hypothesis

A program can be represented more clearly when the following are first-class concepts:

- entities
- relationships
- actions
- claims
- evidence
- transformations
- provenance
- context

---

## Test cases

### Case 1 — Banking workflow

Purpose:

Validate ActionFrame, claims, evidence, and effects.

Example concepts:

- transfer action
- source account
- destination account
- authorization claim
- transaction evidence
- balance effects

Questions:

- Can the model explain why an action is allowed?
- Can it identify missing prerequisites?
- Can it preserve lineage after transformations?

---

### Case 2 — AI/Data pipeline

Purpose:

Validate transformation lineage.

Example:

raw data
→ cleaning
→ model input
→ prediction

Questions:

- Can every output trace its origin?
- Can confidence and evidence be preserved?

---

### Case 3 — Literary interpretation

Purpose:

Validate observation, inference, and uncertainty.

Example:

Observation:

A word or image appears in a text.

Possible interpretations:

- explicit meaning
- inferred meaning
- uncertain meaning

Questions:

- Can the system separate evidence from interpretation?
- Can competing interpretations coexist?

---

### Case 4 — Knowledge system

Purpose:

Validate evolving claims.

Example:

Entity:

person/place/concept

Claims:

- known facts
- inferred facts
- disputed facts

Questions:

- Can knowledge change without losing history?

---

## Success criteria

The experiment succeeds if the model provides better explanations for:

1. Why a computation is valid.
2. Why a computation is rejected.
3. Where information came from.
4. How meaning changes over time.

---

## Failure criteria

The experiment fails if the model is only a renamed combination of:

- type systems
- object models
- provenance systems
- workflow engines

without providing a clearer programming model.

---

## Status

Research experiment. No syntax or implementation decisions are made.