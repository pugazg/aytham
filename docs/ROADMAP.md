# Aytham Roadmap

Status: **research-first / evidence-gated roadmap**

Authoritative reset reference: `docs/AYTHAM_DIRECTION_RESET_2026-08-23.md`

Aytham must not advance merely because another experiment can be described. Each stage has an evidence gate, and planned architecture must never be presented as implemented behaviour.

---

# Current position — 2026-08-23

A full repository review after Experiments 001–034 found:

- the Tamil-source research foundation is strong;
- ActionFrame, semantic roles, claim lineage, checked composition, and meaning-oriented diagnostics remain promising;
- modern programming-language prior-art comparison is incomplete;
- several later prototype documents moved ahead of executable evidence;
- the live implementation is currently only `prototype/validator/aytham_validator.py`, a minimal claim-requirement checker;
- transformation discovery and path search are design-only and are now paused.

No Experiment 035 path-search implementation is authorized during the reset phase.

---

# Phase 0 — Research foundation

**Goal:** establish the question Aytham is trying to answer without forcing Tamil grammatical categories into compiler terminology.

Core work:

- Tamil programming-language prior art;
- direct Tolkāppiyam source study;
- Nannūl and commentary comparison;
- explicit source/commentary/interpretation/Aytham-design provenance;
- complete 27-iyal anti-selection-bias survey;
- modern PL comparison;
- novelty wording discipline.

## Current assessment

**SUBSTANTIALLY COMPLETE as foundation, but modern prior-art depth remains incomplete.**

Strongly retained findings:

- `vinai = function` rejected;
- `idai = generic control flow` rejected;
- `uri = refinement type` reopened;
- `Eccaviyal = compiler inference` withdrawn;
- semantic roles are Aytham design inspired by relation/case research, not copied grammatical cases;
- punarcci inspires boundary-sensitive joining only as a modern design question;
- ActionFrame research is strengthened by Nannūl/commentarial evidence but remains a modern computational invention.

## Remaining Phase-0 gate

Complete focused comparison against:

- semantic roles / thematic roles;
- Fillmore-style case grammar;
- frame semantics;
- knowledge graphs;
- graph rewriting;
- Hoare-style contracts;
- refinement/dependent typing;
- typestate/session/protocol types;
- effect/capability systems;
- provenance-aware programming;
- rule systems / logic programming;
- automated planning;
- proof search / type-directed synthesis.

**Exit condition:** we can state precisely what Aytham changes relative to the closest established techniques and what empirical result would falsify each retained mechanism.

---

# Phase 1 — Semantic Kernel Candidate

**Goal:** define and challenge the smallest useful semantic layer before treating it as language architecture.

Current candidate:

```text
Subject / Value Identity
Relation / Role
Claim
Action / Transformation
Composition Judgment
```

Supporting structures when needed:

```text
Evidence
Context
Authority
Confidence / epistemic status
Provenance / lineage
```

Current Action/Transformation contract under test:

```text
participants
requires
establishes
preserves
invalidates
effects
capabilities
```

## Important boundary

Aytham must not become a universal knowledge ontology.

The semantic layer should enrich ordinary programming rather than requiring simple arithmetic, loops, or algorithms to be represented as large semantic graphs.

## Exit condition

The kernel survives a serious comparison with conventional alternatives and remains compact enough for progressive disclosure.

---

# Phase 1A — Focused prior-art comparison — CURRENT

**Goal:** test the kernel against its nearest neighbours before adding more architecture.

Required output:

For each retained Aytham mechanism record:

```text
problem solved
closest established technique
semantic overlap
Aytham difference
expected benefit
measurement method
falsification condition
```

Priority mechanisms:

1. contextual semantic roles;
2. ActionFrame as programmer-facing operation contract;
3. independent claims attached to value lineage;
4. establish/preserve/invalidate semantics;
5. semantic boundary composition;
6. meaning-oriented diagnostics.

No new path-planning, syntax, compiler, or backend work belongs here.

---

# Phase 1B — Comparative Benchmark 001

**Goal:** obtain executable comparative evidence rather than another conceptual case study.

## Benchmark

Verified sensitive-email workflow.

Model the same problem in:

1. TypeScript;
2. Rust;
3. Aytham Semantic Kernel Candidate.

Required concerns:

```text
email_syntax_valid
ownership_verified
verification scope
verification freshness
value mutation / lineage invalidation
network_send effect
```

Required invalid cases:

1. raw text sent directly;
2. parsed but ownership-unverified email;
3. evidence belonging to another value;
4. verified value mutated afterward;
5. stale verification;
6. wrong verification scope/account;
7. network effect attempted without permitted capability/context.

Measure:

- invalid states prevented;
- annotation/ceremony;
- wrapper/type proliferation;
- API readability;
- diagnostic quality;
- provenance explanation;
- mutation invalidation clarity;
- progressive-disclosure burden.

## Pass condition

Aytham demonstrates a material advantage on at least one important dimension without imposing disproportionate complexity elsewhere.

## Fail/revise condition

If established approaches express the same guarantees more clearly and economically, revise or remove the Aytham mechanism.

---

# Phase 1C — Critical benchmark review

After Benchmark 001:

- identify which Aytham mechanisms actually earned their complexity;
- remove mechanisms that merely rename existing constructs;
- decide whether provenance/freshness/authority belong in the everyday model or only optional layers;
- decide whether Action and Transformation remain separate concepts;
- decide whether Relation is fundamental or can be represented more simply;
- reassess whether a new programming language is justified.

**Exit condition:** publish a revised Semantic Kernel Candidate with accepted/rejected mechanisms and explicit evidence.

---

# Phase 1D — Minimal executable semantic validator

Only after the benchmark gate, build the smallest real validator corresponding to the surviving kernel.

Expected implementation may include:

```text
prototype/
  validator/
    semantic_graph.py
    validator.py
    explanation.py
  examples/
  tests/
```

But filenames are not commitments until the benchmark determines what the kernel actually needs.

Implementation rules:

- no claim that code exists until it is present on `main`;
- no claim that tests pass unless tests were executed;
- every behaviour must correspond to an accepted semantic rule;
- explanations are outputs, not decorative logging.

Exit criteria:

- executable fixtures exist;
- automated tests exist and run;
- invalid cases fail for the intended semantic reason;
- explanations expose the missing role/claim/evidence/effect accurately;
- ordinary cases remain lightweight.

---

# Phase 2 — Core language specification 0.1

**BLOCKED until Phase 1D succeeds.**

Goal: decide whether the surviving semantic kernel justifies a programming language and integrate it with ordinary computation.

A future specification must account for at least:

- values and literals;
- expressions;
- binding/identity;
- action/transformation invocation;
- role/relationship semantics if retained;
- claims/requirements if retained;
- branching/choice;
- repetition or recursion;
- failure/result semantics;
- state and effects;
- module boundaries;
- Unicode/source rules.

Do not add classes, macros, generics, async, package management, or metaprogramming unless the semantic core requires them.

Exit criteria:

- every construct has defined semantics;
- invalid programs have defined rejection reasons;
- ordinary computation is not forced into knowledge-graph ceremony;
- examples can be evaluated manually from the spec;
- terminology has source/design provenance notes.

---

# Phase 3 — Surface syntax exploration

**BLOCKED.**

Experiment 011 remains historical evidence that:

- role-labelled action notation is the strongest current human-facing candidate;
- graph/triple notation is more suitable as semantic IR/debugging form;
- Tamil sentence-like notation is worth later study but must not control meaning;
- progressive disclosure is mandatory.

Only after Specification 0.1 should competing surfaces be tested:

- Tamil-first;
- bilingual/alias;
- minimal-symbol;
- role-marked;
- block/indentation alternatives;
- ASCII-accessible alternatives where needed.

No Tamil keyword is frozen today.

---

# Phase 4 — Reference interpreter

**BLOCKED.**

Goal: executable correctness model for Specification 0.1.

Priorities:

- clarity;
- conformance tests;
- semantic trace mode;
- meaning-oriented diagnostics;
- no optimization pressure.

---

# Phase 5 — Formalization and semantic audit

**BLOCKED.**

Potential work:

- formal grammar;
- semantic judgments;
- role/relation judgments;
- claim/requirement judgments;
- effect rules;
- composition laws;
- preservation/invalidation rules;
- soundness properties where applicable;
- property-based testing/fuzzing;
- independent design review.

---

# Phase 6 — Compiler/backend prototype

**BLOCKED.**

Only after interpreter semantics stabilize, evaluate:

- WebAssembly;
- LLVM;
- C transpilation;
- custom bytecode/VM;
- another IR/backend.

Backend choice must not redefine source semantics.

---

# Phase 7 — Interoperability

**BLOCKED.**

Potential targets:

- C ABI;
- WebAssembly host APIs;
- JSON/HTTP;
- filesystem/process;
- database clients;
- foreign libraries.

Critical research question:

> How are semantic roles, claims, guarantees, and effects represented when crossing into languages that do not understand them?

---

# Phase 8 — Tooling

**BLOCKED.**

Potential tooling:

- formatter;
- syntax highlighting;
- LSP;
- semantic hover;
- provenance trace;
- meaning-oriented diagnostics;
- rename/refactor;
- test runner;
- docs generator;
- REPL/playground.

---

# Phase 9 — Standard library

**BLOCKED.**

Only after the language core stabilizes.

Potential areas:

- Unicode/text;
- collections;
- result/error;
- filesystem;
- HTTP;
- time;
- JSON;
- testing.

---

# Phase 10 — Ecosystem and release

**BLOCKED.**

Only after semantic, implementation, and interoperability maturity:

- package format;
- dependency resolution;
- registry strategy;
- semantic versioning;
- compatibility policy;
- governance;
- security process;
- release channels;
- tutorials;
- benchmarks.

---

# Paused research branch — Transformation discovery / path planning

Experiments 029–034 remain preserved as exploratory history.

Status: **PAUSED**.

Do not resume until:

1. the semantic kernel is benchmarked;
2. transformation pre/postconditions are formally stable;
3. the state model supports simultaneous facts/relations/capabilities/effects;
4. prior art in planning, rule systems, graph rewriting, proof search, and type-directed synthesis is compared directly;
5. a real programming problem demonstrates that automatic path discovery is needed.

Path planning is not required to prove the core language idea.

---

# Immediate work queue

1. Keep `docs/AYTHAM_DIRECTION_RESET_2026-08-23.md` authoritative for current status.
2. Complete the focused modern prior-art comparison for the Semantic Kernel Candidate.
3. Prepare Benchmark 001 with explicit TypeScript, Rust, and Aytham comparison criteria.
4. Execute the benchmark rather than writing further implementation-plan experiments.
5. Perform a critical benchmark review.
6. Freeze/revise the kernel only after evidence.
7. Then implement the smallest real validator and automated tests.
8. Only afterward reconsider Specification 0.1.

---

# Governing principle

Aytham advances through:

```text
problem
→ prior art
→ hypothesis
→ falsification test
→ comparative evidence
→ decision
→ implementation
```

not:

```text
idea
→ planning document
→ next experiment number
→ assumed maturity
```
