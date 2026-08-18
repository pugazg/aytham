# Aytham Roadmap

Status: **research-first roadmap**

The project should not skip directly from naming to compiler implementation. Each phase has an exit condition.

## Phase 0 — Research foundation

**Goal:** establish the question Aytham is trying to answer.

Work:

- document Tamil programming-language prior art;
- document relevant computational Tamil grammar work;
- map Tolkāppiyam concepts without claiming direct equivalence;
- define evidence labels: SOURCE / INTERPRETATION / AYTHAM DESIGN;
- identify closest non-Tamil programming-language concepts;
- maintain an originality/overlap matrix;
- research name/trademark separately from technical novelty.

Exit criteria:

- no foundational claim depends on keyword translation;
- at least three serious computational hypotheses are defined;
- prior-art gaps are explicitly listed;
- novelty wording remains appropriately provisional.

**Current status: ACTIVE.**

---

## Phase 1 — Paper semantics

**Goal:** demonstrate a useful semantic core without committing to surface syntax.

Prototype on paper/data structures:

1. `peyar` — denotable entity/value;
2. `vinai` — transformation/action/effect;
3. `uri` — qualification/refinement;
4. role relation inspired by `vēṟṟumai`;
5. composition law inspired by `puṇarcci`;
6. resolved contextual meaning (`poruḷ`, terminology still reviewable).

Required examples:

- bank/account transfer;
- validated data pipeline;
- state/protocol transition;
- pure vs effectful transformation;
- at least one Tamil-text/Unicode example.

Exit criteria:

- three examples show a measurable safety/readability/composition benefit;
- comparison with Rust/TypeScript/Python or another relevant language is documented;
- concepts that merely rename existing constructs are removed or narrowed;
- a small-step or equivalent executable semantic description is possible.

---

## Phase 2 — Core language specification 0.1

**Goal:** freeze the first minimal language semantics.

Specify:

- values and literals;
- binding/identity;
- semantic categories;
- role system;
- qualification/refinement system;
- transformations/effects;
- composition;
- branching/choice;
- failure/error;
- scope/lifetime at a minimal level;
- module boundary;
- Unicode/source rules.

Do **not** add classes, macros, generics, async, package management, or metaprogramming unless the semantic core requires them.

Exit criteria:

- every construct has defined semantics;
- invalid programs have defined rejection reasons;
- examples can be evaluated manually from the spec;
- terminology has source/design provenance notes.

---

## Phase 3 — Surface syntax exploration

**Goal:** find syntax that naturally expresses the semantic model.

Build multiple competing syntaxes rather than adopting the first attractive Tamil keyword set.

Explore:

- Tamil-first syntax;
- minimal-symbol syntax;
- bilingual/alias surface;
- role markers;
- explicit vs inferred uri;
- expression-oriented layout;
- block delimiters vs indentation;
- ASCII accessibility where necessary.

Usability tests:

- Tamil-speaking beginner;
- experienced Tamil-speaking programmer;
- programmer unfamiliar with Tamil reading code through tooling/translation views;
- mixed-team library interop.

Exit criteria:

- syntax makes role/qualification/composition semantics clearer rather than hiding them;
- Unicode input burden is understood;
- formatting/parser ambiguities are resolved;
- syntax 0.1 is documented with grammar.

---

## Phase 4 — Reference interpreter

**Goal:** executable correctness model.

Priorities:

- simple implementation;
- exhaustive tests;
- readable diagnostics;
- semantic trace mode;
- no optimization pressure.

Candidate implementation languages should be compared before selection.

The interpreter should expose internal reasoning useful for research, for example:

```text
form → category → role → qualification → effect → composition → result
```

Exit criteria:

- specification examples execute;
- invalid examples fail for the specified reason;
- Unicode tests are comprehensive;
- role/refinement/composition diagnostics are demonstrably useful.

---

## Phase 5 — Formalization and semantic audit

**Goal:** ensure the model is coherent before optimizing it.

Work may include:

- formal grammar;
- type/role judgments;
- effect rules;
- composition laws;
- soundness properties where applicable;
- property-based tests;
- fuzzing;
- independent design review.

Exit criteria:

- major semantic contradictions resolved;
- known unsound/unsafe areas documented;
- behaviour is not defined only by the reference interpreter implementation.

---

## Phase 6 — Compiler/backend prototype

**Goal:** compile Aytham programs while preserving reference semantics.

Evaluate backends:

- WebAssembly;
- LLVM;
- C transpilation;
- custom bytecode/VM;
- another IR.

Selection criteria:

- implementation complexity;
- debugging/source maps;
- FFI/interoperability;
- portability;
- optimization;
- runtime requirements.

Exit criteria:

- compiled output matches interpreter semantics on conformance tests;
- errors remain source-level and meaning-oriented;
- backend details do not leak into language semantics unnecessarily.

---

## Phase 7 — Interoperability

**Goal:** make Aytham practically usable without abandoning its model.

Targets may include:

- C ABI;
- WebAssembly host APIs;
- JSON;
- HTTP;
- filesystem;
- environment/process;
- database clients;
- calling or wrapping existing libraries.

Research question:

> How are Aytham roles, uri constraints, and effects represented when crossing into languages that do not understand them?

Exit criteria:

- safe FFI boundary rules;
- explicit loss-of-guarantee diagnostics where needed;
- usable real-world example application.

---

## Phase 8 — Tooling

**Goal:** make semantic information visible to programmers.

Build:

- formatter;
- syntax highlighting;
- LSP;
- hover showing category/role/uri/effect;
- semantic diagnostics;
- rename/refactor;
- test runner;
- documentation generator;
- REPL/playground.

Aytham's tooling should make the new semantic model easier to understand than reading the formal specification.

---

## Phase 9 — Standard library

**Goal:** demonstrate the model through disciplined APIs.

Initial library candidates:

- text/Unicode;
- collections;
- result/error;
- filesystem;
- HTTP;
- time;
- JSON;
- testing.

The standard library should be the strongest demonstration of role-safe, qualification-aware, effect-explicit API design.

---

## Phase 10 — Ecosystem and release

Only after the language core is stable:

- package format;
- dependency resolution;
- registry strategy;
- semantic versioning policy;
- compatibility policy;
- language governance;
- security process;
- release channels;
- documentation/tutorials;
- benchmark suite.

---

# Near-term work queue

1. Deepen prior-art research.
2. Build a concept-by-concept source notebook for வேற்றுமை, பெயர், வினை, இடை, உரி, புணர்ச்சி.
3. Design the role-safe transfer model without syntax.
4. Compare it against Rust newtypes, TypeScript branded types, named arguments, refinement types, and capability/effect systems.
5. Design a second example around validated data flow.
6. Decide whether `poruḷ` is the right Aytham term for resolved computational meaning.
7. Only after these: draft `SPEC_0_1.md`.
