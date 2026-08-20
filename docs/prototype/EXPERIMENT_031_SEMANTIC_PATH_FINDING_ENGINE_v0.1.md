# Experiment 031 — Semantic Path Finding Engine v0.1

## Objective

Design the first reasoning layer that can discover semantic transformation paths between a current state and a desired state.

Aytham moves from:

```
Validation:
    Is the requirement satisfied?
```

to:

```
Reasoning:
    What transformations can establish the missing meaning?
```

## Core Model

A semantic path consists of:

```
Current Claim
      |
      | Transformation
      v
Intermediate Claim
      |
      | Transformation
      v
Goal Claim
```

## Example

Current:

```
Customer42 : identity_verified
```

Goal:

```
Customer42 : loan_approved
```

Possible path:

```
identity_verified
        |
        | VerifyIncome
        v
income_verified
        |
        | CalculateEligibility
        v
loan_eligible
        |
        | ApproveLoan
        v
loan_approved
```

## Discovery Rules

1. Only registered transformations can be used.
2. Suggested paths must not be treated as completed actions.
3. Every step must preserve provenance.
4. Missing evidence must be reported.
5. Multiple possible paths may exist.

## Future Implementation

The engine should support:

- graph traversal
- shortest semantic path discovery
- confidence-aware paths
- evidence requirements
- explanation generation

## Research Significance

This experiment represents a move from program execution toward semantic problem solving. It connects Aytham's original ideas from Tolkappiyam-inspired roles, context and relationships with executable reasoning structures.
