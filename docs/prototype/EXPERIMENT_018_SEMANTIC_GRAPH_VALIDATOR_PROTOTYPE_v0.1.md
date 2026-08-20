# Experiment 018 — Aytham Semantic Graph Validator Prototype v0.1

## Objective

Define the first executable proof layer for Aytham.

This experiment does not create a programming language. It defines the minimum validator required to prove that the semantic graph model can reason about programs.

## Validator responsibilities

The prototype validator should:

1. Load an Aytham semantic graph.
2. Validate object identity.
3. Validate references between nodes.
4. Check action requirements.
5. Track established claims.
6. Explain failures.

## Processing flow

```
Semantic Graph JSON
        |
        v
Graph Loader
        |
        v
Semantic Validator
        |
  ----------------
  |              |
Valid Result   Explanation
```

## Validation categories

### Structural validation

Checks:

- required fields exist
- IDs are unique
- relations point to valid objects
- transformations have input/output

### Semantic validation

Checks:

- action requirements are satisfied
- required claims exist
- transformations can establish missing claims

## Example

Action:

```
ApproveLoan
```

Required claims:

```
identity_verified
income_verified
```

Available:

```
identity_verified
```

Validator output:

```
Cannot execute ApproveLoan.

Missing claim:
    income_verified

Suggested transformation:
    VerifyIncome
```

## Explanation model

The validator should answer:

### Why?

Why was an action allowed or rejected?

### How known?

Which evidence and transformations established a claim?

### What changed?

Which transformation changed the semantic state?

### What is missing?

Which claim or relation prevents execution?

## Initial implementation boundary

The first prototype should be small:

- JSON input
- in-memory graph
- rule-based validation
- text explanations

No compiler, parser, or runtime execution is required yet.

## Success criteria

The prototype succeeds if it demonstrates that Aytham can explain semantic failures better than simple type/error messages.
