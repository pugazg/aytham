# Tolkāppiyam Source Notebook 001

Status: **source-level reading in progress**  
Scope: first direct-source pass over the Aytham concepts currently under highest pressure from Experiments 001–002.

Controlling electronic source for this notebook:

`pugazg/tolkappiyam-arivagam/data/source/project-madurai-pmuni0100.html`

This notebook deliberately distinguishes:

- **SOURCE OBSERVATION** — what the Tamil source in the cited iyal/nūṟpā directly gives us;
- **INTERPRETATION CAUTION** — what we must not infer too quickly;
- **AYTHAM CONSEQUENCE** — how the programming-language research should change.

This is not a full philological commentary. Commentarial and Nannūl comparison are pending.

---

# 1. கிளவியாக்கம் — the four word families are not four programming kinds

A useful early source statement in கிளவியாக்கம் says, in summary, that `சொல்` is known as `பெயர்` and `வினை`, while `இடைச்சொல்` and `உரிச்சொல்` occur along their paths/relations (2.1, nūṟpā 4–5 in the supplied source numbering).

### SOURCE OBSERVATION

The source does **not** present four modern mutually exclusive compiler node kinds called:

```text
Peyar
Vinai
Idai
Uri
```

as Aytham's early concept diagrams may visually suggest.

The relationship among the categories is structurally subtler: பெயர்/வினை are foregrounded, while இடை/உரி are described in relation to them.

### AYTHAM CONSEQUENCE

Do not freeze this enum:

```text
SemanticCategory = Peyar | Vinai | Idai | Uri
```

Instead test a relational model in which:

- a semantic subject may be denoted (`peyar`-inspired perspective);
- an action/transformation may occur (`vinai`-inspired perspective);
- connective/mediating information may relate other forms (`idai` inspiration);
- qualification/contextual meaning information may modify or resolve other forms (`uri` inspiration).

The source itself is a reason to prefer **relationships over a flat taxonomy**.

---

# 2. வேற்றுமையியல் — relation marking is promising, but “semantic roles” is Aytham's invention

The opening of வேற்றுமையியல் enumerates the case system and its forms; it then discusses the subject case and functions/uses associated with nominal expressions (2.2, nūṟpā 1–6).

### SOURCE OBSERVATION

This is a grammatical case system, not a general ontology of software roles.

### INTERPRETATION CAUTION

The Aytham relation:

```text
Account A --source-of--> Transfer
```

is **not** a direct translation of a Tolkāppiyam case.

Likewise, Aytham must not create a fixed programming list of “eight semantic cases” merely to mirror the source.

### AYTHAM CONSEQUENCE

What survives as inspiration is the broader insight:

> The same underlying nominal entity participates in different meaning relations according to how it stands in a larger expression.

Aytham's role graph remains viable only as **AYTHAM DESIGN** and must be compared against semantic-role systems, labelled arguments, relational typing, and graph calculi.

---

# 3. வேற்றுமை மயங்கியல் — source study required before role coercion

The structured corpus describes this iyal as case overlap: one case form may do the work of another in specified circumstances.

### CURRENT RESEARCH TEMPTATION

Aytham could imagine role entailment/coercion such as:

```text
owner-of(resource)
    ? entails
reader-of(resource)
```

or contextual interpretation of one visible role as another.

### CAUTION

That analogy is currently too early.

Before designing role hierarchies we must read the complete iyal and later commentary to understand:

- whether the source phenomenon is formal overlap, semantic overlap, governed alternation, interpretation from context, or a combination;
- how the source constrains acceptable variation;
- when ambiguity is resolved by meaning rather than form.

### AYTHAM CONSEQUENCE

`role coercion` remains **HOLD**.

Potential future experiment:

> Can Aytham permit a relation to satisfy another required relation only through an explicit, inspectable entailment rule, never through silent “close enough” coercion?

---

# 4. வினையியல் — `vinai` is not simply “effectful function”

The opening source says, approximately:

- வினை does not take case;
- when considered, it appears with time;
- three times/tenses are then distinguished (2.6, nūṟpā 1–3).

### SOURCE OBSERVATION

Time/tense is structurally central immediately in the source definition.

### INTERPRETATION CAUTION

Aytham's provisional modern structure:

```text
vinai
  requires
  establishes
  preserves
  invalidates
  effects
  capabilities
  produces
```

is **not** what the source defines.

Calling an arbitrary pure/effectful graph transformation `vinai` is a modern design borrowing.

### AYTHAM CONSEQUENCE

Two changes follow.

#### A. Keep the term provisional

`vinai` remains a good conceptual candidate for “action/transformation,” but must be labelled AYTHAM DESIGN in the specification.

#### B. Add temporal semantics to the research agenda

Experiment 002 already introduced freshness/expiry:

```text
verified_at(T)
valid_until(T2)
```

The source-level prominence of time suggests a legitimate **research question**, not a direct mapping:

> Should an Aytham action's temporal relationship to its established facts be explicit in the semantic graph?

For example:

```text
VerifyOwnership
  establishes ownership_verified
  at T
  valid_until T + 30d
```

This overlaps heavily with temporal logic, effect systems, event sourcing, and distributed-systems validity models and needs non-Tamil prior-art comparison.

---

# 5. இடையியல் — stronger source support for “mediating relation” than for generic control flow

The opening says that `idai` occurs/operates with `peyar` and `vinai` and does not have the same independent nature; the following source lines enumerate functions including assistance in meaning at combination boundaries, temporal relation to action, case-related meaning, rhythmic/phonological functions, and contextual meaning (2.7, opening nūṟpās).

### SOURCE OBSERVATION

`idai` is relational and dependent on surrounding forms.

That is more specific than saying:

```text
idai = if / pipe / semicolon / connector
```

### AYTHAM CONSEQUENCE

Revise the Aytham hypothesis:

> `idai` should be investigated as **semantic mediation**, not as a generic bucket for control-flow syntax.

Possible modern questions:

- What relation makes two actions sequential rather than merely adjacent?
- What relation carries evidence from one transformation to another?
- What relation chooses a success/failure continuation?
- Can a connector itself contribute temporal, role, or contextual meaning?

Aytham should only retain `idai` formally if a connector has semantic content independent of punctuation.

---

# 6. உரியியல் — major correction to Experiment 002 terminology

The opening of உரியியல் is richer than the convenient English label “qualifier.”

The first nūṟpā discusses `uriccol` arising through `isai`, `kurippu`, and `paṇpu`; it notes movement/indeterminacy around noun and verb behaviour, one word relating to multiple meanings, and multiple words relating to one meaning, with interpretation tied to usage/tradition/context. The second distinguishes what needs explanation from what is already explicit (2.8, nūṟpā 1–2).

### SOURCE OBSERVATION

This is **not equivalent to a refinement predicate**.

The source opening has strong concerns with:

```text
qualification / character
indication / contextual signification
lexical-semantic multiplicity
contextual interpretation
relationship to peyar/vinai
```

### CONSEQUENCE 1 — reopen `uri = refinement`

Experiment 002 currently uses `uri` for:

```text
positive
email_syntax_valid
ownership_verified
fresh_within(30d)
```

That can remain an Aytham experiment, but the formal term `uri` must be considered **REOPENED** until comparative source study is deeper.

### CONSEQUENCE 2 — a potentially richer research question

Instead of only:

> What predicate is true of this value?

Aytham could ask:

> What qualification/indication is valid for this semantic subject **in this context**, and how was that meaning resolved?

This might eventually connect:

- properties;
- contextual facts;
- evidence;
- meaning resolution;
- ambiguity.

But this is also much harder to formalize safely.

### CONSEQUENCE 3 — keep a neutral implementation name for now

In paper semantics, use:

```text
qualification claim
```

as the neutral English technical phrase.

`uri` can remain a research alias, not a frozen keyword or formal type-system name.

---

# 7. புணரியல் — composition inspiration is specifically boundary-sensitive

The opening of புணரியல் classifies what happens when forms meet at their **end and beginning boundaries**, beginning from properties of those boundary forms (1.4, opening nūṟpās).

### SOURCE OBSERVATION

The source is about linguistic/form combination and boundary behaviour.

### AYTHAM CONSEQUENCE

The useful transferable design question is more precise than generic “function composition”:

> When two individually meaningful computational forms meet, what **boundary contract** determines whether and how they may combine?

This strengthens Experiment 002's composition boundary:

```text
producer resolved meaning
        │
        │ boundary compatibility
        ▼
consumer required meaning
```

Possible boundary dimensions remain:

- role;
- qualification/evidence;
- effect;
- capability;
- temporal validity;
- protocol state;
- context.

### TERMINOLOGY CAUTION

`puṇarcci` / `punarcci` must be documented as **inspiration for checked joining**, not as a historical term for semantic composition.

---

# 8. எச்சவியல் — major correction: do not reduce the iyal to ellipsis

The structured `sections.json` gloss calls this iyal “Ellipsis” / “elliptical and residual constructions.”

However, direct source reading shows that the iyal opens by classifying words as:

```text
இயற்சொல்
திரிசொல்
திசைச்சொல்
வடசொல்
```

before proceeding into a broad range of remaining word-grammar matters. Later source material includes `பெயரெஞ்சு` and `வினையெஞ்சு` constructions and other `எச்ச` usages.

### SOURCE OBSERVATION

The one-word research label **ellipsis** is insufficient for safely deriving a programming concept from the whole iyal.

### AYTHAM CONSEQUENCE

Withdraw the early priority claim:

```text
Eccaviyal → implicit/recoverable omission
```

Replace with:

> Complete a full source/commentary study of Eccaviyal before extracting any Aytham hypothesis.

If omission/inference eventually survives as one sub-theme, compare it against:

- implicit arguments;
- type inference;
- elaboration;
- bidirectional typing;
- hole-driven development;
- contextual resolution.

But **no Eccaviyal-inspired inference feature is active now**.

---

# 9. Cross-cutting result

The direct source reading pushes Aytham away from a flat renamed type system and toward a more cautious relational interpretation:

```text
peyar / vinai
   central forms/perspectives

idai
   mediates with surrounding forms

uri
   contextual qualification/signification; not safely reducible to predicate

vetrumai
   grammatical relation marking; inspires but does not define Aytham roles

punarcci
   boundary-sensitive joining; inspires but does not define software composition
```

This is **more interesting**, but also means Aytham should delay formal Tamil terminology until the source/comparative grammar phase is stronger.

---

# 10. Immediate next source notebooks

## Notebook 002 — complete Sollatikāram high-priority read

Read and classify source passages from:

1. கிளவியாக்கம்
2. வேற்றுமையியல்
3. வேற்றுமை மயங்கியல்
4. பெயரியல்
5. வினையியல்
6. இடையியல்
7. உரியியல்
8. எச்சவியல்

Output per passage:

```text
source reference
minimal source paraphrase
commentary questions
possible Aytham question
forced-analogy risk
status
```

## Notebook 003 — Eḻuttatikāram boundary/composition read

Focus:

- புணரியல்;
- தொகை மரபு;
- உருபியல்;
- உயிர்/புள்ளி மயங்கியல்;
- குற்றியலுகரப் புணரியல்.

Research goal:

> Understand how the grammar structures boundary rules, general rules, contextual rules, and exceptions—without converting the phonological rules themselves into programming semantics.

## Notebook 004 — Poruḷatikāram anti-analogy study

Before borrowing `poruḷ`, `tiṇai`, `marabu`, or related concepts, document what they actually do in the source and what they **must not** be made to mean in Aytham.

Sangam literature becomes especially useful at this stage.

---

# 11. Nannūl request produced by this notebook

Nannūl is now needed for a concrete reason:

> We need to test whether our understanding of relation, word class, qualification, residual constructions, and joining remains stable or is clarified/reorganized in later Tamil grammatical tradition.

Priority comparison targets:

```text
வேற்றுமை
பெயர்
வினை
இடை
உரி-related treatment
எச்சம்
புணர்ச்சி
```

No Aytham Tamil keyword should be frozen before that comparative pass.