# Experiment 030 — Transformation Graph Prototype v0.1

## Objective

Define the first prototype model for discovering semantic paths between a current state and a required state.

Aytham should not only answer:

> Is the requirement satisfied?

It should also answer:

> What transformation can establish the missing meaning?

---

## Core model

```
Current Claim
      |
      v
Transformation
      |
      v
New Claim
```

Example:

```
email : syntax_valid

        |
        | VerifyOwnership
        v

email : ownership_verified
```

---

## Transformation graph

A transformation is represented as a directed semantic edge.

```text
Claim A
  |
  | transformation
  v
Claim B
```

The graph stores:

- input claims
- output claims
- required evidence
- confidence after execution
- provenance

---

## Discovery behaviour

Given:

```
Required:
    income_verified

Available:
    identity_verified
```

The engine searches registered transformations:

```
identity_verified
        |
        | VerifyIncomeEligibility
        v
income_verified
```

Result:

```
Suggested path:
    VerifyIncomeEligibility

Required evidence:
    IncomeDocument
```

---

## Important rules

### No invented paths

Only known transformations may be suggested.

### Planned is not executed

States remain separate:

```
possible
planned
executed
verified
```

### Multiple paths allowed

A requirement may have several valid semantic routes.

---

## Relation to Aytham principles

This experiment extends:

- semantic validation
- provenance tracking
- explanation-first computing
- Tolkappiyam-inspired relationship modelling

The next step is implementing a small graph traversal engine to discover transformation paths.