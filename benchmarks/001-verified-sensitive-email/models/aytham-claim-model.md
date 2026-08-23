# Aytham Claim Environment Model

## Concept

Represent semantic state as claims attached to subjects.

Example:

```text
Subject: Email

Claims:
  ownership_verified
  marketing_consent
  mfa_verified

Each claim may include:
  scope
  provenance
  validity
  history
```

## Potential Advantage

A claim is not only a boolean field. It can carry semantic context.

## Validation Questions

- Does this reduce application complexity?
- Is annotation burden acceptable?
- Are diagnostics better than existing approaches?
