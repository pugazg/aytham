# Experiment 021 — Executable Semantic Graph Test Suite v0.1

## Objective

Move from validator examples to repeatable automated validation of the Aytham semantic model.

The purpose is not to test a programming language yet. It is to verify that the semantic graph, claims, transformations and explanations behave consistently.

## Test structure

```
prototype/
  validator/
  examples/
  tests/
```

## Test domains

### 1. Banking transfer

Validates:
- Entity
- Action
- Requirement checking
- Established claims
- Effects

Expected:

```
Transfer allowed.
Requirement satisfied:
Account : debit_authorized
```

### 2. Loan approval

Validates missing semantic requirements.

Example:

```
Action:
 ApproveLoan

Requires:
 income_verified

Available:
 identity_verified
```

Expected explanation:

```
Missing claim:
income_verified
```

### 3. COBOL batch processing

Validates enterprise lineage.

Flow:

```
RawRecord
  |
ValidateRecord
  |
ValidatedRecord
  |
UpdateDatabase
```

Checks:
- transformation lineage
- batch state changes
- audit claims

### 4. Sangam interpretation

Validates uncertainty handling.

Flow:

```
Observation
  |
Interpretation
  |
Claim
  |
Evidence
```

Checks:
- inferred vs explicit knowledge
- commentary/source separation
- confidence preservation

### 5. AI pipeline

Validates data lineage.

Flow:

```
Dataset
 |
Cleaning
 |
FeatureExtraction
 |
Prediction
```

Checks:
- transformation history
- model authority
- prediction provenance

## Initial assertions

The test suite should verify:

1. Invalid references are rejected.
2. Missing claims produce explanations.
3. Valid claims allow actions.
4. Transformations preserve lineage.
5. Inferred claims are not treated as established facts.

## Success criteria

Aytham succeeds if the validator can answer:

- Why did this action succeed?
- Why did this action fail?
- How is this claim known?
- What transformation created this state?
- What information is missing?

