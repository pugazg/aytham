# Experiment 003A — Banking ActionFrame Case Study v0.1

## Purpose

This experiment tests whether the Aytham semantic model provides useful explanations for a common software workflow.

Scenario: a financial transfer operation.

This is not a claim that banking concepts come from Tamil grammar. The purpose is to test whether the proposed model is computationally useful.

---

## Conventional programming model

Typical representation:

```
transfer(sourceAccount, destinationAccount, amount)
```

The function signature describes values, but additional meaning is usually distributed across:

- documentation
- validation code
- database constraints
- authorization rules
- audit logs

---

## Aytham research representation

```
ActionFrame: Transfer

Participants:
  sourceAccount
  destinationAccount
  amount

Required claims:
  sourceAccount.debit_authorized
  amount.positive
  destinationAccount.active

Established claims:
  transfer.completed

Effects:
  source.balance_changed
  destination.balance_changed
  ledger.updated

Evidence:
  transaction_record
  authorization_event
```

---

## Research questions

### Q1. Explanation

Can the system explain why an action is allowed?

Example:

```
Transfer cannot execute.

Missing claim:
  sourceAccount.debit_authorized

Available evidence:
  account_exists
```

---

### Q2. Provenance

Can every important claim answer:

```
Who established it?
When?
Based on what evidence?
```

---

### Q3. Composition

Can actions connect through semantic compatibility?

Example:

```
VerifyAccount
      |
      v
Transfer
```

is valid because the first action establishes a required claim.

---

## Comparison with existing systems

This overlaps with:

- design by contract
- typestate
- refinement types
- workflow systems
- event sourcing

The experiment does not claim these concepts are new.

The research question is whether combining them into a single semantic action model improves programmer understanding.

---

## Current assessment

Status: OPEN

Success condition:

The model should produce explanations and reasoning that are clearer than ordinary type/error messages.

Failure condition:

The model reduces to renamed existing abstractions without additional value.
