# Experiment 006 — Context and Authority Model v0.1

## Objective

Refine the Aytham semantic core after Experiment 005 exposed two missing dimensions:

- Context
- Authority

The experiment asks whether these should become first-class concepts.

---

## Current core under test

```
Entity
Relation
Action
Claim
Evidence
Transformation
Context
Authority
```

---

## 1. Context model

Aytham observations showed that meaning often depends on circumstances.

Context may include:

```
Context
 ├── time
 ├── place
 ├── participants
 ├── situation
 └── constraints
```

Example:

A statement may have different meanings depending on:

- who said it
- when it was said
- where it occurred
- what relationships existed

Context is not metadata attached later; it can influence meaning resolution.

---

## 2. Authority model

Evidence answers:

> What supports this claim?

Authority answers:

> Why should this source be trusted for this claim?

Example:

```
Claim:
    document_is_valid

Evidence:
    document_scan

Authority:
    issuing_organisation
```

These should not be collapsed.

---

## 3. Historical knowledge test

Example:

```
Entity:
    HistoricalEvent

Claim:
    event_date = X

Evidence:
    inscription

Authority:
    primary archaeological source

Confidence:
    established
```

---

## 4. AI decision test

Example:

```
Prediction

Context:
    input conditions

Evidence:
    training data + model output

Authority:
    approved model version

Claim:
    decision recommendation
```

---

## 5. Sangam interpretation test

Example:

```
Observation:
    word/image appears

Context:
    poem + literary situation

Claim:
    possible interpretation

Evidence:
    source text + commentary

Authority:
    commentary tradition
```

The model should preserve the difference between observation and interpretation.

---

## Findings

Preliminary conclusion:

Context and Authority appear to be strong candidates for first-class concepts.

Updated semantic core candidate:

```
Entity
Relation
Action
Claim
Evidence
Authority
Context
Transformation
```

Further testing is required before freezing the model.
