# Tolkāppiyam 27-Iyal Survey — Source Corrections / Errata

Status: **active correction layer**  
Applies to: `docs/research/TOLKAPPIYAM_27_IYAL_SURVEY.md`

The 27-iyal survey was intentionally a first-pass map. Direct source reading after that survey corrected some of its working glosses and research temptations. This file has precedence wherever the first-pass survey conflicts with a later direct-source notebook or decision log.

---

## E-001 — எச்சவியல் is not to be treated as “compiler inference / ellipsis”

### Earlier first-pass wording

The structured corpus gloss `Ellipsis` suggested a research question about omitted computational information, implicit arguments, and role inference.

### Correction

Direct reading of the Tolkāppiyam source shows that எச்சவியல் opens with:

```text
இயற்சொல்
திரிசொல்
திசைச்சொல்
வடசொல்
```

and includes broader residual word-grammar material. The chapter cannot be reduced to a modern concept of ellipsis/inference from its editorial English gloss.

### Current status

```text
Eccaviyal → compiler inference
```

is **WITHDRAWN**.

If Aytham later studies implicit/recoverable information, that feature must be motivated independently by a programming problem and modern inference theory, not by the iyal title.

See:

- `docs/research/TOLKAPPIYAM_SOURCE_NOTEBOOK_001.md`
- `docs/DECISIONS.md` D-0017

---

## E-002 — `uri` is not currently an Aytham refinement-type term

### Earlier first-pass wording

The survey described Uriyiyal as a possible basis for a first-class qualification/refinement relation and used an `UriClaim` sketch.

### Correction

Direct source reading shows that Tolkāppiyam Uriyiyal is broader: it concerns `isai`, `kurippu`, `paṇpu`, relation to peyar/vinai, lexical-semantic multiplicity, and contextual interpretation.

Nannūl later reorganizes/characterizes `uriccol` differently, foregrounding qualities and poetic usage.

### Current status

- Experiment 002's modern **qualification claim** remains an Aytham design hypothesis.
- The Tamil term `uri` is **REOPENED** and must not be frozen as a formal refinement construct.

See D-0015 and `NANNUL_COMMENTARY_NOTEBOOK_002.md`.

---

## E-003 — `idai` is mediation/dependence research, not a control-flow category

### Earlier first-pass wording

The survey listed sequencing, branching, synchronization, composition and causality as possible `idai` research areas.

### Correction

Direct Tolkāppiyam reading and Nannūl comparison both emphasize that `idai` functions with/through surrounding peyar/vinai material and has dependent/mediating functions.

### Current status

Research:

> semantic mediation/connection whose presence contributes meaning

Do not define:

```text
idai = if | else | pipe | semicolon | await | control operator
```

See D-0016 and `NANNUL_COMMENTARY_NOTEBOOK_002.md`.

---

## E-004 — `vinai` is not simply an effectful function

### Earlier first-pass wording

The survey proposed a transformation contract with requires/establishes/preserves/invalidates/effects/capabilities/produces.

### Correction

That entire structure is **Aytham design**. Tolkāppiyam's Vinaiyiyal foregrounds temporal/tense structure, and Nannūl/commentarial evidence analyzes vinai through an action-centred relational frame.

### Current status

Keep the modern transformation contract, but describe it neutrally as `ActionFrame` in research documents until terminology is settled.

Do not imply that the historical grammatical category already contains effect systems, capability systems, contracts or graph rewriting.

---

## E-005 — புணர்ச்சி research is boundary-sensitive joining, not a pipe operator

### Correction

Nannūl and Mayilaināthar commentary strengthen the structural lesson that joining depends on both participants, the relation/context under which they meet, and whether rule-governed change occurs at the boundary.

### Current status

Aytham tests composition outcomes such as:

```text
DIRECT
TRANSFORMED
REQUIRES_ADAPTOR
AMBIGUOUS
REJECTED
```

This remains modern language design inspired by boundary-sensitive joining, not a translation of Tamil phonology into software semantics.

---

## Precedence rule

For current Aytham research, use this order:

```text
DECISIONS.md
    > dedicated source/commentary notebooks
    > this ERRATA file
    > first-pass 27-IYAL survey
```

The first-pass survey remains useful as evidence of the research path and should not be silently rewritten to erase earlier hypotheses.
