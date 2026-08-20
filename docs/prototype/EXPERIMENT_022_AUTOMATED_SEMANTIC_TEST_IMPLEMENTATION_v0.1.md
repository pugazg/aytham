# Experiment 022 — Automated Semantic Test Implementation v0.1

## Objective

Create the first automated validation cycle for the Aytham semantic engine.

This experiment moves from test design to executable verification.

## Scope

The first test suite validates:

- graph loading
- entity references
- claim matching
- action requirements
- transformation lineage
- provenance preservation

## Test structure

```
prototype/
  validator/
    validator.py
  examples/
    banking_transfer.json
    loan_approval.json
    cobol_batch_update.json
    sangam_interpretation.json
    ai_pipeline.json
  tests/
    test_requirements.py
    test_claims.py
    test_transformations.py
    test_provenance.py
```

## Initial test cases

### Requirement validation

Input:

```
Action: ApproveLoan
Requires: income_verified
```

Expected:

- pass when claim exists
- explain missing claim when absent

### Provenance validation

Expected:

- established claims retain evidence
- inferred claims retain uncertainty
- disputed claims cannot become facts silently

### Transformation validation

Expected:

```
Input Entity
    |
Transformation
    |
Output Entity
```

Lineage must remain traceable.

## Success criteria

The validator should produce explanations, not only boolean results.

Example:

```
Cannot execute action.
Missing claim:
    customer_verified
Suggested transformation:
    VerifyCustomer
```

## Next step

Implement the minimal Python test runner and execute the first semantic graph validation cycle.
