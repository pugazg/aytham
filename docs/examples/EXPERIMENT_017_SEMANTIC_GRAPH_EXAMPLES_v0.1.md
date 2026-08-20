# Experiment 017 — Semantic Graph Examples v0.1

## Purpose

This document provides first concrete examples using the Aytham canonical semantic graph model.

The goal is validation of the schema, not language syntax.

## Example 1 — Banking Transfer

### Intent

Represent a transfer where execution depends on verified claims.

```text
Entity:
  AccountA
  AccountB
  Amount100

Action:
  Transfer

Participants:
  source = AccountA
  destination = AccountB
  amount = Amount100

Requires:
  AccountA : debit_authorized
  Amount100 : positive

Establishes:
  transfer_completed

Effects:
  AccountA.balance changed
  AccountB.balance changed
```

## Example 2 — COBOL Batch Update

### Intent

Represent a traditional enterprise batch flow with lineage.

```text
Entity:
  CustomerRecord

Transformation:
  ValidateCustomerRecord

Input:
  RawCustomerRecord

Output:
  ValidatedCustomerRecord

Action:
  UpdateAccount

Evidence:
  ValidationReport
  AuditRecord
```

## Example 3 — Sangam Interpretation

### Intent

Preserve observation, interpretation and confidence separately.

```text
Observation:
  Landscape imagery appears in poem

Context:
  Poem situation
  Literary framework

Claim:
  Possible landscape association

Evidence:
  Source text
  Commentary

Confidence:
  inferred
```

## Example 4 — AI Pipeline

```text
Entity:
  Dataset

Transformation:
  DataCleaning

Transformation:
  FeatureExtraction

Action:
  ModelInference

Claim:
  PredictionResult

Evidence:
  ModelVersion
  EvaluationMetrics
```

## Validation questions

For every graph:

1. Can the system identify entities?
2. Can it explain actions?
3. Can it trace transformations?
4. Can it show evidence for claims?
5. Can it preserve uncertainty?
