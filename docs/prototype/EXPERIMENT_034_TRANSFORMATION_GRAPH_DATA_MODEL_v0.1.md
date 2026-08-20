# Experiment 034 — Transformation Graph Data Model v0.1

## Purpose

Define the first executable data model for representing semantic transformations in Aytham.

The goal is to represent meaning changes as validated graph transitions rather than ordinary function calls.

## Core model

```
Current Semantic State
          |
          | Transformation
          v
New Semantic State
```

## Components

### TransformationNode

Represents a semantic state.

Examples:

- identity_verified
- income_verified
- loan_eligible
- loan_approved

Properties:

- identifier
- claims represented
- provenance
- confidence

### TransformationEdge

Represents a valid meaning-changing operation.

Properties:

- transformation name
- input state
- output state
- required evidence
- produced claims
- confidence

Example:

```
identity_verified
        |
   VerifyIncome
        |
income_verified
```

### SemanticPath

Represents a sequence of transformations from current state to goal state.

Example:

```
identity_verified
 -> VerifyIncome
 -> income_verified
 -> CalculateEligibility
 -> loan_eligible
 -> ApproveLoan
 -> loan_approved
```

### PathSearchResult

Contains:

- whether a path exists
- ordered transformations
- missing prerequisites
- explanation

## Design rules

1. Transformations must be registered before use.
2. A proposed path is not execution.
3. Every transition must preserve provenance.
4. Ambiguous paths must be reported, not silently selected.

## Connection to Aytham principles

The transformation graph enables Aytham to reason about how a valid state can be reached while preserving evidence and explanation.

This keeps the original semantic focus: relationships and context determine meaning.
