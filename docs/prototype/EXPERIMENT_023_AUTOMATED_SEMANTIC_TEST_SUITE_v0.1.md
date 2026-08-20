# Experiment 023 — Automated Semantic Test Suite v0.1

## Purpose

Define the first executable test cycle for Aytham's semantic engine.

The goal is not to test syntax. The goal is to verify whether the semantic graph model behaves consistently.

## Test categories

### 1. Requirement validation

Verify that actions can check required claims.

Example:

Action: ApproveLoan

Requires:
- identity_verified
- income_verified

Expected behaviour:
- allow when all required claims exist
- explain missing claims when requirements are incomplete

### 2. Claim state validation

Verify that claim confidence is preserved.

States:
- established
- derived
- inferred
- unknown
- disputed

Rule:

An inferred claim must not silently become an established claim.

### 3. Transformation lineage

Verify that semantic changes preserve origin.

Example:

RawCustomer
  -> VerifyIdentity
VerifiedCustomer

The engine should answer:
- What changed?
- Which transformation caused it?
- What claims were created?

### 4. Provenance validation

Verify that claims can retain:

- evidence
- authority
- confidence
- origin

## Initial test domains

1. Banking transaction
2. COBOL-style batch processing
3. Sangam interpretation workflow
4. AI/data pipeline

## Success criteria

Aytham should provide explanations instead of only boolean validation results.

Example:

Cannot execute action.

Missing claim:
    income_verified

Possible next transformation:
    VerifyIncome

## Next step

Implement runnable tests against semantic graph examples.