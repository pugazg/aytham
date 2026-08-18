# Experiment 002 — Comparative Analysis

Status: **research comparison; no novelty claim**  
Scope: compare the Validated Data Flow experiment against established programming-language and provenance techniques before admitting any idea into Aytham's specification.

## 1. Question

Experiment 002 proposes that Aytham may represent progressively established facts about a value using a semantic graph containing:

- `peyar` / entity-value identity;
- `vinai` / transformation-action;
- `uri` / qualification claims;
- evidence and provenance for claims;
- semantic lineage/versioning;
- effects/capabilities;
- checked composition (`punarcci`-inspired);
- a resolved contextual meaning graph (`poruḷ`, provisional Aytham terminology).

None of these ingredients should be assumed new merely because Aytham combines them or gives them Tamil-derived names.

---

## 2. Comparison summary

| Established technique | What it already provides | Direct overlap with Experiment 002 | Consequence for Aytham |
|---|---|---|---|
| Refinement types | Types refined by logical predicates; static verification of value properties | `uri` as `positive`, `non_empty`, `email_syntax_valid`, etc. | Predicate-only `uri` is not distinctive. |
| Typestate | Permitted operations depend on the current state of an object/resource | Parsed → verified → sendable stages; state-sensitive legality | Aytham cannot claim novelty from validation-state transitions. |
| Dependent types | Types can depend on values; proofs/properties can be reflected in types | Very rich value-dependent claims and evidence | Aytham should not compete on raw expressive power. |
| Proof-carrying approaches | Machine-checkable evidence accompanies code/data claims | Evidence-bearing `uri` | Evidence attachment alone is not distinctive. |
| Effect systems | Effects are represented/inferred as part of function/expression types | `vinai` carries network, state, clock, database effects | Effect-labelled `vinai` alone is not distinctive. |
| SSA / immutable lineage techniques | New semantic versions arise as values are transformed | `candidate@v0 → @v1 → @v2`; facts do not follow a reused variable name | Versioning alone is not distinctive. |
| Provenance models (e.g. W3C PROV) | Entities, activities, derivations, agents, provenance relations | Who/what established a qualification; derivation lineage | Provenance metadata alone is not distinctive. |
| Language-integrated provenance | Provenance is propagated and protected as a language/type-system concern | Safe propagation and non-misattribution of evidence | Aytham must compare against this directly. |
| Session/protocol types | Legal communication/action sequences are checked statically | State/protocol transition aspects of composition | Later protocol experiments must acknowledge this prior art. |

---

## 3. Refinement types

Refinement types enrich ordinary types with logical predicates. Liquid Haskell, for example, demonstrates SMT-backed checking of properties beyond ordinary Haskell types.

Aytham overlap:

```text
Text { email_syntax_valid }
Money { value > 0 }
Token { not_expired }
```

If `uri` means only a predicate restricting a value, it is simply a refinement system with different vocabulary.

### What remains worth testing

Aytham's `uri` hypothesis is broader and more operational:

```text
claim
subject
established_by
evidence
authority
scope
freshness
preservation/invalidation
```

This should not be called a new kind of refinement type yet. The research question is whether programmers benefit from treating an established qualification as a first-class **semantic claim relation** in the same graph as roles, transformations, effects, and provenance.

---

## 4. Typestate

Strom and Yemini's typestate concept already makes operation legality depend on contextual state rather than nominal type alone.

This closely overlaps:

```text
raw → parsed → verified → sendable
```

Therefore Aytham must not say that it invented state-sensitive operation legality.

### Possible distinction to test

A single object can have many independent, scoped facts simultaneously:

```text
email_syntax_valid
ownership_verified_for(user_42)
verified_at(T)
marketing_consent_absent
password_reset_authorized_until(T2)
```

Aytham's research question is whether a **fact graph** with independent claims, provenance, scope, and preservation rules is more natural than forcing the entity into one monolithic typestate.

This is a usability/model distinction, not yet a theoretical novelty claim.

---

## 5. Dependent types and explicit proofs

Dependently typed languages such as Idris allow types to depend on values. Dependent pairs can carry a value together with evidence whose type depends on that value.

This is strictly more expressive than many simple Aytham examples.

Aytham should therefore avoid the goal:

> Make properties expressible that dependent types cannot express.

That is not the useful battle.

Better goal:

> Make a useful class of role/fact/effect/provenance relationships ordinary, inspectable, inferable, and explainable without requiring everyday application programmers to construct dependent proofs manually.

If Aytham eventually needs full theorem proving for routine code, the model has probably become too heavy.

---

## 6. Proof-carrying approaches

Proof-Carrying Code established the broader pattern that a producer can supply machine-checkable evidence of adherence to a policy.

Aytham's evidence-bearing `uri` therefore has clear conceptual relatives.

Aytham should distinguish several evidence strengths rather than collapse them into `verified`:

```text
statically derived
runtime checked
externally attested
cryptographically attested
trusted assertion
unsafe assertion
unknown/unproven
```

The value may lie in how evidence participates in ordinary composition and diagnostics, not in the existence of proof objects.

---

## 7. Effects

Koka and other effect systems make computational effects visible in and inferable through program types.

Therefore:

```text
VerifyOwnership
  effects: clock_read, store_read

Send
  effect: network_send
```

is not by itself a distinctive Aytham contribution.

The research question is whether `vinai` can unify:

```text
required relations
established facts
invalidated facts
preserved facts
effects
capabilities
produced relations
```

as one transformation contract that tooling can visualize and use for composition planning.

---

## 8. SSA and semantic lineage

SSA-based representations give each assigned value a single definition and naturally separate successive values produced by transformations.

Aytham's explanatory notation:

```text
candidate@v0
candidate@v1
candidate@v2
```

must therefore not be presented as an invention.

### Aytham-specific research question

Can the semantic front end automatically associate claim validity with value lineage and require each `vinai` to state or infer:

```text
establishes
preserves
invalidates
```

without exposing SSA/compiler machinery to the programmer?

Example:

```text
NormalizeDisplay
  preserves: ownership_verified

ReplaceDomain
  invalidates: ownership_verified
```

The benefit would be domain-level reasoning and diagnostics rather than a novel versioning mechanism.

---

## 9. Provenance

W3C PROV already models provenance using entities, activities, agents, derivation, generation, and related relations. Language-integrated provenance research also demonstrates that provenance metadata can be propagated safely inside a programming language and protected from accidental misattribution.

This is particularly close to Experiment 002.

Aytham therefore needs to answer:

1. Is claim provenance part of ordinary semantic checking rather than optional audit metadata?
2. Can a consumer require not only `P(value)` but `P(value)` established by an acceptable authority/process?
3. Can provenance participate in composition and capability checks?
4. Can the compiler explain missing evidence in domain terms?
5. Can provenance be erased when it is irrelevant, avoiding runtime metadata cost?

If not, Aytham should reuse established provenance concepts rather than invent parallel terminology.

---

## 10. Closest interpretation of `uri` after comparison

Do **not** define:

```text
uri = refinement type
```

or:

```text
uri = proof
```

The current hypothesis is narrower and relational:

> An Aytham `uri` is a qualification claim attached to a semantic subject/relation/action, whose validity may have a derivation, authority, scope, lifetime, and preservation law.

This definition remains experimental.

A simple static property should remain simple:

```text
positive
```

Aytham must not force every property to carry heavyweight provenance records.

---

## 11. Strongest surviving Aytham hypothesis

After comparison, **ingredient novelty is low**.

The stronger research hypothesis is the **unified programmer-facing semantic graph**:

```text
                semantic subject / peyar
                       │
         ┌─────────────┼─────────────┐
         │             │             │
       role           uri        provenance
         │             │             │
         └─────────────┼─────────────┘
                       │
                     vinai
         ┌─────────────┼─────────────┐
         │             │             │
      effects       lineage       produced facts
         │             │             │
         └─────────────┼─────────────┘
                       │
             checked composition
```

The language may be valuable if this unified graph lets ordinary programmers reason about concerns that currently require several distinct techniques:

- named/role parameters;
- refinements;
- typestate;
- provenance;
- effects;
- capability checks;
- protocol constraints;
- dataflow composition.

Combination alone is not automatically novel. It must prove coherence and usability.

---

## 12. New pass criteria for Experiment 002

Experiment 002 should remain alive only if a prototype can show most of the following:

1. A value can accumulate multiple independent facts without wrapper-type explosion.
2. Facts are tied soundly to semantic value lineage.
3. Provenance/scope/freshness can be required only where relevant.
4. Transformation contracts make preservation/invalidation understandable.
5. Effects and semantic facts compose in one inspectable model.
6. A missing pipeline step can be diagnosed or suggested from semantic requirements.
7. Diagnostics are clearer than equivalent Rust/TypeScript/Idris-style encodings for the target audience.
8. Common code remains lightweight; advanced proofs are opt-in rather than mandatory.

### Fail/revise if

- `uri` becomes an awkward duplicate of refinement types;
- the graph is hidden compiler metadata programmers cannot use;
- provenance makes ordinary code verbose;
- every state change requires theorem-prover-level annotations;
- composition search becomes ambiguous or unpredictable;
- the same benefits are clearer in a conventional ADT + typestate + effect design.

---

## 13. Candidate distinctive feature: semantic gap explanation

A promising developer-facing capability is **gap explanation / missing-link discovery**.

Given:

```text
ParseEmail
  establishes: email_syntax_valid

SendSensitiveMessage
  requires:
    email_syntax_valid
    ownership_verified
```

Aytham tooling can derive the semantic gap:

```text
missing: ownership_verified
```

If the environment contains:

```text
VerifyOwnership
  requires: email_syntax_valid
  establishes: ownership_verified
```

it may suggest that transformation.

This resembles type-directed synthesis, proof search, planning, and workflow tooling, so it also requires prior-art comparison. It is retained as a **prototype target**, not a novelty claim.

---

## 14. References / prior art to keep in the research set

- R. E. Strom & S. Yemini, *Typestate: A Programming Language Concept for Enhancing Software Reliability*, IEEE TSE, 1986.
- G. C. Necula, *Proof-Carrying Code*, POPL, 1997.
- N. Vazou et al., *Refinement Types for Haskell*, ICFP, 2014.
- R. Jhala & N. Vazou, *Refinement Types: A Tutorial*, 2020.
- D. Leijen, *Koka: Programming with Row-Polymorphic Effect Types*, 2014.
- E. Brady / Idris documentation and literature on dependent types.
- W3C, *PROV-DM: The PROV Data Model*, 2013.
- S. Fehrenbach & J. Cheney, *Language-integrated provenance*, PPDP 2016 / SCP 2018.
- LLVM Language Reference, SSA/PHI representation (implementation comparison only; SSA is not an Aytham surface model).

---

## 15. Current verdict

**Experiment 002: CONTINUE, NARROWED.**

Do not pursue “validated types” as the central invention.

Pursue and test instead:

> **A unified semantic relation graph in which roles, evidence-backed qualifications, transformations, lineage, effects, and composition are ordinary parts of resolved program meaning and therefore available to checking, diagnostics, visualization, and safe composition.**

This is the current strongest formulation, still subject to falsification.