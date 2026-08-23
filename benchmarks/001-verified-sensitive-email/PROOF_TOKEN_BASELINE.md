# Proof Token Baseline Model

Status: conceptual comparison

A strong conventional alternative is to represent independent facts as explicit proof objects.

Example:

```text
Email
OwnershipProof(email, account, expiry)
MfaProof(email, expiry)
NetworkSendCapability
```

Actions receive only required proofs:

```text
SendSecurityAlert(
  email,
  OwnershipProof,
  MfaProof,
  NetworkSendCapability
)
```

## Advantages

- no global fact registry;
- no unrelated metadata in simple functions;
- explicit authority transfer;
- compatible with existing languages.

## Remaining challenges

- consistent diagnostics across APIs;
- proof invalidation after transformations;
- discovering which proof is missing;
- maintaining subject identity.

The Aytham comparison must beat this model, not a weaker state-wrapper design.
