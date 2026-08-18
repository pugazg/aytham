# Tolkāppiyam → Aytham Concept Map

Status: **research map; no direct equivalence claims**

## Purpose

This document prevents two opposite mistakes:

1. using Tamil grammatical terminology decoratively without allowing it to change the programming model;
2. falsely claiming that modern compiler/programming concepts are literally present in Tolkāppiyam.

For each concept we therefore record:

- **SOURCE DOMAIN** — what the source/repository describes;
- **RESEARCH QUESTION** — what Aytham wants to investigate;
- **AYTHAM DESIGN HYPOTHESIS** — a modern invention inspired by the source;
- **STATUS** — whether the idea is merely interesting or strong enough to prototype.

## Structural source

The current source map is grounded initially in the structured Tolkāppiyam dataset maintained in `pugazg/tolkappiyam-arivagam`, itself derived from the Project Madurai `pmuni0100` electronic text.

The dataset represents three major divisions:

- **எழுத்ததிகாரம் · Eḻuttatikāram** — letters/sounds, their production, classification, and combination;
- **சொல்லதிகாரம் · Collatikāram** — words, expression formation, case, inflection, nouns, verbs, particles, qualifiers;
- **பொருளதிகாரம் · Poruḷatikāram** — subject matter, akam/puram conventions, poetic/literary structures, embodied feeling, simile, prosody, tradition.

Aytham must preserve this distinction. In particular, **Poruḷatikāram is not simply a historical equivalent of a modern compiler semantic-analysis phase.**

---

# A. எழுத்ததிகாரம் research family

## A1. எழுத்து · Eḻuttu

### SOURCE DOMAIN

Letters/sounds, their classification, form, length, production, and behaviour in combination.

### RESEARCH QUESTION

What should count as an atomic source-language unit in a Unicode-native Tamil-first programming language?

### AYTHAM DESIGN HYPOTHESIS

Aytham could expose a first-class **source-form model** in which:

- Tamil grapheme identity is stable and inspectable;
- canonical-equivalence problems are diagnosed explicitly;
- literals and identifiers operate on semantic characters/graphemes rather than accidental byte/code-point assumptions;
- compiler diagnostics can explain malformed Tamil text precisely.

### STATUS

**Prototype candidate.** Technically useful, but not sufficient for conceptual originality by itself.

---

## A2. மொழி மரபு · Moḻi marabu

### SOURCE DOMAIN

The structured dataset describes conventions concerning how letters begin/end words and word-forming behaviour.

### RESEARCH QUESTION

Should Aytham make lexical well-formedness and identifier composition configurable or rule-governed rather than arbitrary?

### AYTHAM DESIGN HYPOTHESIS

A module may declare a machine-checkable **source convention** governing identifier/literal forms for a domain.

Example possibilities:

- financial identifiers follow one convention;
- scientific units another;
- generated code can declare a stricter lexical marabu.

### STATUS

**Exploratory.** Risk of over-design.

---

## A3. புணரியல் · Puṇariyal

### SOURCE DOMAIN

Rules governing joining/combination when forms meet.

### RESEARCH QUESTION

Can composition itself be made a checked semantic operation rather than relying only on function compatibility and operator overloading?

### AYTHAM DESIGN HYPOTHESIS

Define **composition contracts** between computational forms. When `A` and `B` combine, the compiler evaluates not only type compatibility but also:

- semantic role;
- refinement/constraint;
- effects;
- state/protocol transition;
- capabilities;
- declared composition law.

A failed composition should explain *why these forms cannot meet*.

### STATUS

**High-priority prototype candidate.** One of the most promising paths to a distinctive model.

---

## A4. தொகை மரபு · Tokai marabu

### SOURCE DOMAIN

The structured dataset describes combination/grouping, including compounds and numeral/measure contexts.

### RESEARCH QUESTION

Can collection/group formation carry semantic invariants rather than merely element types?

### AYTHAM DESIGN HYPOTHESIS

A collection might preserve a **grouping law**:

- homogeneous by type;
- homogeneous by semantic role;
- dimensionally compatible;
- ordered by a declared relation;
- bounded by a constraint.

### STATUS

**Exploratory.** Revisit after core role/refinement system exists.

---

## A5. உருபியல் · Urupiyal

### SOURCE DOMAIN

The structured dataset describes forms of case suffixes and connective increments.

### RESEARCH QUESTION

Can visible markers attach computational role/relationship information to expressions without making APIs verbose?

### AYTHAM DESIGN HYPOTHESIS

Aytham may eventually experiment with **role markers** that participate in type checking. These must not be literal imitation of Tamil morphology unless linguistically justified.

### STATUS

**Research required before syntax design.** Do not freeze terminology.

---

# B. சொல்லதிகாரம் research family

## B1. கிளவியாக்கம் · Kiḷaviyākkam

### SOURCE DOMAIN

The structured dataset describes formation/classification of words and expressions.

### RESEARCH QUESTION

Should Aytham define programs in terms of **formed expressions** rather than a statement-centric grammar?

### AYTHAM DESIGN HYPOTHESIS

Every valid top-level construct could be an expression with a resolved `poruḷ`, allowing declarations, branches, transformations, and composition to participate in a common semantic model.

### STATUS

**Strong candidate**, though expression-oriented languages already exist; the distinction must come from the category/role system.

---

## B2. வேற்றுமை · Vēṟṟumai

### SOURCE DOMAIN

Case relations and their grammatical functions.

### RESEARCH QUESTION

Can API arguments and values carry **semantic roles** as part of static meaning, instead of role being inferred primarily from parameter order/name?

### AYTHAM DESIGN HYPOTHESIS

A value has at least two separable dimensions:

```text
what it is        → value/type/refinement
what role it has  → relation in the current computation
```

For example, `Source<Account>` and `Destination<Account>` may have the same underlying data representation but incompatible roles.

Role information could support:

- argument-order independence;
- safer APIs;
- more precise diagnostics;
- relation-based overload resolution;
- protocol checking;
- domain modelling.

### STATUS

**Highest-priority prototype candidate.**

---

## B3. பெயர் · Peyar

### SOURCE DOMAIN

Noun-class material in the word grammar.

### RESEARCH QUESTION

What is the computational category of something that **denotes an entity/value**?

### AYTHAM DESIGN HYPOTHESIS

`peyar` is broader than "variable." It may include:

- values;
- bindings;
- named entities;
- capabilities;
- type-level entities;
- immutable resources.

### STATUS

**Core semantic category candidate.**

---

## B4. வினை · Vinai

### SOURCE DOMAIN

Verb-class material in the word grammar.

### RESEARCH QUESTION

Can transformations/actions be represented as a distinct semantic species with explicit effect behaviour?

### AYTHAM DESIGN HYPOTHESIS

A `vinai` describes a transformation and declares what changes, what remains invariant, and what effects occur.

Possible future distinction:

```text
pure vinai       : value → value
effectful vinai  : world/state interaction
transition vinai : state A → state B
```

The goal is not to rename `function`; it is to make **action/effect semantics** part of the language model.

### STATUS

**Core semantic category candidate.**

---

## B5. இடை · Idai

### SOURCE DOMAIN

Particle/connective-class material in the word grammar.

### RESEARCH QUESTION

Can connection/control be represented explicitly rather than encoded as punctuation or hidden evaluation order?

### AYTHAM DESIGN HYPOTHESIS

`idai` may represent semantic connectors:

- sequencing;
- branching relation;
- composition;
- dependency;
- flow;
- synchronization.

### STATUS

**Promising but under-defined.** Requires comparison against combinators, arrows, pipes, monadic composition, and workflow languages.

---

## B6. உரி · Uri

### SOURCE DOMAIN

Qualifier-class material in the word grammar.

### RESEARCH QUESTION

Can constraints/properties refine computational meaning without forcing every distinction into a nominal type hierarchy?

### AYTHAM DESIGN HYPOTHESIS

`uri` could be the foundation for:

- refinements;
- contracts;
- predicates;
- units/ranges;
- permissions;
- capability restrictions;
- domain properties.

Conceptually:

```text
peyar + uri → a value constrained by a meaningful property
vinai + uri → an action constrained by an effect/property contract
```

### STATUS

**High-priority prototype candidate.**

---

# C. பொருளதிகாரம் research family

## C1. பொருள் · Poruḷ

### SOURCE DOMAIN

The historical/source domain concerns meaning/subject matter and extensive literary/poetic conventions; it is not reducible to modern compiler semantics.

### RESEARCH QUESTION

What does it mean for a computational form to have **resolved meaning in context**?

### AYTHAM DESIGN HYPOTHESIS

Aytham may use `poruḷ` as its own explicitly modern term for the resolved semantic record of an expression, provided documentation always marks this as an **Aytham design borrowing**, not a historical equivalence.

A possible poruḷ record:

```text
identity
value domain
semantic category (peyar/vinai/idai/uri/...)
role relations
constraints
effects
capabilities
state/protocol
context
```

### STATUS

**Useful umbrella concept, but high risk of historical overclaim.** Documentation discipline is mandatory.

---

## C2. திணை · Tiṇai

### SOURCE DOMAIN

Contextual/literary classification central to the poetics of akam/puram and associated situations/conventions.

### RESEARCH QUESTION

Can computation use explicit **context domains** where valid actions depend on the active context?

### AYTHAM DESIGN HYPOTHESIS

Possible future use for:

- capability domains;
- execution contexts;
- protocol environments;
- domain-specific sublanguages.

### STATUS

**Do not adopt yet.** This analogy is attractive but currently too easy to force. Requires deeper source study and a concrete programming problem first.

---

## C3. மெய்ப்பாடு, உவமை, செய்யுள், மரபு

These concepts are **not currently mapped to core programming constructs**.

Aytham should resist the temptation to exhaustively convert every Tolkāppiyam category into a language feature.

Possible future research may examine:

- `marabu` for machine-checkable conventions/protocols;
- pattern/representation ideas inspired by other categories;
- DSLs for literary/textual computation.

But no feature should exist simply because a source term is available.

---

# D. Current priority matrix

| Concept | Programming hypothesis | Priority | Originality potential | Risk |
|---|---|---:|---:|---:|
| எழுத்து | Unicode/source-form semantics | Medium | Medium | Low |
| புணர்ச்சி | checked composition laws | **High** | **High** | Medium |
| வேற்றுமை | role-aware relationships | **Highest** | **High** | Medium |
| பெயர் | denotable entities | High | Medium | Medium |
| வினை | transformations/effects | **High** | Medium-High | Medium |
| இடை | explicit semantic connectors | Medium | High | High |
| உரி | refinements/constraints | **High** | Medium-High | Medium |
| பொருள் | contextual semantic record | High | Medium | **High historical-risk** |
| மரபு | machine-checkable conventions | Medium | Medium | High |
| திணை | execution/context domains | Hold | Potentially high | **Very high forced-analogy risk** |

---

# E. First prototype target

Before choosing concrete Tamil syntax, build a **paper semantics** for one ordinary API problem:

```text
transfer(sourceAccount, destinationAccount, amount)
```

Aytham should demonstrate whether:

1. `sourceAccount` and `destinationAccount` can share the same underlying type while having statically distinct **வேற்றுமை-inspired roles**;
2. `amount` can carry `உரி` constraints such as positive/non-zero/currency;
3. the transfer operation is a `வினை` whose effects are explicit;
4. composition with a following operation is checked through a `புணர்ச்சி-inspired` compatibility rule;
5. the resulting `பொருள்` record explains the whole computation to tooling and diagnostics.

If this design is no safer, clearer, or more composable than a conventional typed implementation, Aytham must change direction before writing a compiler.

---

# F. Evidence discipline

Every future concept entry should carry one of these labels:

- **SOURCE** — directly supported by a source text or scholarly reference;
- **INTERPRETATION** — a modern explanatory reading;
- **AYTHAM DESIGN** — a new programming-language invention inspired by the source.

No Aytham feature should be retroactively attributed to Tolkāppiyam.
