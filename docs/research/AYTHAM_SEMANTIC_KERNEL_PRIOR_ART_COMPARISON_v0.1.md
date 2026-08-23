# Aytham Semantic Kernel — Focused Prior-Art Comparison v0.1

Status: **research comparison / evidence gate**  
Date: 2026-08-23  
Reset dependency: `docs/AYTHAM_DIRECTION_RESET_2026-08-23.md`

## Purpose

This document performs the focused comparison required by the 2026-08-23 direction reset.

It asks whether the current Semantic Kernel Candidate contains a programming-language contribution that survives comparison with the closest established ideas.

Current kernel candidate:

```text
Subject / Value Identity
Relation / Role
Claim
Action / Transformation
Composition Judgment
```

Supporting structures when needed:

```text
Evidence
Context
Authority
Confidence / epistemic status
Provenance / lineage
```

Action/Transformation contract under test:

```text
participants
requires
establishes
preserves
invalidates
effects
capabilities
```

This document does **not** attempt to prove novelty. Its job is to identify overlap, narrow the Aytham hypothesis, and determine what Benchmark 001 must actually test.

---

# 1. Semantic roles and case frames

## Established prior art

Charles Fillmore's case-grammar work treated predicate arguments through abstract semantic roles such as Agent, Patient, and Instrument rather than only through surface syntactic position. Case frames associated predicates with obligatory or optional semantic participants.

Later frame semantics broadened this into representations of structured situations/scenarios with associated roles, expectations, and perspectives. FrameNet operationalized this tradition using frames and frame elements tied to lexical units and corpus annotations.

Representative sources:

- Charles J. Fillmore, **The Case for Case** (1968), DOI record / open copy: https://doi.org/10.5281/zenodo.5809982
- Charles J. Fillmore, **Frame Semantics and the Nature of Language** (1976): https://doi.org/10.1111/j.1749-6632.1976.tb25467.x
- Frame-semantic parsing overview, *Computational Linguistics* 40(1): https://direct.mit.edu/coli/article/40/1/9/1461/Frame-Semantic-Parsing

## Direct overlap with Aytham

Aytham's ActionFrame idea:

```text
Transfer
  source      AccountA
  destination AccountB
  amount      Money100
```

is **not conceptually novel merely because roles are attached to an action rather than encoded positionally**.

The broader idea that a predicate/action is understood through a structured participant frame is longstanding linguistic prior art.

The Nannūl action-frame research therefore strengthens Aytham's Tamil design inspiration, but it does not establish a new computational mechanism.

## What Aytham may still test

The narrower programming-language question is:

> Can semantic roles become statically checked, programmer-facing relations that participate in ordinary API contracts, composition, inference, diagnostics, refactoring, and tooling without requiring permanent wrapper types?

This differs from natural-language semantic-role analysis in **application**, not necessarily in abstract representation.

## Falsification condition

Reject or narrow Aytham's role mechanism if a conventional combination of:

- named parameters;
- records/row types;
- branded/newtypes;
- phantom types;
- ordinary API schema tooling

provides the same safety and diagnostics with less conceptual machinery.

## Benchmark obligation

Benchmark 001 must include at least one case where two values share the same nominal type but have distinct contextual roles, and compare Aytham with normal typed encodings.

---

# 2. Frame semantics versus Aytham ActionFrame

## Established prior art

Frame semantics associates meaning with structured scenarios whose participants occupy frame elements. A frame can include more than argument labels: related expectations, relations, presuppositions, and perspectives may participate in interpretation.

This matters because Aytham has been moving toward:

```text
ActionFrame {
    participant_relations
    circumstantial_relations
    temporal_context
    requirements
    effects
    established_claims
}
```

## Direct overlap

The claim:

> an action has participants, circumstances, and contextual meaning

is not novel.

Aytham must not present `ActionFrame` as a new abstract model solely because it packages those dimensions together.

## Potential Aytham distinction

Aytham's stronger programming question is whether such a frame can be an **executable/static contract boundary** rather than an analysis structure.

A programming ActionFrame would need machine-checkable rules for:

```text
role compatibility
required claims
established claims
preservation/invalidation
effects/capabilities
composition
```

and should generate programmer-facing explanations.

## Falsification condition

If ActionFrame becomes only a verbose record containing parameters plus preconditions/postconditions, it should not be treated as a distinctive core abstraction.

---

# 3. Hoare logic and pre/postcondition contracts

## Established prior art

Hoare logic provides a formal basis for specifying and verifying imperative programs using preconditions and postconditions. A judgment such as:

```text
{P} C {Q}
```

relates a command `C` to required and resulting state assertions.

Modern verification and contract systems build extensively on this family of ideas.

Reference overview:

- University of Cambridge, Hoare Logic course: https://www.cl.cam.ac.uk/teaching/1213/HoareLogic/

## Direct overlap with Aytham

Aytham's:

```text
requires
establishes
```

has an obvious relationship to:

```text
precondition
postcondition
```

Likewise:

```text
preserves
invalidates
```

relates to frame conditions, state transition reasoning, separation logic, and explicit mutation specifications.

Therefore Aytham cannot claim novelty from a transformation contract of the form:

```text
requires P
establishes Q
```

## Potential Aytham distinction

The narrower hypothesis is that pre/post-like reasoning becomes integrated with:

- contextual semantic roles;
- independently accumulated claims;
- evidence/provenance;
- validity/freshness;
- effects/capabilities;
- domain-level diagnostics.

The value, if any, is an **everyday programmer-facing semantic contract model**, not the invention of pre/postconditions.

## Falsification condition

If conventional contracts, refinement types, and effect systems express the benchmark more clearly, Aytham's unified contract should be simplified or abandoned.

---

# 4. Graph transformation / graph rewriting

## Established prior art

Graph transformation is a mature formal discipline in which graph structures are modified by rules. It has been applied to software models, distributed/concurrent systems, visual languages, and model transformation.

Representative sources:

- Reiko Heckel, **Graph Transformation in a Nutshell** (2006): https://doi.org/10.1016/j.entcs.2005.12.018
- Barbara König, Dennis Nolte, Julia Padberg, Arend Rensink, **A tutorial on graph transformation**: https://research.utwente.nl/en/publications/a-tutorial-on-graph-transformation/

## Direct overlap with Aytham

The general model:

```text
semantic graph
      ↓ rule / transformation
new semantic graph
```

is established prior art.

Likewise, application conditions, typed graphs, constraints, and rule-based evolution are not new because Aytham gives their nodes Tamil-inspired semantic interpretations.

## Consequence for Aytham

The canonical semantic graph should currently be treated primarily as:

- an IR / semantic representation;
- a validation/explanation substrate;
- a possible formalization tool.

It must **not** become the project's claimed innovation by itself.

## Potential Aytham distinction

Aytham may still test whether ordinary source-language constructs resolve into a graph whose role/claim/evidence/effect edges provide unusually useful diagnostics and composition checks.

That is an ergonomics/language-integration hypothesis.

## Falsification condition

If the graph is merely hidden compiler metadata or a graph database model that ordinary programmers do not benefit from, remove it from the language-facing story and keep it only as an implementation IR.

---

# 5. STRIPS and automated planning — critical result for Experiments 029–034

## Established prior art

Fikes and Nilsson's STRIPS framework searches for a sequence of operators that transforms an initial world model into one satisfying a goal condition.

In the classical formulation, operators/actions are described through applicability conditions and effects. Later presentations commonly express STRIPS actions through:

```text
preconditions
add effects
delete effects
```

Primary source:

- Richard E. Fikes and Nils J. Nilsson, **STRIPS: A New Approach to the Application of Theorem Proving to Problem Solving**, *Artificial Intelligence* 2 (1971), 189–208: https://ai.stanford.edu/~nilsson/OnlinePubs-Nils/PublishedPapers/strips.pdf

## Direct overlap with Aytham

Experiments 029–034 proposed:

```text
current claims
     ↓ registered transformation
new claims
     ↓ registered transformation
goal claim
```

with transformation requirements, produced claims, path search, and a distinction between planned and executed actions.

This is **very close in problem shape to classical planning**.

Aytham's proposed:

```text
requires
establishes
invalidates
```

also maps naturally onto:

```text
preconditions
add list
delete list
```

when used for path planning.

## Decision consequence

The 2026-08-23 pause on semantic path search is strongly validated.

Aytham must not continue path planning as though it were an organic novel consequence of the Tamil-inspired model.

## Potential later Aytham distinction

If reopened, the relevant question would be much narrower:

> Can a programming language use its semantic contracts to provide local, explainable repair suggestions or composition guidance without becoming a general planner?

That may be useful, but it is a tooling feature and must be compared against planning/synthesis systems.

## Falsification condition

If the desired behaviour is simply planning over preconditions/effects, use or adapt established planning methods rather than reinventing them under Aytham terminology.

---

# 6. Program synthesis / type-directed search

## Established prior art

Program synthesis automatically searches for a program satisfying a user specification. Modern approaches include enumerative search, constraint solving, stochastic search, deduction, and type-directed/component-based synthesis.

Representative sources:

- Sumit Gulwani, Oleksandr Polozov, Rishabh Singh, **Program Synthesis** (2017): https://www.microsoft.com/en-us/research/publication/program-synthesis/
- Zheng Guo et al., **Program Synthesis by Type-Guided Abstraction Refinement** (2019): https://arxiv.org/abs/1911.04091

Type-directed component synthesis can search a collection of available components for a term satisfying a target type/specification. Some methods explicitly use graph reachability as part of this search.

## Direct overlap with Aytham

Aytham diagnostics such as:

```text
Missing:
    ownership_verified

Candidate bridge:
    VerifyOwnership
```

can range from a simple indexed lookup to a synthesis/planning problem depending on how many transformations, intermediate requirements, polymorphic conditions, and alternatives are involved.

The more Aytham attempts multi-step automatic bridge construction, the more directly it enters established synthesis/search territory.

## Safe current boundary

For the near term, Aytham should restrict itself to:

```text
explain the missing requirement
```

and possibly:

```text
show directly registered one-step transformations
whose prerequisites are already satisfied
```

Multi-step search remains paused.

## Falsification condition

If bridge discovery requires general component search or proof search, treat it explicitly as synthesis/planning and reuse established algorithms rather than presenting it as a new semantic primitive.

---

# 7. Provenance models

## Established prior art

The W3C PROV family models provenance around entities, activities, agents, derivation, generation, and related relations. Its semantics explicitly supports reasoning about where information came from and how it was produced.

Source:

- W3C, **PROV-DM / Semantics of the PROV Data Model**: https://www.w3.org/TR/prov-sem/

## Direct overlap with Aytham

Aytham concepts such as:

```text
claim established_by transformation
derived_from prior value
evidence source
authority
lineage
```

overlap strongly with established provenance modelling.

## Potential Aytham distinction

The meaningful programming-language question is:

> Can provenance participate directly in ordinary static/runtime eligibility checks and diagnostics instead of being optional audit metadata?

Example:

```text
requires ownership_verified
established_by acceptable_verifier
within 30 days
for account user-42
```

This is a useful research direction, but provenance representation itself is not new.

## Falsification condition

If provenance is used only for tracing after execution and does not participate in language semantics, Aytham should reuse a standard provenance representation rather than creating a parallel conceptual system.

---

# 8. Effect and capability systems

## Established prior art

Effect systems make side effects part of program typing/reasoning, while capability systems make authority/resources explicit.

Aytham's concepts such as:

```text
network_send
clock_read
database_write
```

are therefore not independently distinctive.

Experiment 002 already acknowledged this overlap.

## Potential Aytham distinction

The remaining question is whether effects/capabilities can participate naturally in the same action contract and diagnostic model as roles and claims.

Example:

```text
SendSensitiveMessage
  recipient email

  requires
      email : ownership_verified

  requires capability
      network_send
```

Aytham must show that this integration is clearer than simply combining a refinement/typestate layer with an established effect system.

---

# 9. Key comparison matrix

| Aytham mechanism | Closest prior art | Novel by itself? | Current status |
|---|---|---:|---|
| Contextual role edges | semantic roles, case frames, named/typed arguments | No | TEST ergonomics + static use |
| ActionFrame participants | frame semantics, event models | No | TEST as executable contract |
| `requires` / `establishes` | Hoare logic, contracts, refinements | No | RETAIN only if integration pays |
| `preserves` / `invalidates` | frame conditions, typestate, transition systems | No | TEST lineage diagnostics |
| Semantic graph | knowledge graphs, graph rewriting, compiler IRs | No | IR/representation candidate |
| Provenance / evidence | W3C PROV, language-integrated provenance | No | TEST semantic eligibility use |
| Effects/capabilities | effect/capability systems | No | TEST integrated diagnostics |
| Transformation path search | STRIPS/planning | No | PAUSED |
| Candidate bridge search | planning / program synthesis / proof search | No | one-step only for now |
| Meaning-oriented diagnostics | compilers, proof assistants, contract systems | Not inherently | HIGH-VALUE UX HYPOTHESIS |
| Unified programmer-facing model | combinations exist in multiple systems | Not proven | CENTRAL RESEARCH QUESTION |

---

# 10. What survives this comparison

The comparison removes several possible novelty stories.

Aytham should **not** claim novelty from:

```text
semantic roles
frame-based action modelling
pre/postconditions
graph transformations
provenance
precondition/effect planning
program synthesis
explicit effects
```

The narrower surviving hypothesis is:

> **Can a general-purpose programming language make contextual roles, independently established claims, value-lineage validity, effects/capabilities, and composition requirements one coherent programmer-facing contract system whose explanations are substantially clearer than conventional combinations of types, wrappers, contracts, and effect mechanisms?**

This is now the central hypothesis to test.

---

# 11. Architectural consequence

Do not treat the entire semantic graph as the user-visible programming model.

The stronger architecture to test is:

```text
             ordinary program
                  |
        +---------+---------+
        |                   |
 values / expressions   semantic contract layer
 control / algorithms   roles / claims
 state / functions      pre/post meaning
                        effects/capabilities
                        provenance when needed
        |                   |
        +---------+---------+
                  |
          resolved semantics
                  |
             validator
                  |
             execution
```

The graph may remain an internal resolved representation used for diagnostics and tooling.

This keeps Aytham on a programming-language path rather than turning every computation into a workflow/knowledge graph.

---

# 12. Benchmark 001 implications

The focused prior-art comparison sharpens the benchmark.

The verified-email benchmark must test **integration cost and explanation quality**, not merely whether Aytham can represent the properties.

Rust and TypeScript can already model much of the safety through wrappers, typestate-like APIs, discriminated unions, branded types, capabilities, and disciplined functions.

Aytham therefore needs to demonstrate one or more of the following:

1. fewer wrapper/type combinations for independent orthogonal claims;
2. clearer invalidation when a value changes;
3. provenance/scope/freshness requirements without severe ceremony;
4. role information reusable across action boundaries;
5. effects and semantic claims explained together;
6. substantially better domain-level diagnostics;
7. progressive disclosure that keeps ordinary code compact.

If it cannot, the semantic kernel should be reduced rather than expanded.

---

# 13. Research decision from this pass

## Retain for Benchmark 001

- contextual role relations;
- claim status attached to exact value lineage;
- `requires`;
- `establishes`;
- `preserves`;
- `invalidates`;
- optional evidence/provenance/scope/freshness;
- explicit effects/capabilities;
- meaning-oriented diagnostics.

## Treat primarily as internal representation

- canonical semantic graph.

## Keep provisional

- Action versus Transformation as separate top-level concepts;
- Context as a first-class object;
- Authority as a first-class object;
- Tamil formal terminology for qualification/action/meaning.

## Continue to pause

- multi-step transformation discovery;
- shortest semantic path search;
- general planning;
- general proof/program synthesis.

---

# 14. Next activity

Proceed to **Benchmark 001 — Verified Sensitive Email Workflow**.

The benchmark should first define exact behavioural requirements and invalid cases, then implement the conventional TypeScript and Rust baselines before judging the Aytham representation.

Do not design more Aytham architecture until those baseline implementations make the comparison concrete.
