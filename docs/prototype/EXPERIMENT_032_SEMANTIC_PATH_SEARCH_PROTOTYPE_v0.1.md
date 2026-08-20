# Experiment 032 — Semantic Path Search Prototype v0.1

## Objective

Design the first reasoning component that can discover a valid semantic path from a current state to a desired state.

## Problem

Earlier Aytham validation could identify missing requirements:

```
Missing claim: income_verified
```

This experiment adds the ability to ask:

```
How can this claim be established?
```

## Model

A semantic path consists of:

```
Current Claim
    |
    | Transformation
    v
New Claim
    |
    | Transformation
    v
Goal Claim
```

## Example

Starting state:

```
Customer42 : identity_verified
```

Goal:

```
Customer42 : loan_approved
```

Transformation graph:

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

## Search behaviour

The first prototype should support:

- graph traversal
- goal detection
- path reconstruction
- transformation explanation

## Constraints

Aytham must not invent transformations.

Only registered transformations can participate in reasoning.

Suggested paths are not executed actions. They remain proposals until separately verified.

## Future implementation

Planned components:

```
transformation_graph.py
path_search.py
path_explanation.py
```

The goal is not general AI planning. The goal is deterministic semantic reasoning over validated transformations.
