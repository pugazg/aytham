# Aytham Research Questions

Status: **active research agenda**

Aytham is not yet a programming-language specification. These questions must be answered before syntax and implementation are frozen.

## RQ1 — What is genuinely new?

Can Tamil grammatical thought produce a programming abstraction that is more than:

- Tamil keywords;
- Tamil identifiers;
- renamed compiler stages;
- renamed functions/types/objects;
- conventional syntax with Tamil vocabulary?

### Falsification test

If an Aytham feature can be translated back into an ordinary language without losing any important semantics, safety, or reasoning benefit, it may not be conceptually distinctive enough to justify inclusion.

---

## RQ2 — Can semantic roles be first-class?

Can a **வேற்றுமை-inspired role system** make APIs safer and clearer than positional/named parameters alone?

### Prototype questions

- Can two values of the same underlying type carry incompatible roles?
- Can role compatibility participate in overload resolution?
- Can roles survive through transformations?
- Can roles be inferred safely?
- Can roles be polymorphic?
- Can a role change only through an explicit transformation?

### Minimum demonstration

A transfer/payment example must statically prevent source/destination reversal without requiring separate wrapper classes written by the programmer.

---

## RQ3 — Can qualification be separated from nominal type?

Can an **உரி-inspired refinement model** represent constraints such as:

- positive number;
- non-empty text;
- verified email;
- authenticated user;
- metres vs seconds;
- read-only resource;
- network-capable action;

without forcing every property into a conventional nominal type hierarchy?

### Falsification test

If `uri` is merely syntax sugar over standard refinement types/contracts with no improved composition or diagnostics, its role in the language must be reconsidered.

---

## RQ4 — Can action/effect be a primary semantic distinction?

Can a **வினை-inspired model** make the difference between:

- computing a value;
- changing state;
- performing I/O;
- crossing a trust boundary;
- consuming a resource;
- transitioning a protocol state

clearer than conventional function syntax?

### Prototype questions

- Should every vinai declare effects?
- Can effects be inferred?
- Can pure vinai compose differently from effectful vinai?
- Can tooling visualize what a vinai changes?

---

## RQ5 — Can composition be checked as a relationship?

Can a **புணர்ச்சி-inspired composition model** express compatibility across:

- input/output meaning;
- semantic roles;
- constraints;
- effects;
- protocol states;
- capabilities?

### Minimum demonstration

Two individually valid transformations should fail to compose when their semantic relationship is invalid, and the compiler should explain the mismatch in domain terms rather than only report a type error.

---

## RQ6 — What should `peyar / vinai / idai / uri` mean formally?

The terms must not become decorative categories.

Questions:

- Are these mutually exclusive semantic classes?
- Can a form belong to multiple categories?
- Is `uri` itself a value?
- Is `idai` syntax, semantics, or both?
- Can `vinai` be passed/stored as a `peyar`?
- If yes, what distinction remains meaningful?

A formal model may reveal that some categories should be teaching/diagnostic concepts rather than runtime categories.

---

## RQ7 — What does `poruḷ` mean in Aytham?

Aytham must not conflate Poruḷatikāram with modern denotational/operational semantics.

Research question:

> Is `poruḷ` a useful Aytham term for a program form's resolved contextual meaning, or does that borrowing create more historical confusion than value?

Possible resolved meaning dimensions:

- value;
- category;
- role;
- refinement;
- effect;
- capability;
- state;
- relation;
- context.

This terminology remains reversible until source review is complete.

---

## RQ8 — How Tamil-specific should the surface language be?

Possible models:

1. Tamil-only keywords, Tamil/English identifiers;
2. Tamil semantic core with bilingual surface aliases;
3. symbolic/minimal keyword language with Tamil conceptual documentation;
4. one semantic language with multiple localized surfaces.

The project must avoid making English mandatory for Tamil users while also avoiding ecosystem isolation.

---

## RQ9 — How should Unicode identity work?

Questions:

- What normalization form is canonical in source files?
- Are canonically equivalent identifiers identical?
- How are combining sequences diagnosed?
- What counts as one source character for columns/error spans?
- How are Tamil numerals treated?
- Are visually confusable Latin/Tamil identifiers warned about?
- Can identifiers mix scripts?

This should be specified before tooling is built.

---

## RQ10 — What programming paradigm best fits the model?

Do not assume imperative, functional, object-oriented, or logic programming in advance.

Compare the research model against:

- expression-oriented languages;
- algebraic data types and pattern matching;
- refinement/dependent typing;
- effect systems;
- capability systems;
- dataflow/pipeline languages;
- logic/rule languages;
- actor/message systems;
- protocol/session types;
- concatenative/compositional languages.

The goal is to understand where Aytham is actually different and where established theory already provides the same abstraction.

---

## RQ11 — What is the smallest useful semantic core?

Before a compiler, define a paper model that can express:

1. literal/value;
2. denotation/binding;
3. transformation;
4. role relationship;
5. refinement/constraint;
6. composition;
7. failure/error;
8. observable effect.

If this cannot be explained without conventional syntax, keep researching.

---

## RQ12 — Can Aytham's errors be meaning-oriented?

Aytham should test whether its semantic model improves diagnostics.

Instead of:

```text
Type mismatch: expected Account, got Account
```

A role-aware diagnostic might say conceptually:

```text
This Account is acting as the source of the transfer,
but this position requires the destination role.
```

Instead of:

```text
Cannot compose F with G
```

A composition diagnostic might explain:

```text
The first action produces unverified text.
The next action requires text with the `verified` qualification.
```

Diagnostic quality is a primary evaluation criterion, not polish added later.

---

## RQ13 — How do we prove usefulness?

Aytham should eventually implement the same small programs in:

- Aytham;
- Rust;
- TypeScript;
- Python;
- another language with a strong type/effect model where relevant.

Compare:

- lines/ceremony;
- invalid states prevented;
- quality of error messages;
- refactoring safety;
- API readability;
- composition complexity;
- runtime cost.

Aytham should survive comparison, not only look culturally distinctive.

---

## RQ14 — What evidence is needed before a novelty claim?

Before using language such as "first" or "unique":

- complete prior-art review;
- search Tamil computing conference archives;
- search theses/dissertations;
- search code/package ecosystems;
- compare against non-Tamil programming-language theory;
- record overlapping concepts honestly;
- obtain specialist review where appropriate.

Aytham can still be worthwhile even if some ideas have predecessors.

---

# Current research order

1. வேற்றுமை-inspired roles
2. உரி-inspired refinements
3. வினை + explicit effects
4. புணர்ச்சி-inspired composition
5. formal relationship among பெயர் / வினை / இடை / உரி
6. Unicode/eḻuttu model
7. only then surface syntax
