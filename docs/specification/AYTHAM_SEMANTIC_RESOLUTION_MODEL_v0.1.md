# Aytham Semantic Resolution Model v0.1

## Status

Research specification only. This document defines how a future Aytham surface notation may resolve into canonical semantic meaning.

It does not define final syntax.

---

## Objective

Experiment 011 established that a human-facing notation should not be the final authority of meaning. Aytham requires a resolution layer:

```
Surface notation
      |
      v
Semantic resolver
      |
      v
Canonical semantic graph
```

The graph becomes the stable representation for validation, explanation, tooling and execution.

---

# 1. Canonical semantic graph

A resolved Aytham program consists of nodes and relations.

Primary nodes:

- Entity
- Action
- Claim
- Transformation

Supporting nodes:

- Evidence
- Context
- Authority

Relations:

- participates-in
- requires
- establishes
- transforms
- supports
- derived-from
- constrained-by

---

# 2. Resolution example

Surface notation:

```
Transfer {
    source AccountA
    destination AccountB
    amount Money100

    requires
        AccountA : debit_authorized
}
```

Canonical graph:

```
AccountA
   |
   | source-of
   v
Transfer#1
   ^
   | destination-of
   |
AccountB

Money100
   |
   | amount-of
   v
Transfer#1

Transfer#1
   |
   | requires
   v
Claim(debit_authorized(AccountA))
```

---

# 3. Identity rules

Every semantic object requires a stable identity after resolution.

Example:

```
Transfer#1
Claim#42
Transformation#7
```

Identity must survive:

- diagnostics
- serialization
- execution
- lineage tracking

---

# 4. Explicit and inferred meaning

The resolver must preserve origin.

Allowed relation origin:

```
explicit
inferred
derived
contextual
```

The resolver must never silently convert an inference into an explicit fact.

---

# 5. Requirement matching

Action execution requires semantic satisfaction.

Example:

Required:

```
Customer : ownership_verified
```

Available:

```
Customer : email_valid
```

Result:

```
Action blocked.

Missing transformation:
VerifyOwnership
```

---

# 6. Ambiguity handling

If multiple valid interpretations exist:

```
Resolution incomplete.

Candidates:
    Interpretation A
    Interpretation B

Required:
    additional context or explicit selection
```

Aytham must prefer explicit resolution over hidden guessing.

---

# 7. Explanation queries

The semantic graph should support questions:

```
why?
```

Why did an action fail?

```
how-known?
```

How was a claim established?

```
what-missing?
```

Which transformation or evidence is required?

---

# 8. Current architectural conclusion

Aytham should separate:

```
Tamil-oriented or other surface notation
              |
              v
      Semantic resolution
              |
              v
       Canonical meaning graph
              |
              v
 Validation + explanation + runtime
```

The semantic graph is currently the most important research artifact. Syntax remains secondary until this model is proven useful.
