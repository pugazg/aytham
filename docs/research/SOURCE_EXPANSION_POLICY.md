# Aytham Source Expansion Policy

Status: **active research policy**

## Purpose

Aytham may draw design inspiration from Tamil grammatical and literary traditions, but additional sources must not be mixed into one undifferentiated authority.

Different sources answer different questions.

The project therefore separates at least four layers:

```text
A. foundational grammatical source
B. later comparative grammar
C. attested literary usage
D. modern scholarship/commentary
```

and keeps **Aytham inventions** separate from all four.

---

# 1. Layer A — Tolkāppiyam

Current role:

> **Primary historical design-inspiration source for the research phase.**

The structured baseline is currently `pugazg/tolkappiyam-arivagam`, derived from Project Madurai `pmuni0100`.

Aytham uses Tolkāppiyam to generate questions, not to claim ancient descriptions of computing.

Required discipline:

- cite exact iyal/nūṟpā when making source claims;
- distinguish source text from editorial gloss/commentary;
- do not translate a grammatical category directly into a compiler construct without an independent programming problem;
- document rejected analogies as well as adopted inspirations.

---

# 2. Layer B — Nannūl

## Recommended status

**BRING IN NEXT.**

Nannūl is immediately useful because it can act as a **comparative grammatical layer**, not as a replacement for Tolkāppiyam.

Research questions include:

1. Which Tolkāppiyam categories remain central in Nannūl?
2. Which are reorganized, narrowed, expanded, or explained differently?
3. Does later grammatical systematization clarify categories relevant to Aytham such as:
   - பெயர்;
   - வினை;
   - இடை;
   - உரி/qualification-related categories;
   - வேற்றுமை;
   - புணர்ச்சி;
   - எச்சம்/ellipsis;
   - word formation;
   - morphology and relation marking?
4. Are concepts Aytham currently finds attractive actually stable grammatical ideas, or are we over-reading one editorial framing of Tolkāppiyam?
5. Does Nannūl provide a cleaner vocabulary for any modern Aytham concept than the terminology we are currently borrowing?

## Historical rule

Never write:

```text
Tolkāppiyam says X because Nannūl explains X this way.
```

Instead record:

```text
Tolkāppiyam source: ...
Later Nannūl treatment: ...
Comparison/interpretation: ...
Aytham design decision: ...
```

Later grammar must not be silently back-projected into the earlier source.

## Preferred source package

For Aytham, the ideal Nannūl material is:

1. Tamil mūlam in a stable edition/digital text;
2. page/verse/sūtra numbering preserved;
3. bibliographic metadata;
4. one or more reliable commentaries if legally usable;
5. an English translation only as a secondary aid, never as sole authority.

If only a scan is available, preserve the scan as controlling source and build a traceable transcription layer before extracting design claims.

---

# 3. Layer C — Sangam literature

## Recommended status

**BRING IN, but use after the first Nannūl comparison notebook is established.**

Sangam literature should not be treated as another grammar manual. Its role is different:

> **attested usage, contextual structure, and stress-testing of grammatical/literary categories in actual texts.**

This becomes especially important before Aytham borrows from பொருளதிகாரம்.

### High-value research uses

#### A. Test context / திணை claims

Before using `tiṇai` as inspiration for computational context, examine how contextual classification actually functions in poems rather than relying on a schematic summary.

Questions:

- What evidence identifies a context?
- Is context explicit or inferred?
- Can multiple signals jointly establish context?
- What happens when expected signals are absent/ambiguous?
- Is classification descriptive, normative, interpretive, or all three depending on source/tradition?

These questions may inform Aytham research about context-sensitive meaning, but no direct mapping is assumed.

#### B. Test ellipsis/context recovery

If Aytham researches `எச்சவியல்` as inspiration for safe inference, literary examples may help us understand how omitted information is recovered through context.

Again, natural-language ellipsis is **not** a compiler algorithm. The value is conceptual comparison.

#### C. Test relation and participant modelling

Poems can provide rich examples in which participants, actions, roles, attributes, context, and implied relations interact.

They may become useful semantic-graph stress tests later:

```text
participants
relations
actions
qualifications
context
evidence for interpretation
ambiguity
```

#### D. Tamil-first example domains

Once the language exists, literary datasets could become excellent Aytham demonstration programs:

- corpus filtering;
- metre checking;
- morphological analysis;
- provenance-aware transcription;
- poem classification;
- lexical search;
- source/edition comparison.

This would demonstrate a Tamil-first language on genuinely Tamil computational problems rather than only translating banking/web examples.

---

# 4. Layer D — commentaries and modern scholarship

Commentaries are essential for interpreting difficult source categories but must be labelled by commentator/edition.

Modern linguistic/computational scholarship is needed to prevent reinvention and forced analogy.

Research records should therefore support:

```text
SOURCE
COMMENTARY
MODERN SCHOLARSHIP
INTERPRETATION
AYTHAM DESIGN
```

This is more precise than collapsing everything non-Aytham into `SOURCE`.

---

# 5. Source disagreement policy

If Tolkāppiyam editions/commentaries or later grammars disagree:

- record the disagreement;
- do not select whichever reading best supports an Aytham feature;
- do not hide uncertainty behind English terminology;
- keep Aytham's programming abstraction independent enough that a scholarly disagreement does not masquerade as compiler correctness.

A feature should be adopted because it is computationally useful, not because one interpretation conveniently resembles computing.

---

# 6. Design provenance record

Every Tamil-inspired Aytham concept should eventually have a record like:

```yaml
concept: evidence_backed_qualification
working_tamil_term: uri

historical_sources:
  tolkappiyam:
    status: source-reviewed
    references: []
  nannul:
    status: pending
    references: []

attested_usage:
  sangam:
    status: not-applicable-or-pending
    references: []

modern_comparison:
  - refinement_types
  - provenance_systems
  - typestate

aytham_invention:
  summary: ...
  status: experimental

historical_claim_strength: limited
computational_evidence: experiment-002
```

The exact schema may change, but provenance must remain inspectable.

---

# 7. Immediate acquisition request

## Needed now

**Nannūl Tamil source**.

Preferred order:

1. mūlam / source text;
2. reliable commentary edition(s), if available;
3. translation/reference aids.

First comparison targets:

```text
வேற்றுமை
பெயர்
வினை
இடை
எச்சம் / omission-related treatment
புணர்ச்சி
word formation / morphology
qualification-related categories
```

We should not assume Nannūl uses identical category boundaries or terminology; that is precisely what the comparison must discover.

## Useful next

**Sangam literature corpus**, beginning with a bounded, traceable work/selection rather than ingesting everything at once.

First Sangam use should be a research notebook on **context and recoverable meaning**, not a new Aytham feature.

---

# 8. Decision rule for adding any future Tamil source

Add a source only if we can state which question it answers.

Bad reason:

> It is another important Tamil classic.

Good reason:

> We need attested examples to test whether our interpretation of contextual role inference is plausible.

Possible future sources may include later grammars, commentaries, lexicons, inscriptions, or domain literature, but each must receive an explicit role before entering the design evidence set.

---

# 9. Current source strategy

```text
Tolkāppiyam
    │
    │ foundational grammatical research
    ▼
Nannūl
    │
    │ diachronic/comparative grammar check
    ▼
Sangam literature
    │
    │ attested usage/context stress tests
    ▼
Modern PL / linguistic scholarship
    │
    │ prior art + formal comparison
    ▼
Aytham experiments
    │
    ▼
Aytham specification
```

The arrows do not mean chronological dependence or scholarly superiority. They represent Aytham's **research workflow**.