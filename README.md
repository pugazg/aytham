# ஆய்தம் · Aytham

**A research-first project exploring whether Tamil grammatical thought can inspire a genuinely new programming-language model.**

> Status: **Research / pre-specification**
>
> No claim is yet made that Aytham is the first Tolkāppiyam-inspired programming language. Novelty claims remain provisional until prior-art research is substantially complete.

## Core question

Most Tamil programming-language projects make programming accessible through Tamil syntax, Tamil keywords, or Tamil identifiers while retaining a familiar imperative/procedural computational model.

Aytham starts with a different question:

> **Can concepts from Tamil grammatical thought—especially the structural worlds of எழுத்து (eḻuttu), சொல் (sol), and பொருள் (poruḷ)—help us design useful programming abstractions that are not merely English/C/Python concepts translated into Tamil?**

The working hypothesis is not that Tolkāppiyam secretly describes computation, nor that its three major divisions map directly onto lexer → parser → semantic analyser. That would be an anachronistic and shallow reading.

Instead, Aytham treats Tamil grammatical categories as a **design research source**. Every borrowed concept must pass two tests:

1. **Historical honesty** — what does the Tamil source actually describe?
2. **Computational usefulness** — does the inspired abstraction improve programming, rather than merely rename an existing construct?

## Why ஆய்தம்?

`ஃ` is a distinctive Tamil sign with a special structural place in the writing system. The project uses **Aytham / ஆய்தம்** as its working language name and `ஃ` as a possible visual identity.

The name is currently a **working project identity**, not a declaration of trademark clearance.

## Research direction

The first design hypothesis is a layered model:

```text
source
  ↓
எழுத்து · eḻuttu
well-formed computational signs and literal forms
  ↓
சொல் · sol
composable computational forms
  ↓
பொருள் · poruḷ
resolved meaning in context: value, role, constraint, effect, relation
  ↓
இயக்கம் · execution / evaluation
```

This diagram is deliberately provisional. Aytham must not reduce Tolkāppiyam to modern compiler terminology.

A second, potentially more distinctive research direction comes from the சொல்லதிகாரம் categories:

- **பெயர் · peyar** — computational entities / denotable values
- **வினை · vinai** — transformations, actions, or effects
- **இடை · idai** — composition, connection, mediation, control
- **உரி · uri** — qualification, refinement, constraints, properties

A third direction investigates whether concepts such as **வேற்றுமை** can inspire role-aware argument relationships, and whether **புணர்ச்சி** can inspire checked composition rules. These are hypotheses to test, not predetermined features.

## What Aytham must NOT become

Aytham should not be only this:

```text
if       → எனில்
else     → இல்லையெனில்
function → வினை
print    → அச்சிடு
```

Tamil keywords may eventually exist, but keyword translation is not the research contribution.

Aytham also must not claim that modern programming concepts are directly present in an ancient grammar. Any modern abstraction derived from a historical concept must be labelled as a **design interpretation**.

## Principles

1. **Research before implementation.** Compiler code follows semantic clarity.
2. **Tamil-inspired, not Tamil-decorated.** A borrowed Tamil concept must affect language behaviour or reasoning.
3. **Source-grounded.** Historical claims require traceable sources.
4. **No forced analogies.** If a Tolkāppiyam concept does not yield a useful programming abstraction, we do not use it.
5. **Useful beyond novelty.** Aytham should eventually solve real programming problems.
6. **Unicode-native.** Tamil text must be handled correctly as Unicode, including normalization and grapheme-aware diagnostics.
7. **Tamil-first, interoperable.** A Tamil conceptual model must not isolate users from existing software ecosystems.
8. **Explicit provenance.** Historical source, scholarly interpretation, and Aytham invention are kept separate.
9. **Provisional originality claims.** Prior art must be documented before asserting novelty.
10. **Readable errors are part of the language.** Diagnostics should explain both form and meaning, not expose only compiler internals.

## Repository structure

```text
docs/
  research/
    PRIOR_ART.md
    RESEARCH_QUESTIONS.md
  design/
    DESIGN_PRINCIPLES.md
    COMPUTATIONAL_MODEL.md
  ROADMAP.md
```

The repository will later grow to include a formal specification, examples, reference interpreter/compiler, tests, tooling, and standard library only after the research foundation is stable.

## Current milestone

**Milestone 0 — Establish the research foundation**

Success means:

- prior Tamil programming-language work is documented fairly;
- Tolkāppiyam-derived concepts are separated from Aytham inventions;
- at least one genuinely useful computational abstraction is demonstrated;
- the language is not dependent on translated keywords for its identity;
- a small semantic core can be specified before syntax is frozen.

## Source discipline

The project should distinguish three evidence labels in research documents:

- **SOURCE** — directly supported by a cited Tamil text / scholarly source;
- **INTERPRETATION** — a reasoned reading or comparison;
- **AYTHAM DESIGN** — a new programming-language abstraction inspired by, but not attributed to, the historical source.

This distinction is foundational to the project.

## License

A license has not yet been selected. Do not assume source or documentation licensing until an explicit repository license is added.
