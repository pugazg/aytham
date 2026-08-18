# Aytham Language Design Principles

Status: **binding principles for research phase**

These principles should be reviewed before any syntax, parser, interpreter, compiler, or standard-library decision.

## 1. Meaning before syntax

Do not begin by choosing Tamil words for `if`, `class`, `for`, or `function`.

Define what Aytham believes a computation **is**, how forms acquire meaning, how roles and constraints interact, and how actions compose. Syntax follows.

## 2. Tamil-inspired must mean behaviourally different

A Tamil grammatical concept earns a place in Aytham only when it affects at least one of:

- static validity;
- runtime behaviour;
- composition;
- type/refinement reasoning;
- effect reasoning;
- API structure;
- diagnostics;
- tooling.

Renaming an existing compiler node is insufficient.

## 3. Historical source and modern invention stay separate

Every major design document should distinguish:

- **SOURCE** — source-supported Tamil grammatical description;
- **INTERPRETATION** — modern explanatory reading;
- **AYTHAM DESIGN** — programming-language invention.

Never write as though Tolkāppiyam contained modern concepts such as compiler IRs, type systems, effect systems, or virtual machines.

## 4. No forced completeness

Aytham does not need to map every Tolkāppiyam category into programming.

A concept with no useful computational role should remain part of the research background rather than become a feature.

## 5. Solve real software problems

Every proposed core feature should eventually be tested against realistic domains:

- financial transactions;
- text processing;
- HTTP/API clients;
- data transformation;
- concurrency;
- state machines;
- file/database access;
- library composition.

Aytham should not exist only as a demonstration language.

## 6. Prefer invalid-state prevention over clever syntax

Role, refinement, effect, and composition systems are valuable only if they prevent mistakes or make intent clearer.

The language should prioritize semantic guarantees over surface novelty.

## 7. Diagnostics are semantic output

Errors are part of the language design.

Diagnostics should explain:

- what form was read;
- what meaning was inferred;
- what role was expected;
- which qualification failed;
- which composition rule was violated;
- how the programmer can repair it.

Tamil diagnostics should be possible without requiring knowledge of English compiler terminology.

## 8. Unicode is part of correctness

Tamil source must be treated as structured Unicode text, not merely bytes.

The specification must eventually define:

- normalization;
- script mixing;
- grapheme-aware spans;
- identifier equivalence;
- confusable characters;
- source encoding;
- diagnostics for malformed/unexpected sequences.

## 9. Tamil-first does not mean ecosystem-isolated

Aytham should eventually interoperate with existing software.

Potential interoperability targets may include C ABI, WebAssembly, JSON/HTTP, and foreign-function interfaces, but no backend is selected yet.

Tamil conceptual independence and practical interoperability are compatible goals.

## 10. Surface bilingualism remains open

The semantic core should not depend on English names.

However, the project should research whether English aliases or an alternate surface syntax improve collaboration without weakening Tamil-first design.

Do not decide this through ideology alone; prototype and test usability.

## 11. Prefer orthogonal concepts

Avoid overlapping features that solve the same problem differently.

For example, if `uri` handles refinements and constraints, do not separately add ad-hoc range types, contract syntax, and validator annotations unless they have distinct semantics.

## 12. Composition should be explainable

If two Aytham forms compose, the programmer should be able to ask **why** they compose.

If they fail to compose, the compiler should identify the violated relationship.

This principle is central to the புணர்ச்சி-inspired research direction.

## 13. Role is not type

Aytham should preserve the research distinction between:

```text
what a value is
```

and

```text
what role that value plays here
```

This is the foundation of the வேற்றுமை-inspired experiment.

Roles may eventually interact with types, but should not be collapsed into nominal wrapper types by default.

## 14. Qualification is not identity

A value's `uri`/qualification should be capable of expressing properties that can be acquired, proven, weakened, combined, or lost without necessarily changing the value's underlying identity.

This gives Aytham a path toward refinement/contract semantics.

## 15. Action must expose consequence

If `vinai` becomes a core category, an action should make its relevant consequence visible:

- pure transformation;
- state mutation;
- I/O;
- resource consumption;
- capability use;
- protocol transition.

Hidden effects weaken compositional reasoning.

## 16. Established PL theory is a comparison set, not an enemy

Aytham must study existing ideas such as:

- algebraic data types;
- refinement/dependent types;
- effect systems;
- session/protocol types;
- capability security;
- named arguments;
- traits/type classes;
- pattern matching;
- dataflow;
- functional composition;
- logic/rule programming.

If an established model already captures an Aytham idea cleanly, acknowledge it and decide whether Tamil grammatical framing adds real value.

## 17. Keep the core small

The first semantic core should be minimal enough to reason about formally and implement more than once.

A large keyword list is not progress.

## 18. Prototype semantics before compiler optimization

The first executable implementation, when appropriate, should be a reference interpreter or semantic prototype optimized for clarity and tests—not speed.

LLVM/native compilation is a later decision.

## 19. Every feature needs a rejection criterion

For each proposed feature record:

- problem solved;
- source inspiration;
- Aytham invention;
- closest existing PL concept;
- expected benefit;
- prototype;
- measurable result;
- condition under which the idea should be abandoned.

This protects the project from becoming attached to attractive terminology.

## 20. Novelty claims must remain narrower than evidence

Until research is complete, preferred wording is:

> "Aytham explores a programming-language model inspired by Tamil grammatical thought."

Avoid:

> "Aytham is the first programming language based on Tolkāppiyam."

The latter requires much stronger evidence.
