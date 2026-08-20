# Aytham Research Baseline v0.1

## Status

Research checkpoint. Aytham remains **research / pre-specification**.

This document records what has been learned so far, what is still uncertain, and what the project should attempt to prove.

---

## 1. Core Question

Aytham is not an attempt to translate existing programming languages into Tamil keywords.

The research question is:

> Can Tamil grammatical and semantic traditions inspire computational abstractions that make programs clearer, safer, or easier to reason about?

Any proposed idea must satisfy:

1. Historical honesty — what does the source actually describe?
2. Computational usefulness — does the abstraction improve programming?

---

## 2. Source Hierarchy

Aytham research uses different sources for different purposes.

```
Tolkāppiyam
    ↓
grammatical theory

Nannūl + commentaries
    ↓
comparative grammatical interpretation

Sangam literature
    ↓
attested usage and contextual testing

Modern programming-language research
    ↓
validation against existing ideas
```

---

## 3. Current Research Direction

The strongest hypothesis is not Tamil syntax, but a semantic model.

Current experimental model:

```
Semantic Object

Identity
Observations
Relations
Claims
Evidence
Transformations
Provenance
```

---

## 4. Action-Centred Model

Aytham is exploring whether computation can be represented through action frames rather than only function signatures.

```
Action
 |
 +-- participants
 +-- roles
 +-- context
 +-- time
 +-- requirements
 +-- established claims
 +-- effects
 +-- outputs
```

This is inspired by grammatical relation study but is a modern computational invention.

---

## 5. Validated Data Flow

Experiment 002 established a key direction:

Values should be able to carry:

- what they are;
- what has been established about them;
- who/what established it;
- evidence;
- validity scope.

A fact is not only true/false.

Possible states:

```
explicit
inferred
derived
contextual
unknown
contested
```

---

## 6. Tamil Concepts Under Investigation

### வேற்றுமை

Research direction:

First-class semantic roles and relationships.

Not a direct conversion of grammatical cases into programming.

### வினை

Research direction:

Actions, transformations and their consequences.

`vinai = function` is rejected as too simplistic.

### இடை

Research direction:

Semantic mediation and connection.

Not a Tamil replacement for control-flow keywords.

### உரி

Status: open research.

`uri = refinement type` is rejected for now.

Current neutral term:

qualification claim.

### புணர்ச்சி

Research direction:

Boundary-sensitive checked composition.

Not merely syntax joining.

---

## 7. Lessons From Sangam Corpus

The Sangam corpus introduced important ideas:

### Provenance

A claim must preserve its origin.

### Uncertainty

Unknown, missing, inferred and explicit information are different states.

### Context

Meaning may require relationships and surrounding information.

### Authority

Source, interpretation and hypothesis must remain separate.

---

## 8. Rejected Shortcuts

Aytham will not become:

```
if → Tamil word
function → வினை
class → தமிழ் equivalent
```

Nor will it assume:

```
திணை = class
உரி = refinement type
எச்சம் = compiler inference
```

without stronger evidence.

---

## 9. Current Goal

The goal is not yet to build a compiler.

The immediate goal is:

> Define and test an Aytham semantic model that provides measurable advantages over conventional programming models.

Success requires demonstrating at least one practical advantage.

---

## 10. Next Research Activities

1. Complete source-grounded Sollatikāram study.
2. Continue Nannūl cross-edition comparison.
3. Continue Sangam context and action analysis.
4. Compare ActionFrame with semantic roles, frame semantics, knowledge graphs, graph rewriting and type systems.
5. Create formal semantic experiments before designing syntax.

---

## Research Discipline

Every future feature must be separated into:

```
Historical source
        ↓
Interpretation
        ↓
Modern programming prior art
        ↓
Aytham design invention
```
