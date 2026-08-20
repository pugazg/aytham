# Aytham Canonical Semantic Graph Schema v0.1

## Status

Experimental specification. This schema is not a final runtime format.

The purpose is to define how Aytham semantic meaning is represented after surface notation resolution.

## Core principle

Surface notation is not authoritative.

The canonical semantic graph is authoritative.

```
Surface notation
        |
        v
Semantic resolver
        |
        v
Canonical semantic graph
```

## Top-level graph

```json
{
  "entities": [],
  "relations": [],
  "actions": [],
  "claims": [],
  "transformations": [],
  "evidence": [],
  "contexts": []
}
```

## Entity

Represents a meaningful object.

```json
{
  "id": "Entity#1",
  "identity": {},
  "claims": [],
  "relations": [],
  "provenance": {}
}
```

## Relation

Represents a meaningful connection between nodes.

```json
{
  "id": "Relation#1",
  "source": "Entity#1",
  "target": "Entity#2",
  "type": "owns",
  "status": "explicit"
}
```

## Action

Represents a semantic operation.

```json
{
  "id": "Action#1",
  "participants": [],
  "requirements": [],
  "effects": [],
  "establishedClaims": []
}
```

## Claim

Represents knowledge held by the system.

```json
{
  "id": "Claim#1",
  "subject": "Entity#1",
  "property": "verified",
  "value": true,
  "confidence": "established",
  "evidence": [],
  "authority": []
}
```

## Transformation

Represents a meaning-changing operation.

```json
{
  "id": "Transformation#1",
  "input": [],
  "output": [],
  "operation": "VerifyOwnership",
  "lineage": []
}
```

## Evidence

Supports claims without being identical to authority.

```json
{
  "id": "Evidence#1",
  "supports": "Claim#1",
  "source": "Document#1"
}
```

## Context

Captures conditions affecting interpretation.

```json
{
  "id": "Context#1",
  "time": "",
  "place": "",
  "domain": ""
}
```

## Epistemic status

Claims and relations may carry:

- explicit
- derived
- inferred
- uncertain
- disputed

## Required graph queries

A future Aytham system should answer:

### Why?

Why does this claim/action exist?

### How known?

What evidence and authority support it?

### What changed?

Which transformation produced this state?

### What is missing?

Which required claim or transformation prevents execution?

## Design boundary

This schema is intentionally semantic, not syntactic.

A future Tamil-native surface language should resolve into this graph rather than directly define meaning through word order.
