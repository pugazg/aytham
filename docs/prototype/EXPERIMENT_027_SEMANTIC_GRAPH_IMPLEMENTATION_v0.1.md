# Experiment 027 — Semantic Graph Implementation v0.1

## Objective

Implement the first executable Aytham semantic object system.

This experiment moves from the object model definition into a minimal runtime representation.

## Scope

The first implementation contains:

- Entity
- Claim
- Action
- Transformation
- SemanticGraph

The implementation intentionally excludes:

- parser
- surface syntax
- compiler
- optimisation

## Semantic Objects

### Entity

Represents a meaningful object with identity, claims and relationships.

### Claim

Represents knowledge about an entity while preserving confidence state.

Supported states:

- established
- derived
- inferred
- unknown
- disputed

### Action

Represents a semantic change requiring conditions to be satisfied.

### Transformation

Represents movement from one semantic state to another while preserving lineage.

### SemanticGraph

Container responsible for storing and resolving semantic objects.

## First Runtime Test

Scenario:

Customer42 has:

```
identity_verified = true
```

Action:

```
ApproveLoan
```

Requirement:

```
identity_verified
```

Expected result:

```
Action allowed
```

Failure case:

Missing requirement should produce an explanation rather than a generic error.

## Design Principle

Aytham should answer not only:

"Can this execute?"

but also:

"Why can this execute?"
"What evidence supports this?"
"What is missing?"

## Next Step

Connect the semantic objects with the validator and run automated tests.
