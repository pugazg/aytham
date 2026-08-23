# ஆய்தம் · Aytham

**A research-first project exploring whether Tamil grammatical thought can inspire a genuinely useful programming-language model.**

> Status: **Research / pre-specification — direction reset active**
>
> Authoritative current-state review: [`docs/AYTHAM_DIRECTION_RESET_2026-08-23.md`](docs/AYTHAM_DIRECTION_RESET_2026-08-23.md)
>
> No claim is made that Aytham is the first Tolkāppiyam-inspired programming language. Novelty claims remain provisional until prior-art and source research are substantially complete.

## Core question

Most Tamil programming-language projects make programming accessible through Tamil syntax, Tamil keywords, or Tamil identifiers while retaining a familiar programming model.

Aytham asks a different question:

> **Can Tamil grammatical thought help us decide which computational relationships a programming language should make explicit, rather than merely translating conventional programming vocabulary into Tamil?**

The project does **not** claim that Tolkāppiyam or Nannūl describes modern computation. Tamil grammatical sources are treated as design-research sources whose concepts must pass two independent tests:

1. **Historical honesty** — what does the source actually describe?
2. **Computational usefulness** — does the inspired abstraction make programming safer, clearer, more composable, or easier to reason about?

## Why ஆய்தம்?

`ஃ` is a distinctive Tamil sign with a special structural place in the writing system. The project uses **Aytham / ஆய்தம்** as its working language name and `ஃ` as a possible visual identity.

The name is a **working project identity**, not a declaration of trademark clearance.

---

# Current project reset — 2026-08-23

A full review of the live repository after Experiments 001–034 found that the **research direction remains promising**, but the recent implementation sequence moved ahead of its own evidence gates.

The project is therefore pausing new path-search/planning work and returning to:

```text
problem
→ prior art
→ falsifiable hypothesis
→ comparative benchmark
→ critical review
→ decision
→ implementation
```

rather than advancing through experiment numbers alone.

## Current implementation truth

The live implementation currently consists only of:

```text
prototype/validator/aytham_validator.py
```

It is a minimal research validator that:

- indexes claims by subject/property;
- finds an action by ID;
- checks whether required claims exist with matching values;
- returns a simple success/failure explanation.

It does **not** yet implement:

- a full `SemanticGraph` object model;
- relation validation;
- confidence ordering;
- provenance/evidence reasoning;
- lineage preservation/invalidation;
- transformation execution;
- transformation discovery;
- semantic path search;
- example JSON fixtures;
- automated tests.

Older prototype documents that describe these components remain **design history**, not evidence that they are implemented.

---

# Strongest current computational hypothesis

The strongest surviving direction is an **action/relation/claim semantic contract model** rather than a generic ontology.

A deliberately small kernel candidate is now under test:

```text
Subject / Value Identity
Relation / Role
Claim
Action / Transformation
Composition Judgment
```

Supporting structures may be introduced only when needed:

```text
Evidence
Context
Authority
Confidence / epistemic status
Provenance / lineage
```

An Action/Transformation may eventually expose:

```text
participants
requires
establishes
preserves
invalidates
effects
capabilities
```

This remains **paper semantics / experimental architecture**, not an accepted language specification.

## Why this direction remains interesting

The potential value is not that any one ingredient is new. The individual ingredients have extensive prior art.

The research opportunity is whether Aytham can make:

```text
roles
+ established claims
+ provenance/lineage when relevant
+ effects/capabilities
+ checked semantic composition
+ meaning-oriented diagnostics
```

one coherent everyday programmer-facing model without excessive annotation burden.

---

# Relationship to Tamil grammatical research

The early organizing frame remains useful:

```text
எழுத்து · eḻuttu
        ↓
சொல் · sol
        ↓
பொருள் · poruḷ
```

but it is **not** treated as a mapping to lexer → parser → semantic analyser.

Direct source study has also made the project more cautious about treating `பெயர் / வினை / இடை / உரி` as four modern programming categories.

### வேற்றுமை

Inspires research into **first-class semantic role relations**.

Aytham roles are a modern invention, not grammatical cases copied into software.

### வினை

Inspires action/transformation research.

Direct Tolkāppiyam/Nannūl study shows the historical category is deeply connected with grammatical and temporal structure, so:

```text
vinai = function
```

is rejected.

### இடை

Currently researched as **semantic mediation/connection**, not as a Tamil label for:

```text
if
pipes
punctuation
generic control flow
```

### உரி

Terminology remains **reopened**.

Direct source study shows `uri` is broader than a simple refinement predicate. Paper semantics therefore use the neutral modern term **qualification claim** where such a programming construct is being tested.

### புணர்ச்சி

Inspires **boundary-sensitive checked composition**.

It is not treated as a historical theory of software composition.

### பெயர்

Remains a research perspective for denotable semantic subjects/entities, not a frozen AST node type.

---

# What prior-art comparison changed

Experiment 002 compared the validated-data-flow hypothesis with:

- refinement types;
- typestate;
- dependent types;
- proof-carrying approaches;
- effect systems;
- SSA/value lineage;
- provenance systems;
- language-integrated provenance;
- protocol/session typing.

The conclusion remains deliberately conservative:

> **Individual mechanism novelty is low.**

Aytham must therefore prove practical value through integration, developer ergonomics, diagnostics, and composition rather than merely combining known mechanisms.

The next prior-art pass must go deeper into:

- semantic roles / thematic roles;
- Fillmore-style case grammar;
- frame semantics;
- knowledge graphs;
- graph rewriting;
- Hoare-style contracts;
- rule systems / logic programming;
- automated planning;
- proof search;
- type-directed synthesis.

---

# What source study changed

The project surveys **all 27 Tolkāppiyam iyals**, rather than selecting only concepts that conveniently resemble programming.

Direct source reading corrected several early assumptions:

- `uri = refinement type` is no longer accepted;
- `idai = generic control flow` is rejected;
- the early `Eccaviyal → compiler inference` analogy is withdrawn;
- `vinai` is not equated with an arbitrary effectful function;
- `punarcci` is studied as a model of rule-governed boundaries rather than translated phonology;
- பொருளதிகாரம் concepts have a deliberately higher bar because forced-analogy risk is especially high.

The first-pass 27-iyal survey is retained as research history; later corrections are recorded explicitly in `TOLKAPPIYAM_27_IYAL_SURVEY_ERRATA.md` rather than silently erasing earlier hypotheses.

---

# Nannūl comparison and commentary

Nannūl entered the research set as a **later comparative grammar**, not as an authority to be back-projected into Tolkāppiyam.

The strongest useful research outcomes have been:

1. **action-centred relation frames**;
2. **idai as mediation/dependence**;
3. **boundary-sensitive composition with general and context-dependent rules**;
4. edition-aware source provenance;
5. the distinction between explicit, inferred, derived, and contextual relations.

Commentarial study includes:

- **Mayilaināthar commentary**;
- **Śaṅkara Namaccivāyar's Virutti commentary revised by Sivajñāna Munivar**.

Nannūl numbering is treated as edition-aware. For example, the same rule may appear as 319 or 320 depending on edition/commentary tradition. Aytham therefore identifies source rules by:

```text
incipit
+ edition/commentary
+ local number
+ printed page/source
```

rather than by bare number alone.

---

# Sangam literature

Sangam literature remains a different evidence layer:

> **attested usage and contextual stress-testing**, especially before Aytham borrows anything from திணை or other பொருளதிகாரம் concepts.

It is not another grammar manual and should not be mined for attractive programming metaphors.

Potential research uses include:

- contextual interpretation;
- participant/relation recovery;
- explicit vs implicit evidence;
- ambiguity;
- provenance-aware textual computation;
- genuinely Tamil computational example domains.

---

# What Aytham must NOT become

Aytham should not be only this:

```text
if       → எனில்
else     → இல்லையெனில்
function → வினை
print    → அச்சிடு
```

Nor should it claim that modern programming concepts are directly present in ancient or medieval grammar.

It also must not silently become only a knowledge graph, provenance database, or workflow engine.

A future programming language must still provide a coherent account of ordinary computation:

- values/literals;
- expressions;
- binding;
- action/transformation invocation;
- choice;
- repetition/recursion;
- failure/results;
- state/effects;
- modules.

The semantic layer should enrich ordinary computation rather than force every calculation or algorithm into a large semantic graph.

---

# Meaning-oriented diagnostics

One of the strongest practical hypotheses remains developer-facing explanation.

Instead of only:

```text
Expected VerifiedEmail
Found EmailAddress
```

Aytham aims to test diagnostics such as:

```text
SendSensitiveMessage cannot execute.

Required:
    ownership_verified

Established:
    email_syntax_valid

The ownership requirement is not established
for this value lineage.
```

A registered bridge transformation may be suggested only when its own prerequisites are satisfied and the system can explain why it applies.

Diagnostic quality is part of the semantic research, not later polish.

---

# Progressive disclosure

Aytham must not require provenance-heavy declarations for trivial computation.

Simple code should remain simple.

Evidence, authority, context, confidence, freshness, lineage, and other advanced structures should become visible only when the problem actually depends on them.

This is a core pass/fail criterion for the entire design.

---

# Current paused direction — transformation path search

Experiments 029–034 explored transformation discovery and semantic path search.

That direction is now **PAUSED**.

Reason:

A realistic semantic state may contain multiple simultaneous claims, relations, capabilities, contexts, and effects. A simple:

```text
Claim A
  → Transformation
  → Claim B
```

model is insufficient as a general action semantics.

Before path planning can resume, the project must complete direct comparison with planning, rule systems, graph rewriting, workflow models, proof search, and type-directed synthesis—and demonstrate a real programming need for automatic path discovery.

Path search is not required to prove the core Aytham language hypothesis.

---

# Current evidence gate

The immediate goal is **not** a compiler and not another path-search experiment.

The project is currently at:

## Phase 1A — Focused prior-art comparison

followed by:

## Phase 1B — Comparative Benchmark 001

The benchmark will model a verified sensitive-email workflow in:

1. TypeScript;
2. Rust;
3. the Aytham Semantic Kernel Candidate.

It must test:

```text
email_syntax_valid
ownership_verified
verification scope
verification freshness
value mutation / lineage invalidation
network_send effect
```

and compare:

- invalid states prevented;
- annotation/ceremony;
- wrapper/type proliferation;
- API readability;
- diagnostic quality;
- provenance explanation;
- mutation invalidation clarity;
- progressive-disclosure burden.

Only after this benchmark and a critical review should the project decide which semantic mechanisms deserve real implementation.

---

# Principles

1. **Meaning before syntax.** Compiler keywords do not define the research model.
2. **Tamil-inspired, not Tamil-decorated.** A borrowed concept must affect reasoning or behaviour.
3. **Source-grounded.** Historical claims require traceable evidence.
4. **No forced analogies.** Attractive but weak mappings are explicitly held or rejected.
5. **Useful beyond novelty.** Aytham must solve real programming problems.
6. **Prior-art honest.** Established mechanisms are acknowledged rather than renamed as inventions.
7. **Unicode-native.** Tamil source must receive correct normalization, grapheme-aware diagnostics, and mixed-script safety.
8. **Tamil-first, interoperable.** A Tamil conceptual model must not isolate programmers from existing ecosystems.
9. **Explicit provenance.** Source, commentary, modern scholarship, interpretation, and Aytham invention remain distinct.
10. **Meaning-oriented diagnostics are part of the language hypothesis.**
11. **Progressive disclosure.** Ordinary code must not require semantic bureaucracy.
12. **Evidence before architecture.** Planned components are not implementation progress.
13. **Falsification before attachment.** Every feature needs a condition under which it will be removed or revised.

---

# Repository map

```text
docs/
  AYTHAM_DIRECTION_RESET_2026-08-23.md
  DECISIONS.md
  ROADMAP.md

  research/
    PRIOR_ART.md
    RESEARCH_QUESTIONS.md
    TOLKAPPIYAM_CONCEPT_MAP.md
    TOLKAPPIYAM_27_IYAL_SURVEY.md
    TOLKAPPIYAM_27_IYAL_SURVEY_ERRATA.md
    TOLKAPPIYAM_SOURCE_NOTEBOOK_001.md
    NANNUL_COMPARATIVE_NOTEBOOK_001.md
    NANNUL_COMMENTARY_NOTEBOOK_002.md
    NANNUL_SOURCE_CONCORDANCE.md
    EXPERIMENT_002_COMPARATIVE_ANALYSIS.md
    AYTHAM_RESEARCH_BASELINE_v0.1.md
    AYTHAM_BASELINE_CRITICAL_REVIEW_v0.1.md
    AYTHAM_SEMANTIC_MODEL_v0.1.md
    AYTHAM_SEMANTIC_MODEL_v0.2.md
    SOURCE_EXPANSION_POLICY.md

  experiments/
    001_ROLE_GRAPH_TRANSFER.md
    002_VALIDATED_DATA_FLOW.md
    ...

  specification/
    experimental semantic-model documents

  prototype/
    historical prototype/design experiments

prototype/
  validator/
    aytham_validator.py
```

The folders `specification/` and `prototype/` contain experimental research artifacts; their names do not mean the project has reached a frozen language specification or mature runtime.

---

# Current maturity

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

# Governing research rule

For every major next step ask:

```text
What real programming problem is being solved?
What established technique already addresses it?
What exactly does Aytham change?
How will that difference be tested?
What result would cause us to reject the idea?
Only then: what should we implement?
```

## License

A license has not yet been selected. Do not assume source or documentation licensing until an explicit repository license is added.
