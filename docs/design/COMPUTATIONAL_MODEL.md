# Aytham Computational Model — Research Draft 0.1

Status: **hypothesis, not language specification**

## 1. Purpose

Aytham is investigating whether Tamil grammatical thought can yield programming abstractions that are computationally useful in their own right.

The project deliberately rejects a shallow equivalence such as:

```text
எழுத்து = lexer
சொல்    = parser
பொருள்  = semantic analyser
```

That mapping may be useful as an explanatory analogy, but by itself it creates no new programming model.

The research goal is to find **behavioural consequences**.

---

## 2. Three-layer working hypothesis

### 2.1 எழுத்து · Eḻuttu — valid computational form

**Historical inspiration:** the structural treatment of letters/sounds and their relationships in Tamil grammar.

**Aytham design hypothesis:** computation begins with forms whose identity must be well-defined before syntax is interpreted.

Potential responsibilities:

- Unicode-normalized source representation;
- Tamil grapheme-aware diagnostics;
- literal forms;
- identifiers;
- operators/signs;
- compositional validity at the smallest source level;
- explicit distinction between visually similar and canonically equivalent Unicode sequences.

Aytham should not treat Tamil source text as an incidental UTF-8 byte stream.

**Test of distinctiveness:** Does an eḻuttu-aware model improve correctness, diagnostics, metaprogramming, or compositional rules beyond what a normal Unicode lexer would provide?

If not, this remains an implementation detail rather than a language feature.

---

### 2.2 சொல் · Sol — composable computational form

**Historical inspiration:** the grammatical treatment of words and word classes.

**Aytham design hypothesis:** a program is not primarily a sequence of statements; it is a composition of meaningful forms that belong to computational categories.

The first category experiment is inspired by the சொல்லதிகாரம் divisions:

#### பெயர் · Peyar

A form that **denotes** something.

Candidate computational meaning:

- value;
- binding;
- entity;
- immutable data;
- named capability;
- type-level entity.

The critical question is whether `peyar` should be broader than the conventional idea of a variable.

#### வினை · Vinai

A form that **does / transforms / changes** something.

Candidate computational meaning:

- pure transformation;
- state transition;
- effectful action;
- function-like behaviour;
- message/operation.

Aytham should investigate whether effects can be made first-class by distinguishing what a computation **is** from what it **does**.

#### இடை · Idai

A form that **connects or mediates** computational forms.

Candidate computational meaning:

- composition;
- control relationship;
- pipeline/flow;
- operator/combinator;
- dependency relation;
- sequencing policy.

If successful, control flow may be modelled as explicit composition rather than hidden statement order.

#### உரி · Uri

A form that **qualifies or constrains** another computational form.

Candidate computational meaning:

- refinement;
- predicate;
- property;
- constraint;
- contract;
- type qualifier;
- capability restriction.

A powerful possibility is that Aytham treats many "types" as `uri`: meaningful constraints on a `peyar` or `vinai`, rather than every type being a nominal container.

### Research test

These four categories are useful only if they change at least one of:

- what programs can express;
- what invalid programs the compiler can reject;
- how composition works;
- how effects are tracked;
- how programmers reason about code;
- how errors are explained.

Otherwise they are only renamed AST node classes and must be rejected.

---

### 2.3 பொருள் · Poruḷ — resolved meaning in context

**Historical caution:** `பொருள்` in Tolkāppiyam must not be casually equated with the modern compiler-science term "semantics." பொருளதிகாரம் has its own historical/literary scope and must be studied on its own terms.

**Aytham design hypothesis:** inspired by the broader idea that form alone is insufficient, Aytham gives every well-formed program element a context-resolved computational meaning.

A possible `poruḷ` record may include:

```text
identity
value domain
role
constraints
relations
effects
lifetime / ownership
capabilities
context
provenance
```

The distinguishing idea is that meaning may depend on **role and relation**, not only nominal type.

---

## 3. வேற்றுமை · Vēṟṟumai — role-aware relationships

This is one of the highest-value research hypotheses.

Conventional function calls often give meaning partly through **position**:

```text
transfer(accountA, accountB, 100)
```

The programmer must remember which position means source, destination, and amount.

Aytham will investigate whether Tamil case/role thinking can inspire a language in which arguments participate through **semantic roles** rather than only position.

Conceptual example only:

```text
transfer {
    from: accountA
    to: accountB
    amount: 100
}
```

This syntax is intentionally ordinary; the research question is deeper. Could role relations be part of the type system so that:

- argument order becomes irrelevant where roles are explicit;
- roles can be checked independently of value type;
- the same value type can carry different semantic roles;
- APIs become self-describing;
- diagnostics say which **relationship** is invalid, not merely which parameter position failed?

Aytham must study Tolkāppiyam's treatment of வேற்றுமை accurately before choosing terminology or formal rules.

**AYTHAM DESIGN:** any programming-language role system derived from this investigation will be a modern invention inspired by the source, not something attributed directly to Tolkāppiyam.

---

## 4. புணர்ச்சி · Puṇarcci — checked composition

A second high-value hypothesis is that program composition should not merely concatenate expressions or call functions.

Aytham can investigate **composition contracts**:

```text
A + B is valid only when their roles, constraints, and effects can legally combine.
```

Potential applications:

- pipeline composition;
- data transformation;
- unit-safe operations;
- effect compatibility;
- protocol/state-machine transitions;
- compile-time API composition;
- safe string/text operations.

The key research question is:

> Can a puṇarcci-inspired composition system provide a clearer and more general model than conventional operator overloading or ad-hoc type coercion?

Again, this is an Aytham design hypothesis, not a claim that historical புணர்ச்சி is programming-language composition.

---

## 5. மரபு · Marabu — explicit rules and conventions

Aytham will investigate `marabu` as a possible concept for **declared computational convention**.

Potential uses:

- protocol rules;
- module contracts;
- effect policies;
- API conventions;
- serialization rules;
- domain-specific constraints.

A `marabu` should be machine-checkable. If it is merely documentation, it is not part of the language model.

---

## 6. Proposed semantic shape

The current experimental picture is:

```text
                  ┌──────────────┐
                  │ எழுத்து       │
                  │ valid forms  │
                  └──────┬───────┘
                         │
                         ▼
                  ┌──────────────┐
                  │ சொல்          │
                  │ composition  │
                  └──────┬───────┘
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
       பெயர்          வினை          இடை / உரி
      entities      actions       relation/constraint
          └──────────────┬──────────────┘
                         │
                         ▼
                 வேற்றுமை / புணர்ச்சி
                 roles + composition
                         │
                         ▼
                  ┌──────────────┐
                  │ பொருள்        │
                  │ meaning      │
                  │ in context   │
                  └──────┬───────┘
                         │
                         ▼
                     execution
```

This is a research map, not a historical map of Tolkāppiyam.

---

## 7. What would count as success?

Aytham should be able to demonstrate at least three examples where this model produces a tangible benefit over a conventional language.

Candidate experiments:

1. **Role-safe API call** — two arguments share the same data type but cannot be accidentally swapped because their semantic roles differ.
2. **Effect-aware vinai** — a transformation that reads data is meaningfully distinct from one that changes the world, and composition rules enforce that distinction.
3. **Uri refinement** — values carry declarative constraints that remain visible to the compiler and error system.
4. **Punarchi composition** — incompatible transformations fail at composition time with an explanation in terms of their roles/constraints.
5. **Tamil-text correctness** — source-level eḻuttu awareness catches Unicode/Tamil-form mistakes with diagnostics a generic lexer would not provide.

If these experiments collapse into ordinary named parameters, functions, traits, or refinement types with only Tamil labels changed, the model must be revised.

---

## 8. What is deliberately undecided

Do **not** freeze these yet:

- concrete syntax;
- keyword list;
- file extension;
- static vs dynamic typing;
- ownership model;
- memory management;
- object orientation;
- functional vs imperative surface style;
- compiler implementation language;
- LLVM vs WebAssembly vs native/custom VM backend;
- package manager;
- standard library design.

The semantic research should drive those decisions, not the other way around.
