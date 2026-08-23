# Benchmark 001 Extension — Orthogonal Facts Comparative Review

Status: **PROVISIONAL — TypeScript executed; Rust execution pending; Aytham not implemented**  
Date: 2026-08-23

This review is intentionally stricter than the earlier paper experiments. It compares what the conventional baselines actually need to encode four independent facts and asks what, if anything, remains distinctive enough to justify an Aytham mechanism.

---

## 1. Central result

The extension falsifies one version of the Aytham argument.

It is **not true** that conventional typestate must create a separate named state type for every combination of independent facts.

The TypeScript baseline successfully uses:

```text
EmailFacts<O, M, F, J>
```

and the Rust baseline implements the analogous:

```text
EmailFacts<O, M, F, J>
```

with marker types.

Therefore this feared family is unnecessary:

```text
OwnedEmail
OwnedAndMfaEmail
OwnedAndMarketingEmail
OwnedAndJurisdictionEmail
OwnedAndMfaAndMarketingEmail
...
```

Strong conventional generic typing already avoids that naïve explosion.

---

## 2. What TypeScript demonstrated

The executed TypeScript extension shows that one value can carry four independent static presence dimensions.

The following transition preserves unrelated state dimensions:

```text
EmailFacts<true, true, true, true>
        |
        | revokeMarketingConsent
        v
EmailFacts<true, false, true, true>
```

After this transition:

```text
SendSecurityAlert     still type-checks
SendRegulatedNotice   still type-checks
SendMarketingMessage  does not type-check
```

That is exactly the independence property Aytham wanted to test.

The conventional baseline therefore passes the basic independent-invalidation challenge.

---

## 3. Where generic typestate pays complexity

Avoiding named-state explosion does not make the conventional representation free.

For four facts, the central state type is:

```text
EmailFacts<O, M, F, J>
```

Adding a fifth independent fact would require changing that definition to something like:

```text
EmailFacts<O, M, F, J, X>
```

and propagating the new dimension through transition signatures that preserve it.

For example, a transition unrelated to the new fact must still carry it generically:

```text
revokeMarketing:
    EmailFacts<O, true, F, J, X>
      ->
    EmailFacts<O, false, F, J, X>
```

Likewise, an action that cares only about ownership and MFA still needs type variables for unrelated dimensions so it remains polymorphic over them:

```text
SendSecurityAlert<M, J>(
    EmailFacts<true, M, true, J>
)
```

This is a real **closed-dimension pressure**.

The generic state definition knows the full universe of tracked dimensions.

---

## 4. The surviving Aytham contrast

Aytham's paper model does not require a single closed tuple of all possible fact kinds.

Conceptually a subject has an open claim environment:

```text
email#v1 {
    email_syntax_valid
    ownership_verified(...)
    marketing_consent(...)
    mfa_verified(...)
    jurisdiction_allowed(...)
}
```

An action asks only for the claims it needs:

```text
SendSecurityAlert
    requires ownership_verified
    requires mfa_verified
```

A fifth unrelated claim should not require changing that action declaration at all.

This suggests a narrower possible advantage:

> **open-ended claim composition rather than closed generic state dimensions.**

But this is not yet a unique programming-language idea.

It overlaps directly with established techniques such as:

- extensible records / row polymorphism;
- effect rows;
- capability/evidence environments;
- logic/rule contexts;
- heterogeneous type-level sets;
- proof-token parameterisation.

Therefore this benchmark does not establish novelty. It identifies the next comparison obligation.

---

## 5. Requirement matching

### TypeScript/Rust baseline

Fact presence is partly encoded in the host type system.

Fact validity is then checked by ordinary code.

For example, the TypeScript security action effectively performs:

```text
static:
    ownership dimension == true
    MFA dimension == true

runtime:
    ownership subject matches
    ownership account scope matches
    ownership is fresh
    MFA subject matches
    MFA account scope matches
    MFA is fresh
```

The baseline shares helper logic for subject/scope checks, but the action still contains explicit code selecting and validating its relevant facts.

### Aytham hypothesis

The Aytham version wants the action to declare:

```text
requires
    ownership_verified(subject=recipient, scope=account, valid_at=now)
    mfa_verified(subject=recipient, scope=account, valid_at=now)
```

and have one shared checker perform the matching.

If implemented successfully, this would move repeated validity logic from each action body into reusable language/runtime semantics.

That is a stronger research proposition than simply storing claims.

---

## 6. Diagnostics

The TypeScript baseline already provides precise runtime variants:

```text
mfa_stale
jurisdiction_mismatch
scope_mismatch
subject_mismatch
```

so Aytham cannot win by comparing itself to intentionally generic host-language errors.

The candidate Aytham advantage is **declarative diagnostic generation**.

For example, after marketing revocation it could potentially derive:

```text
SendMarketingMessage cannot execute.

Missing:
    marketing_consent

History:
    established by GrantMarketingConsent
    invalidated by RevokeMarketingConsent
```

The conventional baseline could also produce this message, but it would need explicit history storage and diagnostic code.

Aytham earns value only if the history/explanation follows automatically from semantics the program already needed for correctness.

If programmers must manually author equivalent metadata and diagnostic branches, the advantage disappears.

---

## 7. Independent freshness is not difficult for conventional code

The extension deliberately makes MFA expire before ownership.

Observed TypeScript behaviour:

```text
SendSecurityAlert
    -> REJECT mfa_stale

SendMarketingMessage
    -> PASS
```

This correctly demonstrates that an irrelevant stale fact need not contaminate the whole state.

Aytham therefore cannot claim that independent runtime fact validity is unavailable in conventional languages.

Its possible value remains centralising the matching/explanation policy.

---

## 8. Independent invalidation is also achievable conventionally

`revokeMarketingConsent()` demonstrates:

```text
O=true
M=true
F=true
J=true
```

becoming:

```text
O=true
M=false
F=true
J=true
```

at the type level in TypeScript, with the analogous marker transition designed in Rust.

Therefore Aytham's:

```text
invalidates marketing_consent
preserves ownership_verified
preserves mfa_verified
preserves jurisdiction_allowed
```

is potentially clearer as a semantic declaration, but it is not by itself a capability conventional languages cannot model.

The comparison is now about **declaration locality, scalability and explanation**, not expressibility.

---

## 9. New challenge to Aytham: open rows / proof tokens

The current TypeScript baseline deliberately uses a four-dimensional generic state because that is a strong and common typestate technique.

However, it is not the strongest possible conventional counter-model.

TypeScript or Rust can instead represent independent evidence as separate proof/capability values:

```text
email
ownershipProof(email)
marketingConsentProof(email)
mfaProof(email)
jurisdictionProof(email)
```

and pass only the proofs an action requires.

That approach avoids threading unrelated generic dimensions through action signatures.

Likewise, languages with row-polymorphic/extensible effect or record systems can model open sets more directly than this benchmark's Boolean tuple.

This means Aytham should **not implement a semantic validator yet solely because it looks more open-ended than `EmailFacts<O,M,F,J>`**.

Before implementation, compare the kernel against:

1. independent proof-token APIs;
2. row-polymorphic/extensible-record models;
3. effect-row style open sets where relevant.

Otherwise the benchmark risks stopping one comparison too early.

---

## 10. Current scorecard

| Property | TypeScript generic state | Rust marker state | Aytham candidate |
|---|---|---|---|
| Avoid named combinatorial states | **demonstrated** | designed | conceptual |
| Different actions require different subsets | **demonstrated** | designed | conceptual |
| Remove one fact, preserve others | **demonstrated** | designed | conceptual |
| Independent freshness | **demonstrated** | designed | conceptual |
| Subject/scope/value checks | **demonstrated runtime** | designed runtime | conceptual shared matcher |
| Open-ended new fact kinds without central state-type change | no | no in current marker design | conceptual yes |
| Generic domain explanation from declarations | no; handwritten helper/action logic | no; handwritten logic | conceptual yes |
| Provenance/history explanation | not automatic | not automatic | conceptual yes |
| Executed evidence | **yes** | pending | no |

The rows marked `conceptual` are not Aytham wins. They are hypotheses awaiting implementation or stronger comparison.

---

## 11. Consequence for the Semantic Kernel Candidate

The benchmark supports retaining these concepts for research:

```text
Claim
requires
establishes
invalidates
preserves
subject identity
scope/freshness
capability requirements
```

But it rejects a broad justification based on typestate explosion.

The kernel should now be evaluated around two narrower properties:

### A. Open claim environment

Can new claim kinds be introduced without editing a central state tuple and without forcing unrelated actions to thread new type variables?

### B. Shared semantic matcher + explanation

Can one bounded checker derive:

```text
missing claim
wrong subject
wrong scope
stale claim
wrong claim value
invalidated claim history
missing capability
```

from action/claim declarations without application authors rewriting those checks for every action?

If these two properties do not survive comparison against proof-token and row-polymorphic approaches, Aytham should narrow again.

---

## 12. Decision from this extension

**Do not freeze the Semantic Kernel.**

**Do not resume path planning.**

**Do not yet build the full validator.**

The orthogonal-facts extension successfully improved the research question, but the TypeScript baseline is stronger than the earlier hypothesis anticipated.

The next evidence gate should compare:

```text
Aytham open claim environment
        vs
independent conventional proof tokens
        vs
row-polymorphic/extensible state techniques
```

with special attention to subject identity, scope/freshness and generated explanations.

Only after that comparison should we decide whether a minimal executable Aytham requirement matcher is justified.