# Experiment 028 — Semantic Validator Integration v0.1

## Objective

Connect the SemanticGraph object model with validation behaviour.

The goal is the first end-to-end semantic execution flow:

SemanticGraph → Action requirement check → Explanation

## Flow

1. Create Entity
2. Attach Claim
3. Create Action
4. Validate requirements
5. Generate explanation

## Example

Entity:

Customer42

Claim:

identity_verified = true

Action:

ApproveLoan

Requirement:

identity_verified

Result:

Action allowed.

## Failure example

Action:

ApproveLoan

Required:

income_verified

Available:

identity_verified

Result:

Cannot execute ApproveLoan.

Missing claim:

income_verified

## Design observations

- Validation operates on semantic meaning rather than raw values.
- Explanation is a first-class output.
- Claims retain confidence and provenance information.
- Transformations remain the future bridge for satisfying missing requirements.

## Next step

Extend the validator with transformation discovery and provenance tracing.
