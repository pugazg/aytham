# Extensible Record Model

## Concept

Represent evolving facts as an open record.

Example:

```text
EmailFacts {
  syntax_valid,
  ownership_verified,
  marketing_consent,
  mfa_verified
}
```

## Strengths

- Open-ended facts
- Natural extension
- Good fit for data-oriented systems

## Questions

- How are fact relationships represented?
- How is evidence lineage preserved?
- How are invalidation rules expressed?
