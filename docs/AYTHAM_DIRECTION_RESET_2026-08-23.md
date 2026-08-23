# Aytham Direction Reset — 2026-08-23

Status: **AUTHORITATIVE CURRENT-STATE REVIEW**

This document records the result of a full review of the live `main` repository after Experiments 001–034.

Its purpose is to prevent research hypotheses, planned implementation, and actually implemented behaviour from being conflated.

Where an older experiment document describes a future or proposed architecture as though it were already implemented, this document has precedence for current project status. Historical experiment documents are retained as research history and should not be silently rewritten to erase the path taken.

---

## 1. Current verdict

Aytham remains a **programming-language research project** exploring whether Tamil grammatical thought can inspire a useful semantic programming model.

The central research direction remains viable.

The strongest surviving hypothesis is not a Tamil keyword layer and not a generic knowledge graph. It is a programmer-facing semantic contract model in which actions and transformations can expose:

- participant relations / semantic roles;
- required claims;
- established claims;
- preserved claims;
- invalidated claims;
- effects and capabilities;
- evidence/provenance where relevant;
- checked composition boundaries;
- meaning-oriented explanations.

Individual ingredients have extensive prior art. Aytham must prove value through integration, usability, diagnostics, and programming semantics rather than ingredient novelty.

---

## 2. What is retained

The following directions remain active research assets.

### 2.1 Source discipline

Retain the strict separation:

```text
SOURCE
COMMENTARY
MODERN SCHOLARSHIP
INTERPRETATION
AYTHAM DESIGN
```

Tolkāppiyam and Nannūl inspire questions and structural perspectives. They are not retroactively described as containing modern programming-language theory.

### 2.2 Action-centred relation frames

Retain the ActionFrame research direction:

```text
ActionFrame {
    action
    participant_relations
    circumstantial_relations
    temporal_context
    requires_claims
    establishes_claims
    preserves_claims
    invalidates_claims
    effects
    capabilities
    output_relations
}
```

The exact structure remains experimental.

### 2.3 Semantic role relations

Retain the distinction between:

```text
what a value is
```

and:

```text
what role that value plays in this action/context
```

A role is primarily a relation, not a permanent wrapper identity.

### 2.4 Claim lineage and provenance

Retain the principle that established facts attach to the relevant semantic value/lineage rather than to a mutable variable name.

Provenance, scope, freshness, and authority remain optional supporting dimensions that should be introduced only where the semantics require them.

### 2.5 Boundary-sensitive checked composition

Retain the research question inspired by rule-governed joining:

> When two individually meaningful computational forms meet, what semantic boundary determines whether they compose directly, require adaptation, are ambiguous, or must be rejected?

Possible outcomes remain experimental:

```text
DIRECT
TRANSFORMED
REQUIRES_ADAPTOR
AMBIGUOUS
REJECTED
```

### 2.6 Meaning-oriented diagnostics

Retain explanation as a first-class evaluation criterion.

Aytham should aim to explain missing meaning in domain terms, for example:

```text
SendSensitiveMessage cannot execute.

Required:
    ownership_verified

Established:
    email_syntax_valid

The ownership requirement has not been established
for this value lineage.
```

Tooling may suggest a registered transformation only when such a transformation is actually known and semantically applicable.

### 2.7 Progressive disclosure

Retain the requirement that simple computation must remain simple.

Evidence, context, authority, confidence, detailed lineage, and other advanced semantic information must not become mandatory boilerplate when they are irrelevant.

---

## 3. What is paused

The following directions are paused until the semantic kernel survives stronger comparative tests.

### 3.1 Generic SemanticObject expansion

Do not continue expanding Aytham into a universal ontology containing an ever-growing collection of peer primitives.

The current working boundary is smaller:

```text
core candidates:
    Subject / Value Identity
    Relation / Role
    Claim
    Action / Transformation

supporting structures:
    Evidence
    Context
    Authority
    Confidence / epistemic status
    Provenance / lineage

cross-cutting rule:
    Composition judgment
```

Even this boundary remains subject to falsification.

### 3.2 Path-finding / planning engine

Experiments 029–034 explored transformation discovery and semantic path search.

That work is now **PAUSED**.

Reason:

A transformation cannot generally be modeled accurately as only:

```text
Claim A -> Transformation -> Claim B
```

A more realistic semantic state contains multiple simultaneous claims, relations, capabilities, contexts, and effects.

Before path search resumes, Aytham must compare the problem directly with:

- automated planning / STRIPS-like operators;
- rule systems;
- logic programming;
- labelled transition systems;
- graph rewriting;
- workflow/Petri-net models;
- proof search;
- type-directed synthesis.

Path search is an optional later capability. It is not currently part of the proof that Aytham deserves to become a programming language.

### 3.3 New syntax work

Do not freeze Tamil keywords, Tamil case-like syntax, block grammar, punctuation, or file extensions.

Experiment 011 remains useful as a notation study only.

### 3.4 Compiler/backend work

No parser, compiler backend, VM, LLVM, WebAssembly, C transpilation, package manager, or optimization work should begin during this reset phase.

---

## 4. Implementation truth

The live implementation currently consists of:

```text
prototype/validator/aytham_validator.py
```

This is a small research prototype that:

- indexes claims by `(subject, property)`;
- finds an action by ID;
- checks whether required claims exist with matching values;
- returns a basic success/failure explanation.

It does **not** currently implement:

- `SemanticGraph` classes;
- graph loading/normalisation;
- relation validation;
- confidence ordering;
- evidence or authority checks;
- provenance tracing;
- lineage preservation/invalidation;
- transformation execution;
- transformation discovery;
- semantic path search;
- example JSON fixtures;
- automated tests.

Documents that proposed these components remain design history, not evidence that they exist.

No future document may say that an implementation exists or that tests pass unless the corresponding live files and executed test evidence exist.

---

## 5. Problem discovered in Experiments 020–034

The repository accumulated a sequence of increasingly detailed planning documents while implementation remained minimal.

This created three risks:

1. **documentation drift** — planned work was described using implementation language;
2. **false maturity** — experiment numbers suggested progress not matched by executable evidence;
3. **premature architecture** — the project moved from semantic hypothesis toward validator/planner architecture before completing its own falsification criteria.

Experiment numbering will no longer be used as a proxy for maturity.

The next work must produce evidence, not merely another numbered planning document.

---

## 6. Programming-language boundary

Aytham must avoid becoming only a semantic knowledge/workflow framework.

A future programming language still needs a coherent account of ordinary computation, including at minimum:

- values and literals;
- expressions;
- binding/identity;
- action/transformation invocation;
- choice/branching;
- repetition or recursion;
- failure/results;
- state and effects;
- module boundaries.

The semantic contract layer should enrich ordinary computation rather than require every arithmetic or algorithmic expression to become a knowledge graph.

Working architectural question:

```text
             Aytham program
                  |
        +---------+---------+
        |                   |
 ordinary computation   semantic contract layer
        |                   |
 values               roles / relations
 expressions          requires
 choices              establishes
 transformations      preserves
 etc.                 invalidates
                      evidence/provenance when needed
                      effects/capabilities
        |                   |
        +---------+---------+
                  |
          resolved semantics
                  |
             validation
                  |
             execution
```

This is a research hypothesis, not accepted language architecture.

---

## 7. Semantic Kernel Candidate

The next comparative work should use the following intentionally small kernel candidate.

### Subject / Value Identity

Represents the semantic subject to which relations and claims apply.

### Relation / Role

Represents contextual semantic participation such as:

```text
AccountA --source-of--> Transfer
AccountB --destination-of--> Transfer
```

### Claim

Represents a statement established about a subject/relation/action.

Minimum dimensions under test:

```text
subject
property
value
status
```

Optional when required:

```text
evidence
provenance
authority
scope
validity/freshness
```

### Action / Transformation

Represents an operation with semantic consequences.

Minimum contract under test:

```text
participants
requires
establishes
preserves
invalidates
effects
capabilities
```

### Composition Judgment

Determines whether produced semantic state satisfies the requirements of a following action and explains any gap.

This kernel is **CANDIDATE / EXPERIMENTAL**, not Specification 0.1.

---

## 8. Mandatory prior-art work before further architecture

The next prior-art pass must go beyond the earlier type-system comparison and directly examine:

- semantic roles / thematic roles;
- Fillmore-style case grammar;
- frame semantics and frame-based representations;
- knowledge graphs;
- graph rewriting;
- Hoare-style pre/postcondition systems;
- refinement and dependent typing;
- typestate and protocol/session types;
- effect and capability systems;
- provenance-aware programming;
- rule engines and logic programming;
- automated planning;
- type-directed synthesis / proof search.

The question is not whether these fields resemble Aytham. They do.

The question is what, if anything, Aytham's integrated programmer-facing model does better.

---

## 9. Next evidence gate — comparative benchmark

Before expanding the validator or planner, perform one serious comparative benchmark.

### Benchmark 001 — Verified sensitive-email workflow

Model the same workflow in:

1. TypeScript;
2. Rust;
3. Aytham Semantic Kernel Candidate.

Required semantic concerns:

```text
email_syntax_valid
ownership_verified
verification scope
verification freshness
value mutation / lineage invalidation
network_send effect
```

Required invalid cases:

1. raw text passed directly to send;
2. syntax-valid but ownership-unverified email;
3. verification for another value;
4. verified value mutated after verification;
5. stale verification;
6. verification valid for the wrong account/scope;
7. network effect attempted from a context that does not permit it.

Measure and document:

- invalid states prevented;
- annotation/ceremony required;
- wrapper/type proliferation;
- API readability;
- diagnostic quality;
- provenance explanation;
- mutation invalidation clarity;
- progressive-disclosure burden.

### Pass condition

Aytham may proceed toward a stronger executable semantic kernel if it demonstrates a material advantage on at least one important dimension without creating disproportionate complexity elsewhere.

### Fail/revise condition

If Rust/TypeScript or an established formal technique expresses the same guarantees more clearly and economically, revise or remove the Aytham mechanism rather than defending it through terminology.

---

## 10. Work order from this reset

```text
1. Repository consolidation / status truth       <- CURRENT
2. Focused prior-art comparison
3. Benchmark 001: TypeScript vs Rust vs Aytham
4. Critical benchmark review
5. Freeze or revise Semantic Kernel Candidate
6. Implement the smallest real validator
7. Add executable fixtures and automated tests
8. Reassess whether a language specification is justified
9. Only then resume notation/syntax work
10. Path planning/discovery only if later justified
```

No Experiment 035 path-search implementation is authorized at this stage.

---

## 11. Current maturity

```text
Tamil-source research foundation              STRONG / CONTINUE
Historical/source provenance discipline       STRONG / CONTINUE
Modern PL prior-art comparison                 PARTIAL / DEEPEN
ActionFrame hypothesis                         PROMISING / TEST
Semantic roles                                 PROMISING / TEST
Claim lineage/provenance                       PROMISING / TEST
Boundary-sensitive composition                 PROMISING / TEST
Meaning-oriented diagnostics                   PROMISING / TEST
Canonical semantic graph                       EXPERIMENTAL
Python validator                               MINIMAL PROTOTYPE
Automated test suite                           NOT IMPLEMENTED
Transformation discovery                       DESIGN ONLY / PAUSED
Semantic path search                           DESIGN ONLY / PAUSED
Aytham language specification                  NOT READY
Surface syntax                                 NOT FROZEN
Compiler/runtime                               NOT STARTED
```

---

## 12. Governing rule after the reset

For every major next step ask, in this order:

```text
What real programming problem is being solved?
What established technique already addresses it?
What exactly does Aytham change?
How will that difference be tested?
What result would cause us to reject the idea?
Only then: what should we implement?
```

The project should advance by **evidence gates**, not by experiment-number accumulation.
