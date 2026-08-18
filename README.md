# ஆய்தம் · Aytham

**A research-first project exploring whether Tamil grammatical thought can inspire a genuinely useful programming-language model.**

> Status: **Research / pre-specification**
>
> No claim is yet made that Aytham is the first Tolkāppiyam-inspired programming language. Novelty claims remain provisional until prior-art and source research are substantially complete.

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

## Current research model

The early organizing frame remains useful:

```text
எழுத்து · eḻuttu
        ↓
சொல் · sol
        ↓
பொருள் · poruḷ
```

but it is **not** treated as a mapping to lexer → parser → semantic analyser.

Direct source study has also made the project more cautious about treating `பெயர் / வினை / இடை / உரி` as four modern programming categories. They are now research perspectives whose historical relationships must be understood before any terminology is frozen.

The strongest current computational hypothesis is an **action-centred semantic relation graph**:

```text
                         action
                           │
          ┌────────────────┼────────────────┐
          │                │                │
   participant roles   circumstances   temporal context
          │                │                │
          └────────────────┼────────────────┘
                           │
                  qualification claims
                    / evidence / scope
                           │
                    effects + lineage
                           │
                  checked composition
```

A transformation may therefore eventually be described by relationships such as:

```text
requires
establishes
preserves
invalidates
effects
capabilities
produces
```

This remains **paper semantics**, not accepted language architecture.

### Current source-inspired research terms

- **வேற்றுமை** — inspires research into first-class semantic role relations; Aytham roles are a modern invention, not grammatical cases copied into software.
- **வினை** — inspires action/transformation research; direct Tolkāppiyam/Nannūl study shows the historical category is deeply connected with temporal and grammatical structure, so `vinai = function` is rejected.
- **இடை** — currently researched as **semantic mediation/connection**, not as a Tamil label for `if`, pipes, punctuation, or generic control flow.
- **உரி** — terminology is **reopened**. Experiment 002 uses the neutral modern term **qualification claim** because direct source study shows `uri` is broader than a simple refinement predicate.
- **புணர்ச்சி** — inspires **boundary-sensitive checked composition**; it is not treated as a historical theory of software composition.
- **பெயர்** — remains a research perspective for denotable semantic subjects/entities, not a frozen AST node type.

## What prior-art comparison changed

Experiment 002 was compared against refinement types, typestate, dependent types, proof-carrying approaches, effect systems, SSA/value lineage, provenance systems, and related work.

The conclusion is deliberately conservative:

> **Individual mechanism novelty is low.**

The research opportunity is whether Aytham can make **role + qualification/evidence + provenance + lineage + effects + composition** one coherent programmer-facing meaning model, with unusually strong diagnostics and tooling.

Combination alone does not establish novelty. It must demonstrate practical value.

## What source study changed

The project now surveys **all 27 Tolkāppiyam iyals**, rather than selecting only concepts that conveniently resemble programming.

Direct source reading has already corrected several early assumptions:

- `uri = refinement type` is no longer accepted;
- `idai = generic control flow` is rejected;
- the early `Eccaviyal → compiler inference` analogy is withdrawn pending full source study;
- `vinai` is not equated with an arbitrary effectful function;
- `punarcci` is studied as a model of rule-governed boundaries rather than translated phonology;
- பொருளதிகாரம் concepts have a deliberately higher bar because forced analogy risk is especially high.

The first-pass 27-iyal survey is retained as research history; later corrections are recorded explicitly in `TOLKAPPIYAM_27_IYAL_SURVEY_ERRATA.md` rather than silently erasing earlier hypotheses.

## Nannūl comparison and commentary

Nannūl has entered the research set as a **later comparative grammar**, not as an authority to be back-projected into Tolkāppiyam.

The first comparison strengthens three research directions:

1. **action-centred relation frames**;
2. **idai as mediation**;
3. **boundary-sensitive composition with general and context-dependent rules**.

Commentarial study now includes two Tamil Virtual Academy sources supplied for the project:

- **Mayilaināthar commentary**, TVA's `நன்னூல் மூலமும் மயிலைநாதருரையும்`;
- **Śaṅkara Namaccivāyar's Virutti commentary revised by Sivajñāna Munivar**.

The commentary pass adds two important requirements:

- semantic relations should be able to record whether they were **explicit, inferred, derived, or contextual**;
- Nannūl provenance must be **edition-aware**, because the same rule can carry different nūṟpā numbers across editions/commentarial traditions.

For example, the opening punarcci definition appears as 150 in the Mayilaināthar/U. Vē. Cā. presentation but 151 in common TVA numbering, while the action-frame rule appears as 319 versus 320. Aytham therefore identifies source rules by **incipit + edition/commentary + local number + page/source**, never by bare number alone.

`uri` and `eccam` terminology remain provisional.

## Sangam literature

Sangam literature is planned as a different evidence layer:

> **attested usage and contextual stress-testing**, especially before Aytham borrows anything from திணை or other பொருளதிகாரம் concepts.

It will not be treated as another grammar manual or mined for attractive programming metaphors.

Potential later uses include:

- contextual interpretation;
- participant/relation recovery;
- explicit vs implicit evidence;
- ambiguity;
- tiṇai in actual literary usage;
- genuinely Tamil computational example programs.

## What Aytham must NOT become

Aytham should not be only this:

```text
if       → எனில்
else     → இல்லையெனில்
function → வினை
print    → அச்சிடு
```

Nor should it claim that modern programming concepts are directly present in ancient or medieval grammar.

Every Tamil-inspired feature must remain separable into:

```text
historical source
commentarial / scholarly interpretation
modern programming-language prior art
Aytham design invention
```

## Principles

1. **Research before implementation.** Compiler code follows semantic clarity.
2. **Tamil-inspired, not Tamil-decorated.** A borrowed concept must affect reasoning or behaviour.
3. **Source-grounded.** Historical claims require traceable evidence.
4. **No forced analogies.** Attractive but weak mappings are explicitly held or rejected.
5. **Useful beyond novelty.** Aytham must solve real programming problems.
6. **Prior-art honest.** Established mechanisms are acknowledged rather than renamed as inventions.
7. **Unicode-native.** Tamil source must receive correct normalization, grapheme-aware diagnostics, and mixed-script safety.
8. **Tamil-first, interoperable.** A Tamil conceptual model must not isolate programmers from existing ecosystems.
9. **Explicit provenance.** Source, commentary, modern scholarship, interpretation, and Aytham invention remain distinct.
10. **Meaning-oriented diagnostics are part of the language.** Errors should explain missing relationships, evidence, effects, or composition conditions in domain terms.

## Research map

```text
docs/
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
    SOURCE_EXPANSION_POLICY.md

  experiments/
    001_ROLE_GRAPH_TRANSFER.md
    002_VALIDATED_DATA_FLOW.md

  design/
    DESIGN_PRINCIPLES.md
    COMPUTATIONAL_MODEL.md

  DECISIONS.md
  ROADMAP.md
```

## Current milestone

**Milestone 0 — Research foundation / paper semantics**

Current work:

1. deepen the complete Sollatikāram source notebook;
2. extend the Nannūl cross-edition concordance and commentarial comparison;
3. extend prior-art comparison into semantic roles, case grammar, frame semantics, knowledge graphs, graph rewriting, and type-directed synthesis/planning;
4. test whether an **action frame** is better than a conventional function signature for several unrelated programming problems;
5. study Eḻuttatikāram composition-rule architecture;
6. conduct a Poruḷatikāram anti-analogy review before borrowing contextual concepts;
7. then introduce bounded Sangam literature as attested-usage/context evidence;
8. only after these decide which Tamil technical terms belong in Specification 0.1.

Success still requires at least one abstraction that proves measurably useful compared with conventional alternatives.

## Source discipline

Research records distinguish:

- **SOURCE** — directly supported by a cited primary text;
- **COMMENTARY** — attributed traditional/commentarial interpretation;
- **MODERN SCHOLARSHIP** — linguistic/computational/programming-language research;
- **INTERPRETATION** — a reasoned comparison or reading;
- **AYTHAM DESIGN** — a modern programming-language invention inspired by the research.

Later sources such as Nannūl must never be silently projected backward into Tolkāppiyam.

## License

A license has not yet been selected. Do not assume source or documentation licensing until an explicit repository license is added.
