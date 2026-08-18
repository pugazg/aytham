# Tolkāppiyam — All 27 Iyal Survey for Aytham

Status: **first-pass research survey**  
Source structure: `pugazg/tolkappiyam-arivagam`, derived from Project Madurai `pmuni0100`.

## Purpose

Aytham initially concentrated on a small set of attractive concepts: `வேற்றுமை`, `பெயர்`, `வினை`, `இடை`, `உரி`, and `புணர்ச்சி`.

That creates a selection-bias risk: we might notice only the parts of Tolkāppiyam that resemble familiar programming concepts.

This document therefore surveys **all 3 அதிகாரங்கள் × 9 இயல்கள் = 27 இயல்கள்**, including those that currently appear unsuitable for core programming-language design.

For every iyal we ask:

1. What is its source domain in the current structured corpus?
2. Is there a computational research question worth asking?
3. What is the forced-analogy risk?
4. Should it be deep-studied now, later, or held out of the core language?

No row below claims that a modern programming construct exists in Tolkāppiyam.

Evidence labels remain:

- **SOURCE** — source/structured-corpus description;
- **INTERPRETATION** — modern reading/question;
- **AYTHAM DESIGN** — any programming-language invention inspired by it.

---

# I. எழுத்ததிகாரம் · Eḻuttatikāram

The structured corpus describes this book as treating letters/sounds, their production and classification, and rules by which forms combine.

## 1. நூல் மரபு · Nūl marapu

**SOURCE focus:** inventory of Tamil letters and length/classification.

**Research question:** what should Aytham consider a stable source-language unit in Unicode text?

Possible areas:

- grapheme-aware diagnostics;
- canonical Unicode identity;
- Tamil/Latin confusable handling;
- literal and identifier segmentation;
- source-column calculation by grapheme rather than byte.

**Assessment:** useful engineering foundation; unlikely to be a semantic differentiator.

**Priority:** MEDIUM — Unicode/source-form notebook.

---

## 2. மொழி மரபு · Moḻi marapu

**SOURCE focus:** conventions governing how letters occur at word boundaries and word-forming behaviour.

**Research question:** should Aytham have explicit source-form conventions (`marabu`) for identifiers/literals/modules rather than treating all Unicode strings as equally well-formed source names?

Potential benefit:

- domain-specific naming conventions that are compiler-checkable;
- safer mixed-script APIs;
- canonical external-name mapping.

Risk: style rules disguised as language semantics.

**Priority:** MEDIUM.

---

## 3. பிறப்பியல் · Piṟappiyal

**SOURCE focus:** production/origin of sounds in the vocal tract.

This is valuable linguistic knowledge but currently has **no credible core programming abstraction**.

Possible non-core uses later:

- speech/voice-oriented Aytham tooling;
- pronunciation-aware identifiers or teaching tools;
- Tamil source input/accessibility.

A mapping such as “sound origin = value origin/provenance” would be a forced analogy and is rejected.

**Priority:** HOLD for core semantics; retain for Tamil tooling research.

---

## 4. புணரியல் · Puṇariyal

**SOURCE focus:** general rules by which forms join/meet.

**Research question:** can Aytham make **semantic composition boundaries** first-class and check more than ordinary input/output type equality?

Candidate boundary dimensions:

```text
role
qualification/evidence
effect
capability
state/protocol
lineage
context
```

This remains one of Aytham's strongest inspirations, but the modern operation is **AYTHAM DESIGN**, not historical sandhi translated into code.

**Priority:** VERY HIGH — active paper semantics.

---

## 5. தொகை மரபு · Tokai marapu

**SOURCE focus:** combination/grouping, including compounds and numeral/measure contexts.

**Research question:** can aggregates carry a declared **grouping invariant** beyond `Collection<T>`?

Examples worth testing later:

```text
all elements share currency
all resources share ownership scope
all tasks are dependency-compatible
all measurements share dimension
collection is complete for a declared role-set
```

This overlaps strongly with refinement/dependent collection types.

**Priority:** MEDIUM-LATER.

---

## 6. உருபியல் · Urupiyal

**SOURCE focus:** forms of case suffixes and connective increments.

**Research question:** if Aytham's semantic graph has role relations, can the eventual surface syntax expose those relations using compact, visible markers?

Important separation:

```text
semantic role system != Tamil case morphology
```

Actual Tamil morphological imitation should not be frozen without linguistic review and usability testing.

**Priority:** SOURCE STUDY NOW; syntax application LATER.

---

## 7. உயிர் மயங்கியல் · Uyir mayaṅkiyal

**SOURCE focus:** combination rules involving vowel-ending forms.

This is a specialized phonological/sandhi domain.

Possible Aytham relevance:

- excellent test corpus for Unicode/source diagnostics;
- possible inspiration for **context-sensitive boundary rules**, but only as a general lesson, not as direct programming semantics.

Do not invent `uyir` types or computational vowel classes merely for branding.

**Priority:** LOW for semantics; MEDIUM for Tamil source tooling tests.

---

## 8. புள்ளி மயங்கியல் · Puḷḷi mayaṅkiyal

**SOURCE focus:** consonant-ending combination rules.

Same conclusion as Uyir mayaṅkiyal:

- valuable for Tamil text correctness/tooling;
- weak justification for core programming abstractions.

The broader lesson that *boundary behaviour depends on properties of both participants* is already captured in the `punarcci` composition research without copying phonological categories.

**Priority:** LOW for core semantics.

---

## 9. குற்றியலுகரப் புணரியல் · Kuṟṟiyalukara puṇariyal

**SOURCE focus:** specialized combination behaviour involving குற்றியலுகரம்.

This is an important warning for Aytham design methodology:

> A general rule system may require narrowly defined exceptional/contextual cases.

But Aytham should not turn this specific phonological phenomenon into a programming feature.

Potential engineering use:

- Tamil token/grapheme test suite;
- diagnostics around source normalization and contextual Tamil forms.

**Priority:** HOLD for semantics.

---

# II. சொல்லதிகாரம் · Collatikāram

This is currently the richest source family for Aytham because the structured corpus explicitly treats expression formation, case, nouns, verbs, particles, qualifiers, and residual/elliptical constructions.

## 10. கிளவியாக்கம் · Kiḷaviyākkam

**SOURCE focus:** formation and classification of words/expressions.

**Research question:** should Aytham be **expression/relationship-centric rather than statement-centric**?

Candidate principle:

> Every well-formed computational form resolves to an inspectable semantic record/graph; declarations, transformations, choices, and compositions are not unrelated statement species.

Expression-oriented languages already exist, so this is not independently novel.

**Priority:** HIGH.

---

## 11. வேற்றுமையியல் · Vēṟṟumaiyiyal

**SOURCE focus:** grammatical case relations/functions.

**Research question:** can **semantic role relations** be first-class and statically checked independently of the underlying value type?

Example:

```text
Account A --source-of------> Transfer
Account B --destination-of-> Transfer
```

The role belongs to the relationship, not permanently to `Account A` or `Account B`.

Closest comparisons include named arguments, semantic roles, row/record typing, dependent relations, capability types, and graph models.

**Priority:** HIGHEST — active Experiment 001.

---

## 12. வேற்றுமை மயங்கியல் · Vēṟṟumai mayaṅkiyal

**SOURCE focus:** the structured corpus glosses this as **case overlap — where one case form does the work of another**.

This is more interesting for Aytham than previously recognized.

**Research questions:**

- Can one surface relation legitimately satisfy another semantic role in a constrained context?
- When is role coercion/subsumption valid?
- Can the compiler explain an ambiguous relation rather than relying on positional guessing?
- Should roles form a hierarchy/lattice, or would that create dangerous implicit conversions?

Example research case:

```text
owner-of Resource
```

might imply some permissions but must not silently imply all roles such as:

```text
deleter-of Resource
```

This could become a strong test for **controlled role entailment**, but direct analogy is risky.

**Priority:** HIGH SOURCE STUDY; no feature yet.

---

## 13. விளி மரபு · Viḷi marapu

**SOURCE focus:** vocative forms / forms of address.

Possible computational question:

> Is explicit addressing of an entity/action meaningfully distinct from merely naming it?

Potential future domains:

- actor/message systems;
- service/resource addressing;
- capability-directed invocation.

However, mapping vocative grammar directly to method invocation would be shallow.

**Priority:** LOW-MEDIUM / HOLD until an actual addressing problem appears.

---

## 14. பெயரியல் · Peyariyal

**SOURCE focus:** noun-class material.

**Research question:** what counts as a computational **denotable entity** (`peyar`)?

Potential category includes:

- ordinary values;
- bindings;
- resources;
- capabilities;
- types/type-level entities;
- semantic subjects carrying claims.

Critical formal question:

> If a `vinai` can itself be named, stored, passed, and returned, is `peyar` a semantic category or only a role/teaching perspective?

**Priority:** HIGH.

---

## 15. வினையியல் · Vinaiyiyal

**SOURCE focus:** verb-class material.

**Research question:** can `vinai` be modeled not merely as “function” but as an explicit **semantic transformation contract**?

Current Experiment 002 candidate:

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

This overlaps with functions, effect systems, Hoare-style contracts, typestate transitions, and graph rewriting.

**Priority:** HIGHEST.

---

## 16. இடையியல் · Idaiyiyal

**SOURCE focus:** particle/connective class.

**Research question:** what connects computational meanings?

Possible `idai` research space:

- sequencing;
- dependency;
- branching;
- synchronization;
- composition;
- fallback/alternative;
- causality.

Risk: simply renaming operators/combinators/control flow.

The key test is whether `idai` represents a **semantic relationship visible in the graph**, rather than syntax punctuation.

**Priority:** HIGH, but under-defined.

---

## 17. உரியியல் · Uriyiyal

**SOURCE focus:** qualifier-class material.

**Research question:** can qualification be separated from nominal identity and become a first-class relation/fact?

After Experiment 002, the strongest provisional form is:

```text
UriClaim
  subject
  predicate
  established_by
  evidence/authority (when relevant)
  scope
  validity/freshness
  preservation law
```

This must remain lightweight for ordinary static properties.

Refinement types, proofs, provenance, and typestate cover much of the ingredient space; Aytham's test is unified usability and composition.

**Priority:** HIGHEST — active Experiment 002.

---

## 18. எச்சவியல் · Eccaviyal

**SOURCE focus:** the structured corpus glosses it as **elliptical and residual constructions**.

Potential research question:

> When can omitted computational information be reconstructed safely from semantic context?

Possible areas to compare:

- implicit arguments;
- type inference;
- defaulting;
- hole-driven development;
- bidirectional typing;
- context-based role inference.

Aytham must not equate Tamil ellipsis with compiler inference. The useful research idea is **recoverability with explicit ambiguity diagnostics**.

Example future test:

```text
transfer A B 100
```

Could roles be inferred from surrounding semantic evidence? If more than one valid graph exists, compilation should require explicit disambiguation.

**Priority:** MEDIUM-HIGH SOURCE STUDY; HOLD as language feature.

---

# III. பொருளதிகாரம் · Poruḷatikāram

The structured corpus correctly treats this as poetics/subject matter: akam/puram convention, phases of love, embodied feeling, simile, prosody, and literary convention.

This book has the **highest forced-analogy risk** for a programming-language project. It should therefore be studied broadly but borrowed from sparingly.

## 19. அகத்திணையியல் · Akattiṇaiyiyal

**SOURCE focus:** interior/love genre and associated landscapes/conventions.

Potential research lesson:

> Meaning and valid action may depend on an explicit context/domain.

This might eventually inspire context-sensitive capabilities or domain protocols, but `tiṇai = execution context` is currently too forced.

Sangam literature would be useful later for understanding how tiṇai operates in actual texts before any computational borrowing.

**Priority:** SOURCE STUDY; CORE FEATURE HOLD.

---

## 20. புறத்திணையியல் · Puṟattiṇaiyiyal

**SOURCE focus:** exterior/public genre and its conventional situations.

Same caution as Akattiṇaiyiyal.

Aytham should not make an `akam/puram` public/private visibility system; that would be superficial.

Possible later question:

- can context categories constrain valid operations and interpretations?

But this should be motivated by a real programming problem first.

**Priority:** HOLD.

---

## 21. களவியல் · Kaḷaviyal

**SOURCE focus:** clandestine/pre-marital phase of love.

Tempting analogies to protocol phase/state are rejected at this stage.

Its value to Aytham may instead be methodological: Tolkāppiyam models **situations with participants, context, conventions, permitted actions, transitions, and expectations**. We should understand that system historically before deciding whether any general relational principle transfers.

**Priority:** SOURCE STUDY only; no computational mapping.

---

## 22. கற்பியல் · Kaṟpiyal

**SOURCE focus:** subsequent/marital phase and associated conventions.

As with Kaḷaviyal, using it as a typestate transition would be a forced cultural analogy.

Potential research value:

- comparative study of how Tamil grammar/literary theory represents phase-dependent relations.

No core Aytham feature currently justified.

**Priority:** HOLD.

---

## 23. பொருளியல் · Poruḷiyal

**SOURCE focus:** the structured corpus glosses it as **general principles unifying the treatment of poetic meaning**.

This is relevant to our terminology decision, but dangerous.

Aytham currently uses `poruḷ` provisionally for a resolved semantic record/graph. Before freezing that term we must answer:

- Does this modern borrowing illuminate or distort?
- Is `poruḷ` better used only in documentation rather than formal grammar?
- Would `meaning graph`, `semantic graph`, or another Tamil term be historically cleaner?

**Priority:** HIGH TERMINOLOGY/SOURCE REVIEW; no historical equivalence claim.

---

## 24. மெய்ப்பாட்டியல் · Meyppāṭṭiyal

**SOURCE focus:** outward bodily expression of emotion.

Potential computational analogy such as “internal state → observable output” is obvious but currently too generic.

Possible later research domain:

- observability: what externally visible evidence permits inference about hidden state?
- UI/reactive diagnostics.

But established program semantics and observation theory already cover this broadly; no Tamil-specific feature is justified.

**Priority:** LOW / HOLD.

---

## 25. உவமவியல் · Uvamaiyiyal

**SOURCE focus:** simile/comparison.

Potential programming temptations:

- structural similarity;
- pattern matching;
- analogical programming;
- coercion by resemblance.

All are currently too speculative.

One potentially useful future research topic is **explainability by analogy** in tooling, not core semantics.

**Priority:** LOW.

---

## 26. செய்யுளியல் · Ceyyuḷiyal

**SOURCE focus:** metre and elements of verse composition; the largest iyal in the structured corpus.

Potential research lesson:

> A composed artifact can be valid only when multiple structural constraints hold simultaneously across different scales.

Possible later applications:

- declarative constrained composition;
- DSL grammars;
- formatter/layout systems;
- verified structured documents;
- compile-time shape constraints.

But ordinary grammar/type systems already provide many related ideas. This is better suited to a future Aytham **domain example** than the core language.

**Priority:** MEDIUM-LATER.

---

## 27. மரபியல் · Marapiyal

**SOURCE focus:** poetic convention, correct usage, naming, and tradition.

Potential research question:

> Can a program/module declare machine-checkable **conventions that govern meaning across a boundary** without baking them into the universal language core?

Possible modern Aytham design areas:

- module/API conventions;
- domain schemas;
- protocol policies;
- naming/representation contracts;
- versioned interoperability conventions.

This could become a way to keep Aytham's core small while allowing domains to publish semantic rules.

However, “marabu = module policy” is not yet justified historically or computationally.

**Priority:** MEDIUM-HIGH research, especially after Nannūl comparison.

---

# IV. Priority result after all 27

## Tier A — active core hypotheses

These deserve deep source notebooks and programming experiments now:

1. **புணரியல்** — checked composition boundaries;
2. **வேற்றுமையியல்** — semantic role relations;
3. **வினையியல்** — transformation/effect contracts;
4. **உரியியல்** — qualification/evidence relations;
5. **கிளவியாக்கம்** — formation of computational expressions/relations;
6. **பெயரியல்** — denotable semantic subjects/entities;
7. **இடையியல்** — semantic connectors/dependencies.

## Tier B — newly important research candidates

8. **வேற்றுமை மயங்கியல்** — controlled role overlap/entailment/ambiguity;
9. **எச்சவியல்** — recoverable omission/inference and ambiguity;
10. **உருபியல்** — possible surface realization of relationships;
11. **பொருளியல்** — terminology/contextual-meaning review;
12. **மரபியல்** — convention/policy systems.

## Tier C — useful supporting engineering/domain research

13. நூல் மரபு
14. மொழி மரபு
15. தொகை மரபு
16. செய்யுளியல்
17. உயிர் மயங்கியல்
18. புள்ளி மயங்கியல்
19. குற்றியலுகரப் புணரியல்

## Tier D — source study, no current core mapping

20. பிறப்பியல்
21. விளி மரபு
22. அகத்திணையியல்
23. புறத்திணையியல்
24. களவியல்
25. கற்பியல்
26. மெய்ப்பாட்டியல்
27. உவமவியல்

Tier D does **not** mean unimportant in Tamil scholarship. It means Aytham currently lacks a non-forced programming problem that they help solve.

---

# V. Revised immediate source-study order

The next deep notebooks should be created in this order:

```text
1. உரியியல்
2. வினையியல்
3. புணரியல்
4. வேற்றுமையியல்
5. வேற்றுமை மயங்கியல்
6. இடையியல்
7. எச்சவியல்
8. பெயரியல்
9. கிளவியாக்கம்
10. உருபியல்
```

`பொருளியல்` and `மரபியல்` should be studied alongside them as terminology/context checks, not as assumed compiler constructs.

---

# VI. What the complete survey changes

The project should no longer describe its inspiration only as:

```text
எழுத்து → சொல் → பொருள்
```

That three-part frame is too coarse.

The stronger research space is:

```text
form
  │
  ├─ formation / கிளவியாக்கம்
  ├─ entity / பெயர்
  ├─ action / வினை
  ├─ relation-role / வேற்றுமை
  ├─ role overlap / வேற்றுமை மயக்கம்
  ├─ qualification / உரி
  ├─ connection / இடை
  ├─ recoverable omission / எச்சம் ?
  ├─ composition boundary / புணர்ச்சி
  └─ convention / மரபு ?
        ↓
resolved contextual computational meaning
```

Question marks indicate especially provisional Aytham borrowings.

The aim is not to reproduce Tolkāppiyam inside a compiler. The aim is to let the **full structure of Tamil grammatical thought challenge which relationships a programming language chooses to make explicit**.