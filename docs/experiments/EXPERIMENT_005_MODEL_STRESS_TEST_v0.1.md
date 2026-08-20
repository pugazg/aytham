# Experiment 005 — Minimal Semantic Core Stress Test v0.1

## Objective

Test whether the proposed Aytham minimal semantic core is sufficient across different domains before considering syntax design.

Minimal core under test:

- Entity
- Relation
- Action
- Claim
- Evidence
- Transformation

## Test 1 — Simple program: Invoice calculation

Question:
Can ordinary computation be represented without losing clarity?

Example concepts:

- Invoice entity
- Line item relations
- CalculateTotal action
- Claim: total_verified
- Evidence: calculation rules
- Transformation: draft invoice → finalized invoice

Observation:
Arithmetic alone does not require the model, but business meaning around the calculation benefits from explicit claims and transformations.

## Test 2 — Business workflow: Loan approval

Question:
Can Aytham represent decisions that depend on evidence?

Example:

Action:
ApproveLoan

Requirements:
- identity_verified
- income_verified
- risk_assessed

Evidence:
- documents
- verification events
- assessment records

Produces:
- loan_approved

Observation:
This is a strong fit for ActionFrame and Evidence models.

## Test 3 — Knowledge workflow: Sangam interpretation

Question:
Can the model represent uncertain meaning?

Example:

Observation:
- imagery appears in poem

Possible claims:
- literal interpretation
- contextual interpretation

Evidence:
- source text
- grammar
- commentary

Status:
- explicit
- inferred
- uncertain

Observation:
This tests whether Aytham can preserve ambiguity rather than forcing premature truth values.

## Findings

The six primitives appear sufficient for expressing the examples, but some refinements may be required:

1. Context may need to become a first-class concept.
2. Authority may need to be separated from Evidence.
3. Relation and Action boundaries need further study.

## Current hypothesis

Aytham may be best understood as a semantic computation model where programs describe:

- entities,
- relationships,
- actions,
- transformations,
- claims,
- and evidence.

Syntax design remains deferred until the model is further validated.
