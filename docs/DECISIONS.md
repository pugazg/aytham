# Aytham Decision Log

This file records project-level decisions separately from research hypotheses. A decision may be revised; revisions should append a new entry rather than silently rewriting history.

## D-0001 — Repository is the canonical project record

**Date:** 2026-08-18  
**Status:** Accepted

`pugazg/aytham` is the canonical repository for Aytham research, design, specification, implementation, experiments, and project history.

---

## D-0002 — Research-first development

**Date:** 2026-08-18  
**Status:** Accepted

Do not start with a compiler, keyword list, or finalized syntax.

Order of work:

```text
prior art
→ source/concept study
→ falsifiable semantic hypotheses
→ paper experiments
→ specification
→ syntax
→ reference implementation
→ compiler/tooling
```

Reason: the project's value depends on discovering a genuine computational model, not producing another Tamil surface over conventional semantics.

---

## D-0003 — Tolkāppiyam is inspiration, not retroactive computer science

**Date:** 2026-08-18  
**Status:** Accepted

Aytham will never claim that Tolkāppiyam literally specifies modern compiler architecture, type systems, effect systems, or graph computation.

All documents distinguish SOURCE, INTERPRETATION, and AYTHAM DESIGN.

---

## D-0004 — `எழுத்து → சொல் → பொருள்` is a research frame, not yet the architecture

**Date:** 2026-08-18  
**Status:** Accepted

The three-part frame is useful for organizing research, but simply mapping it to lexer → parser → semantics would be too shallow to justify Aytham.

The project will look for behavioural consequences instead.

---

## D-0005 — First high-priority concepts

**Date:** 2026-08-18  
**Status:** Accepted for experimentation, not language specification

Research priority:

1. வேற்றுமை-inspired semantic roles;
2. உரி-inspired refinements/qualifications;
3. வினை as explicit transformation/effect;
4. புணர்ச்சி-inspired checked composition;
5. பெயர் / வினை / இடை / உரி as possible semantic categories.

These concepts must survive comparison against established programming-language techniques.

---

## D-0006 — Semantic relation graph is the first major hypothesis

**Date:** 2026-08-18  
**Status:** Experimental

Aytham will test a model in which resolved program meaning is a graph containing:

- peyar/entity nodes;
- vinai/action nodes;
- role relationships;
- uri constraints;
- composition/control relationships;
- effects/capabilities.

This is not accepted language architecture yet. Experiment 001 defines pass/fail criteria.

---

## D-0007 — No `first/unique` originality claim yet

**Date:** 2026-08-18  
**Status:** Accepted

Permitted wording:

> Aytham explores a programming-language model inspired by Tamil grammatical thought.

Not yet permitted:

> Aytham is the first Tolkāppiyam-based programming language.

Reason: prior-art review is incomplete.

---

## D-0008 — The name Aytham / ஆய்தம் remains provisional from an IP perspective

**Date:** 2026-08-18  
**Status:** Accepted

The project may use Aytham as its working identity. This does not constitute trademark clearance or legal advice.

A separate name/trademark review is required before commercial branding or filing.

---

## D-0009 — Concrete syntax remains deliberately undecided

**Date:** 2026-08-18  
**Status:** Accepted

Do not treat examples in research documents as final Aytham syntax.

Still open:

- Tamil-only vs bilingual surface;
- keywords;
- punctuation;
- blocks/indentation;
- file extension;
- role-marker syntax;
- type/refinement syntax.

---

## D-0010 — No backend selected

**Date:** 2026-08-18  
**Status:** Accepted

LLVM, WebAssembly, C transpilation, custom bytecode/VM, or another backend must not drive early semantics.

The first executable model should optimize for semantic clarity and conformance testing.

---

## D-0011 — Validated data flow is the second semantic stress test

**Date:** 2026-08-18  
**Status:** Accepted for experimentation

Experiment 002 tests whether the model generalizes beyond semantic roles such as transfer source/destination.

The test pipeline is:

```text
raw text
→ ParseEmail
→ syntactically valid email value
→ VerifyOwnership
→ ownership-verified email value
→ SendSensitiveMessage
```

Aytham must demonstrate a benefit over branded/newtypes, refinement types, typestate, and related established techniques before this model can enter the specification.

---

## D-0012 — `uri` may carry evidence and provenance

**Date:** 2026-08-18  
**Status:** Experimental

For Experiment 002, `uri` is tested as more than a Boolean property or nominal tag.

A qualification may carry:

- the claim being established;
- the exact semantic subject/value;
- the vinai that established it;
- evidence or attestation where applicable;
- scope/context;
- freshness/validity.

This is a hypothesis, not accepted language semantics. It must remain simple enough for ordinary programming and must be compared with proof/refinement/provenance systems.

---

## D-0013 — Semantic facts attach to value lineage, not variable names

**Date:** 2026-08-18  
**Status:** Experimental

A fact established about one value must not silently transfer to a different value merely because the same variable/binding name is reused.

Experiment 002 therefore tests a versioned/lineage model in which a meaning-changing vinai can:

- establish qualifications;
- preserve explicitly compatible qualifications;
- invalidate qualifications whose evidence no longer applies.

This must be compared against SSA, immutability, typestate, refinement typing, and proof-carrying approaches before adoption.
