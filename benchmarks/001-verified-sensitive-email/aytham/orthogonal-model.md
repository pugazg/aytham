# Benchmark 001 Extension — Aytham Orthogonal-Facts Model

Status: **paper semantics / not executable**  
Controlling extension: `../ORTHOGONAL_FACTS.md`

This document models exactly the same four-fact stress case used by the TypeScript and Rust baselines.

It is not Aytham syntax and does not assume a planner, theorem prover, or general knowledge graph.

---

## 1. Kernel assumption

The extension uses only:

```text
Subject / Value Identity
Claim
Action / Transformation
requires
establishes
preserves
invalidates
effect/capability requirement
```

Optional claim dimensions are used only where the benchmark needs them:

```text
scope
freshness
provenance/evidence
```

---

## 2. One subject, four independent claims

After `ParseEmail`, the subject is conceptually:

```text
email#v1
```

with:

```text
email#v1 : email_syntax_valid
```

The four extension facts are separate claims about that same semantic subject.

### Ownership

```text
Claim ownership_verified {
    subject       email#v1
    scope         account(user-42)
    established   T0
    expires       T0 + 120s
    evidence      ownership-challenge-001
}
```

### Marketing consent

```text
Claim marketing_consent {
    subject       email#v1
    scope         account(user-42)
    established   T0
    evidence      consent-001
}
```

### MFA

```text
Claim mfa_verified {
    subject       email#v1
    scope         account(user-42)
    established   T0
    expires       T0 + 30s
    evidence      mfa-challenge-001
}
```

### Jurisdiction

```text
Claim jurisdiction_allowed {
    subject       email#v1
    scope         account(user-42)
    value         IN
    established   T0
    expires       T0 + 120s
    evidence      jurisdiction-policy-001
}
```

No combined state name such as:

```text
OwnedAndConsentedAndMfaAndJurisdictionEmail
```

is introduced.

But that absence is not itself an Aytham victory: TypeScript and Rust also avoid named combinatorial states by using generic/marker dimensions.

---

## 3. Action requirements are subsets of the claim set

### SendSecurityAlert

```text
Action SendSecurityAlert {
    recipient -> email#v1
    account   -> user-42

    requires
        ownership_verified {
            subject = recipient
            scope   = account
            valid_at = now
        }

        mfa_verified {
            subject = recipient
            scope   = account
            valid_at = now
        }

        capability network_send

    effects
        network_send
}
```

The action does not inspect or require:

```text
marketing_consent
jurisdiction_allowed
```

### SendMarketingMessage

```text
Action SendMarketingMessage {
    recipient -> email#v1
    account   -> user-42

    requires
        ownership_verified {
            subject = recipient
            scope   = account
            valid_at = now
        }

        marketing_consent {
            subject = recipient
            scope   = account
        }

        capability network_send

    effects
        network_send
}
```

It does not require MFA or jurisdiction approval.

### SendRegulatedNotice

```text
Action SendRegulatedNotice {
    recipient    -> email#v1
    account      -> user-42
    jurisdiction -> IN

    requires
        ownership_verified {
            subject = recipient
            scope   = account
            valid_at = now
        }

        jurisdiction_allowed {
            subject = recipient
            scope   = account
            value   = jurisdiction
            valid_at = now
        }

        capability network_send

    effects
        network_send
}
```

It does not require marketing consent or MFA.

---

## 4. Independent invalidation

The benchmark transformation is deliberately narrow:

```text
Action RevokeMarketingConsent {
    subject -> email#v1

    requires
        marketing_consent(subject=email#v1)

    invalidates
        marketing_consent(subject=email#v1)

    preserves
        ownership_verified(subject=email#v1)
        mfa_verified(subject=email#v1)
        jurisdiction_allowed(subject=email#v1)
}
```

After execution on the success path, the semantic state is:

```text
email#v1
  + email_syntax_valid
  + ownership_verified
  - marketing_consent
  + mfa_verified
  + jurisdiction_allowed(IN)
```

Expected consequences:

```text
SendSecurityAlert    eligible, subject to freshness/scope
SendRegulatedNotice  eligible, subject to freshness/scope/value
SendMarketingMessage blocked: marketing_consent missing
```

### Important soundness question

The word `preserves` cannot be accepted as an unchecked comment.

A future Aytham implementation must define why preservation is trusted. Possible mechanisms remain open:

```text
preservation follows structurally because the subject value is unchanged
compiler-verifiable rule
trusted library contract
runtime re-check
explicit unsafe assertion
```

Benchmark 001 does not choose among them.

---

## 5. O1 — Missing exactly one fact

Available:

```text
ownership_verified
mfa_verified
```

For `SendSecurityAlert`:

```text
ALLOW if both claims are fresh and correctly scoped
```

For `SendMarketingMessage`:

```text
REJECT

missing:
    marketing_consent

established but irrelevant to this requirement:
    mfa_verified
```

The desired Aytham advantage is not merely rejection. It is that the same generic requirement matcher should be able to produce this explanation from action declarations rather than custom code in every action.

That advantage is not yet implemented.

---

## 6. O2 — Fact kinds are not interchangeable

Available:

```text
ownership_verified
jurisdiction_allowed(IN)
```

Required:

```text
marketing_consent
```

Expected:

```text
REJECT
missing claim: marketing_consent

jurisdiction_allowed does not satisfy marketing_consent
```

No claim-substitution rule exists merely because both claims share subject/scope metadata.

---

## 7. O3 — Revoke one, preserve others

Before:

```text
ownership_verified
marketing_consent
mfa_verified
jurisdiction_allowed(IN)
```

After `RevokeMarketingConsent`:

```text
ownership_verified
mfa_verified
jurisdiction_allowed(IN)
```

Expected diagnostic for marketing send:

```text
SendMarketingMessage cannot execute.

Missing:
    marketing_consent

History:
    marketing_consent was established by GrantMarketingConsent
    marketing_consent was invalidated by RevokeMarketingConsent

Unaffected claims:
    ownership_verified
    mfa_verified
    jurisdiction_allowed(IN)
```

The `History` section is a candidate Aytham benefit only if it can be generated automatically from lineage/invalidation information.

---

## 8. O4 — MFA stale independently

State:

```text
ownership_verified  valid through T0 + 120s
marketing_consent   present
mfa_verified         expires T0 + 0.5s
```

At:

```text
now = T0 + 1s
```

`SendSecurityAlert`:

```text
REJECT

mfa_verified is established but stale
expired_at: T0 + 0.5s
required_valid_at: T0 + 1s
```

`SendMarketingMessage`:

```text
ALLOW
```

because MFA is not one of its requirements.

This tests whether claim validity is evaluated only when semantically relevant.

---

## 9. O5 — Jurisdiction mismatch independently

Available:

```text
jurisdiction_allowed {
    value = IN
}
```

Action requires:

```text
jurisdiction_allowed {
    value = EU
}
```

Expected:

```text
REJECT

claim kind exists: jurisdiction_allowed
subject matches
scope matches
freshness matches
value does not satisfy requirement

available: IN
required:  EU
```

This is richer than `missing jurisdiction_allowed`; the matcher must distinguish an absent claim from an incompatible claim.

---

## 10. O6 — Wrong subject

Facts established for:

```text
emailA#v1
```

cannot satisfy requirements whose recipient is:

```text
emailB#v1
```

Expected:

```text
REJECT

ownership_verified exists for another subject
required subject: emailB#v1
claim subject:    emailA#v1
```

Subject identity is therefore part of requirement matching, not optional provenance metadata.

---

## 11. State representation candidate

The minimal conceptual state required by this extension is closer to:

```text
SemanticState {
    subject email#v1
    claims {
        email_syntax_valid
        ownership_verified(...)
        marketing_consent(...)
        mfa_verified(...)
        jurisdiction_allowed(...)
    }
    capabilities {
        network_send
    }
}
```

An action does not require the state to have a predeclared nominal name. It declares a **query/requirement over the state**.

Conceptually:

```text
State ⊨ Requirements(Action)
```

This is now the central benchmark question.

However, this is not automatically novel. It overlaps with refinement constraints, proof/evidence contexts, capability environments, rule engines and logic-style fact matching.

---

## 12. Direct comparison pressure from TypeScript/Rust

The conventional baselines show that independent presence can be encoded as:

```text
TypeScript:
EmailFacts<O, M, F, J>

Rust:
EmailFacts<O, M, F, J>
```

where each dimension records present/absent.

Therefore Aytham cannot claim:

> Independent facts require 2^N named wrapper classes in conventional languages.

That claim is falsified by the benchmark design itself.

The real comparison is:

```text
Conventional generic state dimensions
             vs
Aytham requirement matching over independent claims
```

Questions:

1. Does adding a fifth/tenth claim make generic state signatures significantly harder to maintain?
2. Does Aytham avoid touching a central generic type definition when a domain adds a new claim kind?
3. Can Aytham actions quantify only over the claims they require without threading unrelated type parameters?
4. Can Aytham generate subject/scope/freshness/history diagnostics generically?
5. Does that generic matcher remain bounded, predictable and understandable?

---

## 13. Current Aytham hypothesis after this extension

The potential advantage has narrowed again.

It is **not**:

```text
Aytham avoids combinatorial state classes.
```

Strong conventional generics avoid that too.

The surviving hypothesis is:

> Aytham may provide an **open-ended semantic fact environment** in which actions state only the relations/claims they require, transformations establish/preserve/invalidate those claims independently, and one shared checker/explanation model handles subject, scope, freshness and provenance without every new claim becoming a new generic state dimension threaded through APIs.

This is precise enough to implement and falsify.

---

## 14. Gate before implementation

Do not implement a full Aytham runtime from this paper model.

First compare the actual TypeScript/Rust artifact ceremony and execution evidence.

A minimal Aytham validator is justified only if the extension leaves a credible advantage around:

```text
open-ended fact composition
shared requirement matching
shared diagnostics
independent invalidation/history
```

If those benefits can be obtained just as cleanly with a small conventional library pattern, narrow Aytham again before implementation.