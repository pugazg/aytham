# Experiment 025 — Minimal Aytham Validator Code Skeleton v0.1

## Objective

Define the first implementation boundary for the Aytham semantic validator prototype.

This is not a programming language runtime yet. It is a semantic verification engine.

## Proposed structure

```
prototype/
  validator/
    graph_loader.py
    semantic_graph.py
    validator.py
    explanation.py

  examples/
    banking_transfer.json
    loan_approval.json
    cobol_batch_update.json
    sangam_interpretation.json

  tests/
    test_claims.py
    test_actions.py
    test_transformations.py
    test_provenance.py
```

## Component responsibilities

### graph_loader.py

Loads and normalises Aytham semantic graph data.

Responsibilities:

- parse graph input
- validate basic structure
- create in-memory representation

### semantic_graph.py

Represents:

- Entity
- Relation
- Action
- Claim
- Transformation
- Evidence
- Context

### validator.py

Initial rules:

- required claim exists
- claim confidence is sufficient
- action requirements are satisfied
- transformation references are valid

### explanation.py

Generates semantic explanations:

- Why allowed?
- Why blocked?
- What claim is missing?
- What transformation may bridge the gap?

## First execution target

Input:

```
Action: ApproveLoan
Requires: income_verified
Available: identity_verified
```

Expected output:

```
Cannot execute ApproveLoan.

Missing claim:
    income_verified

Possible transformation:
    VerifyIncome
```

## Design principle

Aytham should explain semantic state transitions, not only report errors.

## Next step

Implement the smallest runnable Python prototype and validate against example graphs.
