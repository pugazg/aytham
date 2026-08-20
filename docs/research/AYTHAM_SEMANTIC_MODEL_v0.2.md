# Aytham Semantic Model v0.2

## Status

Research model only. This document does not define syntax, compiler behaviour, or a programming language specification.

## Purpose

Aytham explores whether meaning-oriented computation can be designed using insights from Tamil grammatical and literary traditions combined with modern programming-language research.

The goal is not to translate existing programming concepts into Tamil words. The goal is to investigate whether relationships, evidence, context, and transformations can become first-class computational concepts.

## Core Semantic Model

An Aytham semantic object consists of:

- Identity
- Representations
- Observations
- Relations
- Claims
- Evidence
- Transformations
- Provenance
- Context

## Observation

Observation is separated from interpretation.

Example:

Observation:
- mountain imagery appears

Possible interpretations:
- literal mountain reference
- symbolic usage

The system should preserve the difference.

## Claims

A claim represents an asserted meaning about an object.

A claim may contain:

- value
- authority
- evidence
- confidence
- validity
- alternatives

Possible states:

- established
- derived
- inferred
- unknown
- disputed

## Evidence and Provenance

Meaning should preserve its origin.

A claim should answer:

- Where did this come from?
- What transformation produced it?
- What evidence supports it?
- Is the evidence still valid?

## ActionFrame

Actions are modelled beyond function calls.

An ActionFrame contains:

- action identity
- participants
- roles
- context
- time
- requirements
- established claims
- effects
- result relations

Example:

Transfer:

Participants:
- source account
- destination account
- amount

Requirements:
- debit authorization
- valid amount

Established:
- transfer completed

Effects:
- balance updated
- ledger updated

## Transformation

A transformation changes what is known about an object.

Example:

Raw Email
  -> ParseEmail
Email Address + syntax_valid claim
  -> VerifyOwnership
Trusted Email + ownership_verified claim

The transformation carries meaning, not only data movement.

## Semantic Composition

Composition is not only structural compatibility.

Research question:

Can produced meaning satisfy required meaning?

Example:

Available:
- email_format_valid

Required:
- ownership_verified

Missing bridge:
- VerifyOwnership

## Lessons from Validation Experiments

### Banking

Tested:
- action
- validation
- effects

### Sangam interpretation

Tested:
- context
- inference
- uncertainty

### AI/Data pipelines

Tested:
- transformation
- lineage
- provenance

The same semantic concepts appear useful across different domains.

## Research Boundaries

Aytham does not claim:

- Tolkappiyam invented modern programming concepts
- Tamil grammar maps directly to compiler structures
- existing programming-language research is replaced

## Next Research Questions

1. Can this model provide better developer explanations?
2. Can meaning-aware composition outperform simple type mismatch messages?
3. What is the minimum language construct required to express this model?
4. Which concepts are genuinely useful and which are only analogies?
