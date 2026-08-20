# Experiment 020 — Validator Example Runs v0.1

## Purpose

Validate the first Aytham validator prototype against representative semantic graphs.

The goal is not language execution. The goal is explanation of semantic state.

## Scenario 1 — Banking Transfer

Input:

- Entity: AccountA
- Entity: AccountB
- Action: Transfer
- Requirement: debit_authorized

Expected:

Valid when the required claim exists.

Explanation:

```
Transfer allowed.
Requirement satisfied:
AccountA : debit_authorized
```

## Scenario 2 — Missing Requirement

Action:

```
ApproveLoan
```

Required:

```
income_verified
```

Available:

```
identity_verified
```

Expected explanation:

```
Cannot execute ApproveLoan.
Missing claim:
income_verified
```

## Scenario 3 — Transformation Suggestion

Available state:

```
email : syntax_valid
```

Required state:

```
email : ownership_verified
```

Expected:

```
Possible transformation:
VerifyOwnership
```

## Scenario 4 — Provenance Check

A claim should preserve:

- evidence source
- confidence
- authority
- origin transformation

Aytham should not convert inferred claims into established facts.

## Outcome

The validator concept is feasible for the next stage: implementing executable graph examples and automated tests.
