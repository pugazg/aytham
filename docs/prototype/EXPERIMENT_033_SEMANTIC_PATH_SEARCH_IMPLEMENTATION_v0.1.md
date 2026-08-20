# Experiment 033 — Semantic Path Search Implementation v0.1

## Objective

Implement the first executable reasoning component for Aytham: finding a validated semantic path from a current state to a target state.

This experiment does not introduce programming language syntax. It operates on the canonical semantic graph.

## Input

A current semantic state:

```
Customer42 : identity_verified
```

A target state:

```
Customer42 : loan_approved
```

A transformation graph:

```
identity_verified
        |
   VerifyIncome
        |
income_verified
        |
CalculateEligibility
        |
loan_eligible
        |
ApproveLoan
        |
loan_approved
```

## Proposed components

```
prototype/

  transformation_graph.py
  path_search.py
  path_result.py
  explanation.py
```

## Transformation Graph

The graph stores:

- source claim/state
- target claim/state
- transformation identity
- required evidence
- confidence
- execution status

## Path Search Behaviour

The first implementation uses deterministic graph traversal.

It should:

1. Start from available claims.
2. Explore registered transformations.
3. Reject paths with unavailable prerequisites.
4. Return a valid semantic path if one exists.
5. Preserve provenance information.

## Example Output

```
Path found:

1. VerifyIncome
   establishes income_verified

2. CalculateEligibility
   establishes loan_eligible

3. ApproveLoan
   establishes loan_approved
```

## Failure Output

```
No valid semantic path found.

Missing prerequisite:
    income_document_verified
```

## Design Principles

- No invented transformations.
- Suggested paths are not executions.
- Every step retains provenance.
- Ambiguous paths must remain explicit.

## Connection to Aytham Research

This experiment extends the original semantic model inspired by Tamil grammatical and literary concepts:

- relationships between entities,
- contextual meaning,
- evidence-backed interpretation,
- transformation of state.

The goal is a system that can reason about how meaning changes, not merely manipulate values.
