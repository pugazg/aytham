# Benchmark 001 — Aytham Semantic Kernel Model

Status: **paper semantic model / not executable yet**

This file models the exact requirements from `../README.md` using the reduced Aytham Semantic Kernel Candidate.

It is **not Aytham syntax**.

The notation below is a neutral research serialization used to expose which semantic information the candidate requires.

---

# 1. Kernel used

Only these core concepts are assumed:

```text
Subject / Value Identity
Relation / Role
Claim
Action / Transformation
Composition Judgment
```

Supporting information is used only because the benchmark requires it:

```text
provenance/evidence
scope
validity/freshness
capability/effect
lineage
```

No general knowledge graph, planner, proof search, or automatic path search is assumed.

---

# 2. Subject identity and lineage

A semantic fact applies to an exact value lineage, not to a mutable variable name.

Example identities:

```text
email#v0
email#v1
email#v2
```

where:

```text
email#v0
    raw input

email#v1
    output of ParseEmail

email#v2
    output of ReplaceDomain(email#v1)
```

The labels are explanatory IDs, not proposed source syntax.

---

# 3. Claim model used by this benchmark

Minimum claim:

```text
Claim {
    subject
    property
    value
    status
}
```

Benchmark-specific optional dimensions:

```text
provenance
scope
established_at
expires_at
```

Examples:

```text
Claim {
    subject: email#v1
    property: email_syntax_valid
    value: true
    status: established
    provenance: ParseEmail#17
}
```

and:

```text
Claim {
    subject: email#v1
    property: ownership_verified
    value: true
    status: established
    provenance: VerifyOwnership#23
    evidence: challenge-001
    scope: account(user-42)
    established_at: T0
    expires_at: T0 + 60s
}
```

The second claim is intentionally richer because ownership verification in the benchmark is scoped and time-sensitive.

Aytham must not require this metadata for simple static facts that do not need it.

---

# 4. ParseEmail

Neutral semantic contract:

```text
Action ParseEmail {
    participant
        input -> email#v0

    requires
        email#v0 : RawEmail

    establishes on success
        email#v1 : email_syntax_valid

    lineage
        email#v1 derived_from email#v0

    effects
        none

    failure
        EmailSyntaxError
}
```

Important:

`email_syntax_valid` does **not** establish ownership.

---

# 5. VerifyOwnership

```text
Action VerifyOwnership {
    participant
        subject -> email#v1
        account -> user-42

    requires
        email#v1 : email_syntax_valid

    establishes on success
        claim: ownership_verified
        subject: email#v1
        scope: account(user-42)
        established_at: T0
        expires_at: T0 + 60s
        evidence: challenge-001
        provenance: VerifyOwnership#23

    effects
        verification_store_read
        clock_read
}
```

For this benchmark, the resulting ownership claim must remain tied to `email#v1`.

It cannot be detached and applied to a different email subject through ordinary semantic composition.

---

# 6. ReplaceDomain

This action deliberately changes identity-relevant email content.

```text
Action ReplaceDomain {
    participant
        input -> email#v1

    requires
        email#v1 : email_syntax_valid

    produces
        email#v2

    lineage
        email#v2 derived_from email#v1

    preserves
        email_syntax_valid

    invalidates
        ownership_verified
}
```

The preservation of syntax validity is still contingent on the replacement producing a syntactically valid address.

The key benchmark point is:

```text
ownership_verified(email#v1)
```

does not become:

```text
ownership_verified(email#v2)
```

merely because both values share a logical variable/history.

---

# 7. Network-send capability

The sending context contains an explicit capability:

```text
Capability {
    holder: SendContext#1
    permits: network_send
}
```

This is not a claim that Aytham has invented capability systems.

The benchmark tests whether capability requirements can participate naturally in the same action explanation as semantic claims.

---

# 8. SendSensitiveMessage

```text
Action SendSensitiveMessage {
    participant relations
        recipient -> email#v1
        account   -> user-42

    requires
        email#v1 : email_syntax_valid

        ownership_verified {
            subject: email#v1
            scope: account(user-42)
            valid_at: now
        }

        capability: network_send

    effects
        network_send

    establishes on success
        DeliveryReceipt#1 : delivered
}
```

The intended validation is semantic rather than nominal-only:

```text
recipient subject matches verification subject
AND
verification scope matches required account
AND
verification is fresh at now
AND
network_send capability is available
```

---

# 9. Composition judgment

A direct boundary from `ParseEmail` to `SendSensitiveMessage` fails.

Produced after parse:

```text
email#v1 : email_syntax_valid
```

Required by send:

```text
email#v1 : email_syntax_valid
email#v1 : ownership_verified for account(user-42), fresh at now
capability: network_send
```

The composition judgment should therefore report a semantic gap.

Target explanation:

```text
SendSensitiveMessage cannot execute.

Recipient:
    email#v1

Established:
    email_syntax_valid

Missing:
    ownership_verified
      scope required: account(user-42)
      freshness required: valid at now

The syntax-valid claim is insufficient to establish ownership.
```

A one-step registered action such as `VerifyOwnership` may be displayed as a possible next action only if its own prerequisites are already satisfied.

No multi-step planner is required by this benchmark.

---

# 10. Mandatory invalid cases

## I1 — Raw text sent directly

Available:

```text
email#v0 : RawEmail
```

Send requires:

```text
email_syntax_valid
ownership_verified
```

Expected:

```text
REJECT
missing: email_syntax_valid
missing: ownership_verified
```

Whether a future Aytham compiler rejects this statically depends on how claims produced only by runtime paths are represented. The benchmark must not assume compile-time knowledge that does not exist.

---

## I2 — Syntax-valid but ownership-unverified

Available:

```text
email#v1 : email_syntax_valid
```

Expected:

```text
REJECT
missing: ownership_verified
```

---

## I3 — Verification belongs to another value

Available:

```text
A : ownership_verified(scope=user-42)
B : email_syntax_valid
```

Attempt:

```text
recipient -> B
use ownership claim whose subject -> A
```

Expected:

```text
REJECT
ownership claim subject mismatch
required subject: B
provided subject: A
```

---

## I4 — Verified value mutated afterward

Before transformation:

```text
email#v1 : ownership_verified
```

After `ReplaceDomain`:

```text
email#v2 : email_syntax_valid
```

with:

```text
ownership_verified INVALIDATED
```

Expected send result:

```text
REJECT
ownership_verified was established for email#v1
and is not preserved for email#v2
```

This is one of the most important Aytham diagnostic hypotheses in the benchmark.

---

## I5 — Stale verification

Available:

```text
ownership_verified {
    subject: email#v1
    expires_at: T1
}

now: T2
T2 > T1
```

Expected:

```text
REJECT
ownership verification exists but is stale
expired_at: T1
required_valid_at: T2
```

This is naturally runtime-dependent unless `now` is statically known.

---

## I6 — Wrong account/scope

Available:

```text
ownership_verified {
    subject: email#v1
    scope: account(user-42)
}
```

Required:

```text
scope: account(user-77)
```

Expected:

```text
REJECT
claim is established but its scope does not satisfy this action
verified scope: user-42
required scope: user-77
```

---

## I7 — Missing network-send capability

All email claims are satisfied but:

```text
network_send capability: absent
```

Expected:

```text
REJECT
semantic requirements satisfied
missing effect capability: network_send
```

This test is useful because the explanation combines claim validity and effect authority without confusing the two.

---

# 11. Valid case

```text
email#v0
  ↓ ParseEmail
email#v1
  + email_syntax_valid
  ↓ VerifyOwnership(account=user-42)
email#v1
  + ownership_verified
    scope=user-42
    valid through T1
    evidence=challenge-001

SendContext#1
  + capability network_send

now <= T1
  ↓ SendSensitiveMessage
DeliveryReceipt#1
```

Expected:

```text
ALLOW

Satisfied:
    email_syntax_valid
    ownership_verified
        subject match
        scope match
        freshness match
    network_send capability
```

---

# 12. Where Aytham pays complexity

Aytham must pay for all of the following in the benchmark comparison:

- subject IDs / lineage;
- explicit claims;
- claim status;
- transformation preservation/invalidation rules;
- scope/freshness metadata;
- effect capability information;
- semantic resolution machinery.

These are not free benefits simply because the diagnostic is attractive.

The benchmark passes only if the integration materially improves safety, explanation, or scalability of independent facts enough to justify this machinery.

---

# 13. Progressive disclosure requirement

A simple syntax parse should conceptually be expressible with no ownership/provenance ceremony:

```text
ParseEmail(raw)
  -> establishes email_syntax_valid
```

Only `VerifyOwnership` introduces:

```text
scope
freshness
evidence
```

because that claim actually depends on them.

Only `SendSensitiveMessage` introduces:

```text
network_send capability
```

because that action performs the effect.

If a future surface language forces these dimensions onto unrelated code, the kernel fails the progressive-disclosure test.

---

# 14. What this model deliberately does not do

It does not:

- invent a path from syntax-valid to ownership-verified;
- perform multi-step planning;
- infer verification authority automatically;
- claim `requires/establishes` are novel;
- claim semantic roles are novel;
- claim provenance or capabilities are novel;
- freeze Tamil keywords;
- define the full Aytham programming language.

---

# 15. Comparison questions now enabled

Against TypeScript and Rust, ask:

1. Does Aytham avoid wrapper/type-state proliferation when independent facts multiply?
2. Is its subject-lineage invalidation clearer than ordinary value/newtype transitions?
3. Does provenance/scope/freshness remain readable rather than becoming metadata bureaucracy?
4. Are its failure explanations materially better than deliberate runtime diagnostics in the baselines?
5. Can effects and semantic claims be explained together without conflating them?
6. What guarantees are genuinely static, and which remain runtime checks?
7. Is the semantic graph necessary to expose these benefits, or can it remain only an internal IR?

The answers belong in `../COMPARISON.md` after the baseline artifacts are reviewed and, where possible, executed.
