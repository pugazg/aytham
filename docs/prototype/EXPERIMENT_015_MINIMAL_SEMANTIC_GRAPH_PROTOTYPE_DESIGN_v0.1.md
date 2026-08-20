# Experiment 015 — Minimal Semantic Graph Prototype Design v0.1

## Status

Design experiment only. This is not yet an implementation.

The purpose is to define the smallest executable prototype that can validate whether the Aytham semantic graph provides useful behaviour.

---

# Objective

Create a minimal engine capable of:

1. loading a canonical semantic graph;
2. validating semantic requirements;
3. tracking transformations;
4. generating explanations.

The prototype should prove the semantic model before any full programming language implementation.

---

# Prototype boundary

The prototype is NOT:

- a compiler;
- a programming language runtime;
- a replacement for existing languages.

It is a semantic execution and reasoning engine.

---

# Minimal input model

The prototype consumes:

```json
{
  "entities": [],
  "relations": [],
  "actions": [],
  "claims": [],
  "transformations": []
}
```

Optional layers:

- evidence;
- authority;
- context;
- confidence.

---

# Core operations

## 1. Load graph

The engine creates an internal representation of:

- entities;
- relationships;
- available claims;
- possible transformations.

---

## 2. Requirement checking

Example:

Action:

```
SendSensitiveMessage
```

Requires:

```
email : ownership_verified
```

Available:

```
email : syntax_valid
```

The engine should not return only failure.

Expected explanation:

```
Cannot execute SendSensitiveMessage.

Missing claim:
    email : ownership_verified

Available:
    email : syntax_valid

Possible bridge:
    VerifyOwnership transformation
```

---

## 3. Transformation execution

A transformation changes semantic state.

Example:

```
RawEmail
   |
   | ParseEmail
   v
EmailAddress
```

creates:

```
email : syntax_valid
```

A later transformation:

```
VerifyOwnership
```
creates:

```
email : ownership_verified
```

---

# Prototype test cases

## Test A — Banking

Input:

```
TransferMoney
```

Requirements:

```
source : debit_authorized
amount : positive
```

Expected:

Successful execution creates:

```
transfer : completed
```

---

## Test B — COBOL-style batch processing

Input:

```
CustomerRecord
```

Flow:

```
RawRecord
    |
ValidateRecord
    |
ValidatedRecord
    |
UpdateDatabase
```

Expected:

The graph preserves validation history and update lineage.

---

## Test C — Sangam interpretation

Input:

```
Observation
```

Flow:

```
TextObservation
      |
Interpretation
      |
Claim
```

Expected:

The engine preserves:

- evidence;
- confidence;
- alternative interpretations.

---

# Prototype architecture

```
Semantic Graph Loader
          |
          v
Semantic State Store
          |
   +------+------+
   |             |
Validator   Explanation Engine
   |
Transformation Resolver
```

---

# Success criteria

The prototype succeeds if it can answer:

1. Why did this action succeed?
2. Why did this action fail?
3. What claim is missing?
4. Which transformation can establish it?
5. Where did this knowledge originate?

---

# Failure criteria

The prototype fails if:

- it becomes only a graph database wrapper;
- explanations are no better than normal error messages;
- semantic objects add complexity without useful reasoning;
- every simple operation requires excessive metadata.

---

# Next step

After this design is validated, implement a small proof-of-concept engine before exploring Aytham language syntax.
