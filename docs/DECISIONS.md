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
**Status:** Experimental; terminology reopened by D-0015

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

---

## D-0014 — Experiment 002 survives comparison only in narrowed form

**Date:** 2026-08-18  
**Status:** Experimental direction accepted

Comparative research found strong prior art for every major individual ingredient:

- refinements;
- typestate;
- dependent proofs;
- proof-carrying evidence;
- effect typing;
- SSA/value lineage;
- provenance;
- language-integrated provenance.

Therefore Aytham must not claim ingredient novelty.

The surviving hypothesis is the **unified programmer-facing semantic graph** in which role, qualification/evidence, provenance, lineage, effects, and composition can participate in one resolved meaning model and one diagnostic/tooling model.

This combination must still prove usefulness and coherence.

---

## D-0015 — `uri = refinement type` is reopened

**Date:** 2026-08-18  
**Status:** Reopened / research required

Direct reading of the opening of Tolkāppiyam உரியியல் shows a broader lexical-semantic and contextual treatment than the convenient modern label “qualifier” suggests.

Therefore:

- do not define historical `uri` as a refinement predicate;
- use the neutral phrase **qualification claim** in paper semantics;
- `uri` may remain a research alias;
- do not freeze `uri` as a keyword or formal type-system construct before source/commentary/Nannūl comparison.

Experiment 002's evidence-backed qualification idea remains valid as **AYTHAM DESIGN** even if the final Tamil term changes.

---

## D-0016 — `idai` is researched as mediation, not generic control flow

**Date:** 2026-08-18  
**Status:** Experimental direction

The opening of இடையியல் emphasizes operation with பெயர்/வினை and relational/contextual functions.

Aytham therefore should not use `idai` as a decorative name for `if`, pipes, semicolons, or arbitrary control operators.

Research `idai` instead as a possible model of **semantic mediation/connection** whose relation contributes meaning between surrounding forms.

---

## D-0017 — Early Eccaviyal → inference mapping is withdrawn

**Date:** 2026-08-18  
**Status:** Withdrawn pending source study

The structured corpus gloss “Ellipsis” is too narrow to justify mapping the whole எச்சவியல் to implicit arguments or compiler inference. Direct source reading shows the iyal opens with broader lexical classification and contains diverse residual word-grammar material.

No Eccaviyal-inspired inference feature is active until the full iyal and commentary are reviewed.

---

## D-0018 — All 27 Tolkāppiyam iyals must be surveyed before core terminology freezes

**Date:** 2026-08-18  
**Status:** Accepted

Aytham will not select only convenient Tolkāppiyam concepts. The complete 27-iyal survey is part of the research baseline, including explicit HOLD decisions where an analogy would be forced.

This reduces selection bias and prevents core language terminology from being frozen from a partial reading.

---

## D-0019 — Nannūl enters before Sangam literature in the next comparison stage

**Date:** 2026-08-18  
**Status:** Accepted research order

Nannūl should be added next as a **later comparative grammar layer** for categories such as relation/case, word class, joining, residual constructions, and qualification-related treatment.

Sangam literature should then be used primarily as **attested usage/context evidence**, especially before borrowing from பொருளதிகாரம் concepts such as திணை.

Neither later grammar nor literature may be silently back-projected into Tolkāppiyam.

---

## D-0020 — Nannūl citations are edition-aware

**Date:** 2026-08-18  
**Status:** Accepted

The same action-frame rule beginning `செய்பவன் கருவி நிலம் செயல் காலம் / செய்பொருள் ஆறும் தருவது வினையே` is numbered **319** in the Mayilaināthar/U. Vē. Cā. TVA presentation and **320** in other TVA Nannūl presentations.

Aytham must therefore preserve:

- edition/commentary identity;
- nūṟpā incipit;
- number in that edition;
- printed page where available;
- source URL.

Never silently normalize conflicting numbering into one universal Nannūl number.

---

## D-0021 — ActionFrame relations record provenance of interpretation

**Date:** 2026-08-18  
**Status:** Experimental direction

Commentarial study strengthens the distinction between overt surface material and resolved semantic relations.

A provisional Aytham ActionFrame relation should therefore be able to record an origin such as:

```text
explicit
inferred
derived
contextual
```

The goal is not to maximize omission. The goal is that any non-explicit relation remains inspectable, uniquely justified, and diagnosable.

If more than one valid semantic frame can be resolved, compilation should require disambiguation rather than silently guessing.

---

## D-0022 — Punarcci research becomes boundary-transformation research

**Date:** 2026-08-18  
**Status:** Experimental direction

Nannūl and Mayilaināthar's commentary reinforce a boundary-oriented reading: two forms meet under a semantic relation and may remain natural/unchanged or undergo rule-governed modification.

Aytham will therefore test composition outcomes such as:

```text
DIRECT
TRANSFORMED
REQUIRES_ADAPTOR
AMBIGUOUS
REJECTED
```

This is a modern Aytham design inspired by the structural idea of rule-governed joining. It is not a claim that Tamil grammatical punarcci is software composition.

---

## D-0023 — Full-repository direction reset becomes authoritative

**Date:** 2026-08-23  
**Status:** Accepted

After a full review of the live repository, `docs/AYTHAM_DIRECTION_RESET_2026-08-23.md` becomes the authoritative current-state review.

Older experiment documents remain historical research records, but they do not override current implementation truth or current research gates.

If an older document describes proposed work using implementation language, the live repository and the reset document control the current status.

---

## D-0024 — Aytham advances through evidence gates, not experiment numbers

**Date:** 2026-08-23  
**Status:** Accepted

Numbered planning documents are not evidence of maturity.

The governing progression is now:

```text
real programming problem
→ closest prior art
→ falsifiable Aytham hypothesis
→ comparative benchmark
→ critical review
→ decision
→ implementation
```

A new experiment document is justified only when it records new evidence, a genuinely new falsifiable question, or a decision-producing test.

---

## D-0025 — Semantic Kernel Candidate is intentionally small

**Date:** 2026-08-23  
**Status:** Experimental direction

The current kernel candidate is reduced to:

```text
Subject / Value Identity
Relation / Role
Claim
Action / Transformation
Composition Judgment
```

Supporting structures may include:

```text
Evidence
Context
Authority
Confidence / epistemic status
Provenance / lineage
```

These supporting structures are not automatically peer-level language primitives.

The goal is to prevent Aytham from becoming a universal ontology rather than a programming language.

---

## D-0026 — Transformation discovery and semantic path search are paused

**Date:** 2026-08-23  
**Status:** PAUSED

Experiments 029–034 are retained as exploratory design history, but no further path-search implementation is authorized now.

Reasons:

1. A real semantic state contains multiple simultaneous claims, relations, capabilities, contexts, and effects.
2. A simple `Claim A → Transformation → Claim B` graph is insufficient as a general action model.
3. The project has not yet completed direct comparison with automated planning, rule systems, graph rewriting, proof search, workflow models, or type-directed synthesis.
4. Path search is not required to prove the central Aytham language hypothesis.

No Experiment 035 path-search work should proceed until a later explicit decision reopens it.

---

## D-0027 — Live implementation truth must be stated explicitly

**Date:** 2026-08-23  
**Status:** Accepted

The live implementation currently consists only of:

```text
prototype/validator/aytham_validator.py
```

It performs basic claim-requirement matching and returns simple explanations.

It does not currently implement the broader `SemanticGraph`, transformation, provenance, lineage, path-search, fixture, or automated-test architecture described in later planning documents.

Future project reporting must distinguish:

```text
DESIGNED
IMPLEMENTED
EXECUTED/TESTED
```

No test may be reported as passing unless it was actually run.

---

## D-0028 — Comparative Benchmark 001 precedes further validator architecture

**Date:** 2026-08-23  
**Status:** Accepted

The next major evidence gate is a verified sensitive-email workflow modeled in:

1. TypeScript;
2. Rust;
3. the Aytham Semantic Kernel Candidate.

The comparison must include:

- syntax validity;
- ownership verification;
- verification scope;
- verification freshness;
- value mutation/lineage invalidation;
- network-send effects/capabilities;
- deliberately invalid cases.

Evaluate:

- invalid states prevented;
- annotation burden;
- wrapper/type proliferation;
- API readability;
- diagnostic quality;
- provenance explanation;
- mutation invalidation clarity;
- progressive-disclosure burden.

Further validator architecture is justified only after this benchmark and critical review identify which Aytham mechanisms actually earn their complexity.

---

## D-0029 — Aytham must remain a programming-language research project, not silently become a workflow/knowledge engine

**Date:** 2026-08-23  
**Status:** Accepted

Aytham's semantic layer may borrow techniques from knowledge representation, provenance, workflows, rules, and graph systems, but the project must preserve a coherent path to ordinary programming constructs such as:

- values;
- expressions;
- binding;
- choice;
- repetition/recursion;
- failure/results;
- state/effects;
- modules.

The semantic contract layer should enrich ordinary computation rather than force every computation into a knowledge graph.

A future language specification remains blocked until the semantic kernel demonstrates comparative value.
