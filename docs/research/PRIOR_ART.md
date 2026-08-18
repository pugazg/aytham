# Prior Art — Tamil Programming Languages

Status: **preliminary research record**  
Last reviewed: 2026-08-18

This document exists to prevent Aytham from claiming novelty merely because it uses Tamil syntax or terminology.

## Research rule

Aytham must distinguish:

- prior work that **uses Tamil to express conventional programming constructs**;
- prior work that **changes programming-language grammar for Tamil usability**;
- prior work that **uses Tamil grammatical theory as the computational model itself**.

The third category is the one most relevant to Aytham's research claim.

## Swaram — 2003

**Classification:** Tamil general-purpose/procedural language; conventional computational model.

The published overview describes Swaram as a general-purpose procedural language. Its design deliberately aimed for similarity to C/C++/Java, and the paper states that most syntax and semantics were derived from Java and C. Its implementation included a compiler, bytecode, and a virtual machine.

### Relevance to Aytham

Swaram is important prior art for a true Tamil programming language, but the available description does **not** show a computational model derived from Tolkāppiyam or Tamil grammatical categories.

### Source

- S. G. Ganesh, G. R. Prakash, K. K. Ravi Kumar, *An Overview of 'Swaram': A Language for Programming in Tamil*, Tamil Internet Conference 2003.
- Indexed/transcribed copy: https://studyres.com/doc/8804131/an-overview-of--swaram---a-language-for-programming-in-tamil

---

## Ezhil — 2009

**Classification:** Tamil interpreted procedural language; Tamil-friendly grammar and keywords; conventional imperative constructs.

Ezhil is explicitly designed for Tamil-speaking beginners. Its published description presents Tamil-language logical constructs corresponding to familiar conditional, branch, loop, function, and imperative programming concepts.

### Relevance to Aytham

Ezhil is stronger prior art than simple keyword substitution because it deliberately shapes grammar for Tamil speakers. However, its published model remains an interpreted procedural programming language rather than a Tolkāppiyam-derived semantic architecture.

### Sources

- Muthiah Annamalai, *Ezhil: A Tamil Programming Language*, 2009: https://arxiv.org/abs/0907.4960
- Official site: https://ezhillang.org/
- Follow-up educational paper: https://arxiv.org/abs/1308.1733

---

## த (TA) — 2012 proposal

**Classification:** proposed Tamil programming language / design exploration.

The public project notes explicitly refer to Ezhil and Swaram and discuss creating a new Tamil programming language and choosing Tamil words for its structure.

### Relevance to Aytham

It is relevant as evidence that Tamil terminology, Tamil syntax, and the desire for a new Tamil programming-language structure have been explored before. The material reviewed so far does not establish a Tolkāppiyam-derived computational model.

### Source

- https://tamilprogrammingta.wordpress.com/

---

## Uyirmei — experimental

**Classification:** experimental Tamil grammar implemented through Ruby metaprogramming.

Uyirmei describes itself as an experimental programming language in Tamil and explicitly maps constructs such as:

- `சாற்று` → let
- `வினை` → function
- `எனில்` → if
- `மீண்டும்` → loop
- `நிறுத்து` → break
- `மெய் / பொய்` → true / false

### Relevance to Aytham

Uyirmei demonstrates that creating a custom Tamil programming grammar is not itself novel. Its documented constructs, however, remain direct counterparts of established programming concepts.

### Source

- https://github.com/rcdexta/uyirmei

---

## Tamizhi — 2026

**Classification:** native compiled language with Tamil syntax; conventional lexer/parser/AST/LLVM architecture.

Tamizhi describes itself as a native compiled programming language using Tamil syntax and LLVM. Its source tree and documentation expose conventional compiler components including lexer, parser, AST, and code generation. Its keyword table maps Tamil terms to main/function/print/input/integer/string/boolean/if/else/for/while/return/call and related concepts.

### Relevance to Aytham

Tamizhi occupies the name `தமிழி / Tamizhi` and is direct prior art for a modern compiled Tamil-syntax language. The documentation reviewed so far does not show a Tolkāppiyam-derived semantic system.

### Source

- https://github.com/BackendDeveloperHub/Tamizhi

---

## Computational Tamil grammar research

There is also substantial work on using grammatical rules computationally for **analysing Tamil natural language**: morphological analysers, generators, parsers, computational grammars, and NLP resources.

This work is highly relevant, because Aytham should learn from formalisation efforts instead of inventing naive interpretations of Tamil grammar.

But the research question is different:

```text
Computational Tamil linguistics:
Tamil language → grammatical analysis → machine representation

Aytham research:
Tamil grammatical concepts → programming abstractions → general computation
```

One useful modern overview is:

- Kengatharaiyer Sarveswaran, *Morphology and Syntax of the Tamil Language* (2024): https://arxiv.org/abs/2401.08367

---

## Preliminary finding

**No verified source reviewed so far documents a Tamil programming language whose fundamental programming model is intentionally built from the Tolkāppiyam structural/categorical framework rather than primarily adapting a conventional procedural/imperative model.**

This is a **preliminary negative finding**, not proof of global novelty.

Aytham must not yet advertise itself as "the first Tolkāppiyam-based programming language."

## Research still required before an originality claim

Search at minimum:

1. INFITT / Tamil Internet Conference proceedings beyond the known Swaram and Ezhil papers;
2. Indian university theses and dissertations;
3. Tamil Virtual Academy / CICT technical publications;
4. ACM / IEEE / Springer / arXiv searches for Tamil programming-language design;
5. older personal-language projects and archived websites;
6. GitHub, GitLab, SourceForge, package registries;
7. Tamil NLP / computational grammar work that might have crossed into language design;
8. patents only where a specific technical invention becomes relevant;
9. trademark records separately for the Aytham name.

## Originality test for Aytham

A future feature should **not** be counted as conceptually original merely because:

- its keyword is Tamil;
- its compiler stage has a Tamil name;
- a familiar type is renamed in Tamil;
- a familiar AST node is given a Tolkāppiyam label.

A feature becomes interesting to Aytham when the Tamil grammatical source helps produce a **different useful rule, relationship, composition mechanism, semantic distinction, error model, or way of reasoning about programs**.
