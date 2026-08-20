# Aytham Graph JSON Schema v0.1

## Purpose

This document defines the first machine-readable representation for the Aytham semantic graph.

It is not a programming language syntax. It is an intermediate semantic representation between surface notation and runtime processing.

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
  "id": "entity-001",
  "type": "Account",
  "identity": {},
  "claims": [],
  "relations": []
}
```

## Relation

Represents a meaningful connection.

```json
{
  "id": "relation-001",
  "source": "entity-001",
  "target": "entity-002",
  "type": "owns",
  "status": "explicit"
}
```

## Action

Represents a semantic change.

```json
{
  "id": "action-001",
  "type": "Transfer",
  "participants": [],
  "requires": [],
  "establishes": [],
  "effects": []
}
```

## Claim

Represents knowledge about an entity or action.

```json
{
  "id": "claim-001",
  "subject": "entity-001",
  "property": "verified",
  "value": true,
  "confidence": "established",
  "evidence": []
}
```

## Transformation

Represents a meaning-changing operation.

```json
{
  "id": "transform-001",
  "input": "entity-001",
  "output": "entity-002",
  "operation": "VerifyIdentity",
  "lineage": []
}
```

## Evidence and Authority

Evidence explains support for claims.
Authority explains trust level of evidence.

## Resolution rules

- Every semantic object must have a stable identity.
- Explicit and inferred relations must remain distinguishable.
- Claims must preserve confidence and provenance.
- Transformations must preserve lineage.
- Ambiguous resolutions must not silently become facts.

## Future implementation targets

- JSON Schema validation
- semantic graph loader
- requirement resolver
- explanation engine
