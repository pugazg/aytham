# Nannūl Comparative Notebook 001

Status: **comparative source study started**  
Purpose: test Aytham's readings of Tolkāppiyam against a later Tamil grammatical system without back-projecting Nannūl into the earlier source.

Primary electronic source used for this notebook:

- Project Madurai `pmuni0147`, *Nannūl* of Pavaṇanti Munivar;
- Unicode e-text conformed to the edition of Mani Thirunavukkarasu Mudaliar, Vavilla Ramaswamy Sastrulu & Sons, Madras, 1926;
- Project Madurai source numbering retained.

Commentary comparison is **not yet complete**.

---

# 1. Structural comparison already matters

## Tolkāppiyam research baseline

```text
Eḻuttatikāram   9 iyal
Collatikāram    9 iyal
Poruḷatikāram   9 iyal
```

## Nannūl structure in pmuni0147

After its prefatory material:

```text
Eḻuttatikāram
  2.1 Eḻuttu iyal
  2.2 Pataviyal
  2.3 Uyir-īṟṟup puṇariyal
  2.4 Mey-īṟṟup puṇariyal
  2.5 Urupu puṇariyal

Collatikāram
  3.1 Peyariyal
  3.2 Vinaiyiyal
  3.3 Potuviyal
  3.4 Idaiyiyal
  3.5 Uriyiyal
```

### Research consequence

Tamil grammatical thought itself is **not one frozen taxonomy**.

Aytham must not treat Tolkāppiyam's 27-iyal organization as if it were a ready-made ontology for programming.

The later reorganization is useful evidence that:

- categories can be regrouped;
- some topics become generalized;
- some are absorbed into broader chapters;
- boundary/combination rules can be reorganized around different organizing principles.

This strengthens Aytham's policy of borrowing **questions/principles**, not copying chapter structure.

---

# 2. Nannūl itself foregrounds rule/system design

The prefatory rules (especially early sūtras in the பொதுப்பாயிரம்) discuss properties of a good treatise: organization, concise expression, explanation, examples, ordering, methods, faults, and ways of establishing meaning.

This is relevant to Aytham primarily as **specification-design discipline**, not programming semantics.

Possible future use:

> Compare Aytham's specification-writing principles with the Tamil grammatical tradition of how a rigorous நூல் should organize and communicate rules.

Do not turn these pedagogical/literary rules into runtime language features.

---

# 3. பதவியல் — a compact form/meaning distinction

Nannūl 128 states, in substance, that a letter/form alone or in sequence, when it gives meaning, is a `பதம்`, then distinguishes further kinds.

Nannūl 131 places `பெயர் / வினை / இடை / உரி` among indivisible word forms in that analysis.

### Comparative significance

This again warns Aytham against treating:

```text
peyar | vinai | idai | uri
```

as a modern AST enum merely because the four labels occur together.

The source classification operates inside a grammatical account of meaningful word forms.

### Aytham question retained

Can the language clearly separate:

```text
source form
resolved computational meaning
```

without claiming that `padam` historically means AST/expression?

**Status:** research-only.

---

# 4. வினையியல் 320 — very important for the role-graph experiment

Nannūl sūtra 320 describes `vinai` through six associated dimensions:

```text
செய்பவன்   — doer/agent
கருவி      — instrument
நிலம்      — place/location
செயல்      — action
காலம்      — time
செய்பொருள் — acted-upon object/patient
```

and states that these are given/revealed by the verb/action expression.

### Why this matters

Experiment 001 proposes an action-centered relation graph:

```text
             Transfer
          /      |       \
     source   amount   destination
```

Nannūl gives us a later Tamil grammatical model in which an action is explicitly understood through a **structured set of participant/circumstance relations**.

This does **not** make Aytham's role graph historically present in Nannūl, and it does not establish programming novelty.

It does, however, strengthen the research question:

> Should Aytham make the semantic frame around an action explicit instead of representing an operation primarily as an ordered tuple of parameters?

### Revised paper-semantics candidate

Neutral modern form:

```text
ActionFrame {
    action
    participants: Relation*
    circumstances: Relation*
    temporal_context
    effects
    established_claims
}
```

Do not hard-code Nannūl's six categories as universal Aytham argument slots.

A database query, parser, tensor operation, or network send will need different role vocabularies.

### Prior-art consequence

This also pushes Aytham toward comparison with:

- semantic roles / thematic roles;
- case grammar;
- frame semantics;
- labelled transition systems;
- graph rewriting;
- knowledge graphs.

Those comparisons now become mandatory before any originality claim around role graphs.

---

# 5. வினையியல் and temporal meaning

Nannūl's வினையியல் continues to treat action together with tense/time and develops verbal forms.

Combined with the Tolkāppiyam source notebook, the historical evidence reinforces one modest but useful research principle:

> **Action and temporal context should not be treated as unrelated concerns when a program fact has time-sensitive validity.**

Experiment 002 examples:

```text
ownership_verified
  established_at T1
  valid_until T2
```

Again this is Aytham design, not a direct grammar translation.

---

# 6. எச்சம் is reorganized — important correction

Nannūl does **not** reproduce Tolkāppiyam's separate எச்சவியல் as one of its five Sollatikāram iyals.

Instead, material concerning `பெயரெச்சம்`, `வினையெச்சம்`, and other residual/elliptical relations appears across வினையியல் and பொதுவியல்.

Examples in the source include:

- பெயரெச்சம் material around 340–341;
- வினையெச்சம் from 342 onward;
- பொதுவியல் rules involving multiple kinds of எச்சம் around 355 onward.

### Consequence

This confirms the decision to withdraw:

```text
Eccaviyal → compiler inference
```

as an active Aytham mapping.

The historical category/topic is broader and reorganizable.

If Aytham later studies recoverable omission, it should be justified by a programming problem and modern inference literature—not by the chapter name alone.

---

# 7. பொதுவியல் — a new comparative target

Nannūl has a **பொதுவியல்** where Tolkāppiyam's Sollatikāram has several separately named iyals/topics.

This chapter includes material on:

- residual forms;
- compounds / தொடர்மொழி;
- தொகை நிலை;
- case-related combination;
- ambiguity/error/acceptable variation (`வழு`, `வழுவமைதி`); and other cross-cutting word-grammar matters.

### Aytham research lesson

Our current semantic graph may also require a **cross-cutting rule layer** rather than forcing every rule under one Tamil-inspired category.

Potential design distinction:

```text
local semantic category rules
vs
cross-category well-formedness/composition rules
```

Do not name this `potuviyal` in Aytham yet.

**Priority:** HIGH comparative study.

---

# 8. இடையியல் 420 — remarkable continuity in relational dependence

Nannūl 420 groups several functions under the `idai` domain and explicitly characterizes them as lacking fully independent operation and occurring in relation to `peyar` and `vinai`.

This is strongly compatible with the Tolkāppiyam Source Notebook 001 observation.

### Comparative result

The inspiration:

> `idai` as **mediating/relational meaning dependent on surrounding forms**

is more historically defensible than:

> `idai` = generic control-flow operator.

### Aytham research candidate

A semantic connector might contribute meaning such as:

```text
sequence
causal dependency
success/failure continuation
evidence flow
temporal relation
role binding
```

But `idai` should only enter the formal language if this mediation model proves computationally useful.

**Status:** strengthened as research inspiration; still not a frozen term.

---

# 9. உரியியல் 442 — Nannūl narrows/clarifies one side of the picture

Nannūl 442 describes `uriccol` in relation to various `paṇpu`/qualities and its relation with names/actions in poetic usage.

### Comparative result

This gives stronger later support for a **quality/qualification** dimension than the simplistic English gloss alone did.

However it still does **not** justify Aytham's full:

```text
QualificationClaim {
  predicate
  provenance
  authority
  freshness
  scope
}
```

That structure remains a modern Aytham invention assembled in response to validated-data-flow problems and modern provenance/type-system research.

### Terminology decision remains

Keep:

```text
qualification claim
```

as neutral paper-semantics terminology.

Do not yet freeze `uri` as the formal programming term.

---

# 10. Punarcci organization — highly useful design-method comparison

Nannūl reorganizes combination material into:

```text
Uyir-ending combination
Mey-ending combination
Urupu combination
```

rather than duplicating Tolkāppiyam's exact Eḻuttatikāram organization.

One later general rule (Nannūl 239) also acts as a broad instruction for how unenumerated/intermediate/borrowed forms should be combined appropriately.

### Research lesson

A composition system benefits from separating:

```text
general boundary law
participant/category-specific law
exception/contextual law
fallback/extensibility rule
```

This could become important for Aytham `composition contracts`.

### Prototype question

Suppose modules A and B compose:

1. Is there a universal compatibility rule?
2. Can the producer/consumer categories add specialized rules?
3. Can a domain define an extension rule?
4. How are conflicts among general/special rules resolved?
5. Can tooling explain which rule decided the boundary?

This is a modern rule-system design question inspired by comparative grammatical organization.

---

# 11. First Tolkāppiyam ↔ Nannūl comparison matrix

| Research area | Tolkāppiyam first reading | Nannūl first reading | Aytham consequence |
|---|---|---|---|
| பெயர்/வினை/இடை/உரி | categories related within Sollatikāram; not safely a flat enum | four appear again, but within reorganized system | Do not use a flat AST taxonomy merely from names |
| வினை | strongly tied to time/tense | explicitly exposes agent, instrument, place, action, time, object | Action-frame research strengthened |
| வேற்றுமை | explicit case system | treated substantially within later reorganized word grammar | Semantic-role graph remains analogy/design, not source copy |
| இடை | operates with peyar/vinai; mediating functions | relational/non-independent characterization persists | `idai = mediation` research strengthened |
| உரி | broad/contextual lexical-semantic opening | quality/qualification dimension made prominent | `uri = refinement` still too narrow; neutral term retained |
| எச்சம் | separate Eccaviyal but broad content | distributed across Vinai/Potuviyal | Inference mapping withdrawn |
| புணர்ச்சி | multiple boundary-focused iyals/rules | reorganized into three combination chapters + broad rules | Study rule architecture, not phonological copying |

---

# 12. New research hypothesis generated by Nannūl

## Action frame, not function signature

The strongest new cross-source research direction is:

```text
Action
  ├── participant relations
  ├── circumstance relations
  ├── temporal context
  ├── qualifications/evidence
  ├── effects
  └── result relations
```

A conventional function type might be:

```text
A × B × C -> D
```

Aytham should test whether an explicit semantic frame provides enough benefit to justify itself:

```text
required relation graph
        ↓
      action
        ↓
produced relation graph
```

This now needs comparison not only with programming types but with linguistic **case/frame semantics** and knowledge-representation systems.

---

# 13. What we need from a Nannūl commentary

The Project Madurai mūlam is enough to start structural comparison, but not enough to settle interpretation.

A commentary is especially useful for:

1. Nannūl 320 — exact interpretation of the six action-related dimensions;
2. 420 — scope and nature of இடை categories;
3. 442 onward — `uri` / `paṇpu` relation;
4. எச்சம் treatment across Vinaiyiyal/Potuviyal;
5. the organization and generalization principles of புணர்ச்சி;
6. differences explicitly noted by commentators between Tolkāppiyam and Nannūl.

Preferred next addition from the user, if available:

> **Nannūl mūlam + a reliable commentary edition** (Mयிலைநாதர் / சங்கர நமச்சிவாயர் or another well-identified edition), ideally as a scan with bibliographic data.

The existing public mūlam remains the baseline; commentary claims must be attributed separately.

---

# 14. Sangam literature remains next, but for a different job

Do not use Sangam texts yet to prove programming semantics.

Use them later to test:

- contextual interpretation;
- participant/relation recovery;
- explicit vs implicit evidence;
- tiṇai/category use in actual literature;
- ambiguity;
- source-grounded Tamil computational examples.

This should begin only after the first full Sollatikāram + Nannūl comparison pass, so the literary corpus is asked a precise question rather than mined for attractive analogies.

---

# 15. Current verdict after Nannūl first pass

Three ideas become **stronger research candidates**:

1. **action-centered relation frames**;
2. **idai as semantic mediation**;
3. **composition as a rule-governed boundary with general/special/contextual laws**.

Two ideas become **more cautious**:

1. `uri` should not be frozen as “refinement type”;
2. `eccam` should not be frozen as “compiler inference.”

The next theoretical comparison should therefore expand from type systems into:

```text
semantic roles
case grammar
frame semantics
knowledge graphs
graph rewriting
planning/type-directed synthesis
```

before Aytham formalizes its action graph.