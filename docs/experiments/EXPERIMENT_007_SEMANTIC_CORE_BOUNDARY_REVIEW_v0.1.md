# Experiment 007 — Semantic Core Boundary Review v0.1

## Purpose

Review the current Aytham semantic core after Experiments 003–006 and decide which concepts are fundamental and which are supporting concepts.

Current candidate:

- Entity
- Relation
- Action
- Claim
- Evidence
- Transformation
- Context
- Authority

## Review principle

Aytham should avoid becoming a collection of concepts. Every primitive must justify why it exists as a first-class abstraction.

## Findings

### Entity

Status: Core candidate.

Reason:

All computation requires identifiable things. Aytham extends this with identity, observations, claims and provenance.

### Relation

Status: Core candidate.

Reason:

Meaning is not only stored inside entities. Relationships between entities are fundamental.

Open question:

Can some relations be represented as action outcomes?

### Action

Status: Core candidate.

Reason:

Actions represent meaningful change and connect participants, requirements and effects.

### Transformation

Status: Core candidate.

Reason:

A transformation changes not only values but what is known about values.

### Claim

Status: Core candidate.

Reason:

Aytham requires a distinction between observation and accepted meaning.

### Evidence

Status: Supporting-to-core boundary.

Reason:

Evidence may be represented as a relationship to a claim, but removing it loses validated data flow.

### Context

Status: Supporting-to-core boundary.

Reason:

Context affects interpretation and meaning resolution. Further testing required.

### Authority

Status: Supporting concept.

Reason:

Authority determines trust level of evidence, but may not need to be a top-level primitive.

## Provisional core

The current smallest candidate is:

```
Entity
Relation
Action
Claim
Transformation
```

with:

```
Evidence
Context
Authority
```

as semantic attributes or supporting structures until further experiments prove otherwise.

## Important conclusion

Aytham should not maximize concepts. The goal is a minimal semantic model capable of expressing:

- meaningful state change
- evidence-backed knowledge
- contextual interpretation
- traceable transformations

## Next step

Define a formal object model for the provisional core and test whether Evidence, Context and Authority can be represented without losing expressive power.
