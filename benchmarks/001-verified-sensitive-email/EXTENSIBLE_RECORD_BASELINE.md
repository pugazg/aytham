# Extensible Record Baseline Model

Status: conceptual comparison

An alternative to fixed state tuples is an extensible record/open row model.

Conceptually:

```text
EmailFacts {
  syntax_valid,
  ownership_verified,
  mfa_verified,
  ...future facts
}
```

An action requests only the fields it needs.

## Advantages

- new fact kinds can be added without changing every existing type;
- functions can remain local to required information;
- avoids Boolean dimension growth.

## Remaining challenges

- field presence is not automatically semantic provenance;
- scope and freshness require additional modelling;
- transformations need preservation/invalidation rules.

The Aytham comparison must demonstrate value beyond open records by showing useful semantics around identity, lineage and explanations.
