# Aytham Baseline Critical Review v0.1

## Purpose

This document challenges the current Aytham research direction after the first baseline checkpoint.

The goal is not to defend the idea, but to identify:

- what is genuinely promising;
- what already exists in programming-language research;
- what experiments are required before implementation.

---

# 1. Current strongest hypothesis

Aytham is exploring whether a programming model centred around semantic relationships can provide value beyond conventional function/type models.

Current hypothesis:

```
Semantic Object
    |
    +-- identity
    +-- observations
    +-- relations
    +-- claims
    +-- evidence
    +-- transformations
    +-- provenance
```

and:

```
ActionFrame
    |
    +-- participants
    +-- context
    +-- time
    +-- requirements
    +-- effects
    +-- established claims
```

These remain hypotheses.

---

# 2. Challenge: Is ActionFrame actually new?

Existing related areas include:

- semantic roles;
- frame semantics;
- event modelling;
- knowledge graphs;
- workflow systems;
- effect systems;
- actor/event models.

Therefore:

"Actions have participants and effects" is not a novel claim.

The research question is narrower:

> Can an integrated action model become the primary programming abstraction instead of being an external analysis layer?

Required experiment:

Model the same problem in:

1. conventional functions;
2. object-oriented design;
3. typed functional design;
4. Aytham ActionFrame.

Measure:

- clarity;
- error detection;
- explainability;
- maintainability.

---

# 3. Challenge: Is Validated Data Flow new?

Existing related areas:

- refinement types;
- typestate;
- dependent types;
- proof-carrying data;
- provenance systems.

Therefore:

"A value can carry verified properties" is not new.

The Aytham research question:

> Can evidence, provenance, validity and transformations become a unified everyday programming model rather than advanced specialist features?

---

# 4. Challenge: Tamil inspiration must affect behaviour

Rejected approach:

```
function -> வினை
class -> வகுப்பு
if -> எனில்
```

This only changes vocabulary.

Accepted research direction:

Tamil concepts should influence:

- what programmers express;
- what the compiler checks;
- what diagnostics explain;
- how meaning is represented.

---

# 5. Current promising differentiators

## 5.1 Meaning-oriented diagnostics

Instead of:

```
Type mismatch
```

possible Aytham goal:

```
This action requires ownership_verified.
The available evidence only proves email_format_valid.
Suggested missing transformation: VerifyOwnership.
```

This must be tested.

## 5.2 Evidence-aware computation

Claims should preserve:

- origin;
- evidence;
- authority;
- confidence;
- validity.

## 5.3 Context-aware interpretation

Sangam research may help test how context influences meaning, but no direct programming feature should be assumed.

---

# 6. Immediate research goals

Before syntax or compiler work:

1. Complete comparison with existing programming concepts.
2. Define Semantic Object model formally.
3. Define ActionFrame formally.
4. Create small examples where Aytham provides measurable benefit.
5. Decide whether a language is justified.

---

# 7. Success criteria

Aytham should proceed to implementation only if it demonstrates at least one capability that is difficult or unnatural in mainstream languages.

Possible examples:

- better explanation of missing evidence;
- safer composition of validated workflows;
- clearer modelling of context-dependent operations.

---

# 8. Current conclusion

Aytham is currently a programming-language research project, not yet a programming language project.

The next milestone is not a compiler.

The next milestone is:

**Aytham Semantic Model v1.0**

A precise model that can be challenged, compared and tested.
