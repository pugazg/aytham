# Experiment 009 — Semantic Model Real Programming Examples v0.1

## Objective

Test whether the Aytham semantic core can express real software problems without immediately becoming a syntax replacement for existing languages.

The current semantic core under evaluation:

- Entity
- Relation
- Action
- Claim
- Transformation

Supporting concepts:

- Evidence
- Context
- Authority

---

## Case 1 — User Registration Workflow

### Conventional model

```text
register(username, email, password)
```

The meaning is distributed across validation functions, database constraints and business rules.

### Aytham model

```text
Entity:
    User

Action:
    RegisterUser

Requirements:
    email_format_valid
    password_policy_satisfied

Transformations:
    RawInput -> ValidatedUser

Claims produced:
    user_identity_created
```

Observation:

The model makes state transitions and reasons visible.

---

## Case 2 — Mainframe Business Transaction

Example domain:

```text
Customer Account Update
```

### Aytham model

```text
Entity:
    Account

Action:
    UpdateAccount

Participants:
    Customer
    System

Requirements:
    customer_verified
    transaction_authorized

Effects:
    account_state_changed

Evidence:
    audit_record
```

Observation:

This maps naturally to long-running enterprise systems where auditability and business rules matter.

---

## Case 3 — Knowledge Interpretation

Example:

```text
Observation:
    statement appears in source

Context:
    historical period

Claim:
    possible interpretation

Evidence:
    source references

Confidence:
    inferred
```

Observation:

The same model can represent uncertain knowledge without forcing a boolean answer.

---

## Findings

The semantic model appears strongest where software involves:

- meaningful state changes
- business rules
- validation
- provenance
- human interpretation

It is not intended to replace:

- arithmetic expressions
- simple algorithms
- low-level computation

---

## Open questions

1. Should Entity and Claim be the foundation, with Action as a derived concept?
2. Should transformations always produce claims?
3. How should context affect execution decisions?
4. What is the smallest executable subset of this model?

---

## Conclusion

Before syntax design, Aytham needs a minimal executable semantic prototype to test whether these abstractions provide practical value.