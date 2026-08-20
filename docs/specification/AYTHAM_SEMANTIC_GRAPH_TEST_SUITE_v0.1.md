# Experiment 013 — Aytham Semantic Graph Test Suite v0.1

## Status

Experimental validation specification only.

This document defines test cases for validating whether the Aytham semantic graph model is coherent enough to support future language design.

It does not define syntax or implementation.

---

# Objective

Validate the pipeline:

```text
Surface expression
        ↓
Semantic resolution
        ↓
Canonical graph
        ↓
Validation
        ↓
Explanation
```

The test suite checks whether Aytham can:

- represent meaning;
- preserve provenance;
- distinguish explicit and inferred information;
- explain failures;
- avoid hidden assumptions.

---

# Test 001 — Banking transfer

## Scenario

A customer transfers money between accounts.

## Expected semantic graph

```text
AccountA -[source-of]-> Transfer#1
AccountB -[destination-of]-> Transfer#1
Amount100 -[amount-of]-> Transfer#1

Transfer#1 -[requires]-> DebitAuthorized(AccountA)
Transfer#1 -[requires]-> Positive(Amount100)

Transfer#1 -[establishes]-> Completed
```

## Validation questions

Can the system answer:

- Why was the transfer allowed?
- Which claims were required?
- Which evidence established those claims?

## Failure example

Missing:

```text
DebitAuthorized(AccountA)
```

Expected explanation:

```text
Transfer cannot execute.

Missing claim:
DebitAuthorized(AccountA)

Possible transformation:
AuthorizeDebit
```

---

# Test 002 — COBOL-style batch transaction

## Scenario

A nightly batch validates customer records and updates balances.

## Semantic flow

```text
InputRecord
    |
ValidateCustomer
    |
ValidatedCustomer
    |
UpdateBalance
    |
UpdatedAccount
```

## Validation questions

Can Aytham represent:

- record lineage;
- validation status;
- batch action effects;
- audit evidence?

This test ensures the model is useful for enterprise systems, not only modern applications.

---

# Test 003 — AI data pipeline

## Scenario

A model produces a prediction.

## Expected graph

```text
Dataset
  |
CleanData
  |
FeatureSet
  |
ModelInference
  |
Prediction
```

Each transformation should preserve:

- input origin;
- operation;
- output meaning;
- confidence;
- model authority.

---

# Test 004 — Sangam interpretation

## Scenario

A poem is analysed with commentary support.

## Expected semantic chain

```text
Observation
      ↓
Interpretation
      ↓
Claim
      ↓
Evidence
      ↓
Confidence
```

Example states:

```text
Observation:
 mountain imagery appears

Claim:
 possible landscape association

Evidence:
 source text
 commentary

Confidence:
 inferred
```

The system must preserve uncertainty.

---

# Test 005 — Simple computation

## Scenario

Calculate an invoice total.

Expected result:

Aytham should not force unnecessary semantic overhead.

A simple computation should remain simple.

This tests progressive disclosure.

---

# Cross-test acceptance criteria

The semantic model succeeds only if it can:

1. explain why an operation succeeds;
2. explain why an operation fails;
3. preserve lineage;
4. distinguish evidence from interpretation;
5. represent uncertainty;
6. avoid requiring unnecessary metadata.

---

# Current hypothesis

Aytham's possible contribution is not any individual feature:

- provenance;
- effects;
- contracts;
- semantic roles;
- knowledge graphs.

These already exist.

The hypothesis is that they can be unified into one programmer-facing semantic graph with better explanation and validation capabilities.

---

# Next step

If this test suite remains coherent, the next activity is to define the first semantic graph schema and a reference serializer.
