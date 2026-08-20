# Experiment 029 — Transformation Discovery Engine v0.1

## Objective

Define how Aytham can discover possible semantic transformations that bridge the gap between the current state and a required state.

This experiment extends validation from:

`missing requirement -> error`

into:

`missing requirement -> possible semantic path`

## Core idea

A transformation is a bridge between semantic states.

Example:

```
Email
 |
 VerifyOwnership
 |
EmailOwnershipVerified
```

## Transformation model

A transformation contains:

- input state
- operation
- output state
- requirements
- produced claims
- evidence

Example:

```
Transformation:
    VerifyIncome

Input:
    Customer

Requires:
    IncomeDocument

Produces:
    income_verified
```

## Discovery process

1. Identify missing claim.
2. Search available transformations.
3. Check transformation requirements.
4. Rank possible paths.
5. Explain the recommended next step.

## Example

Current state:

```
Customer42

claims:
    identity_verified
```

Required:

```
income_verified
```

Possible result:

```
Cannot complete ApproveLoan.

Missing:
    income_verified

Suggested transformation:
    VerifyIncome

Required evidence:
    IncomeDocument
```

## Design principles

- Never invent a transformation.
- Preserve provenance.
- Keep inferred paths separate from executed transformations.
- Explanations are part of the result.

## Relation to Tolkappiyam-inspired model

This reflects the idea that meaning emerges through relationships and context. A missing meaning is not only an error; it may represent an unfinished transformation path.

## Next step

Implement a small transformation graph and test path discovery.