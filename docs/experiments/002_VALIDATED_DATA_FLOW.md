# Experiment 002 — Validated Data Flow

Status: **paper experiment**  
Purpose: test whether Aytham's `peyar + vinai + uri + punarchi + poruḷ` model generalizes beyond role-heavy APIs and can represent evidence-bearing validation flows more clearly than conventional branded/refinement/typestate techniques.

> This document is **AYTHAM DESIGN**. Tamil grammatical concepts inspire the experiment; no claim is made that Tolkāppiyam specifies validation pipelines, proof objects, dataflow graphs, or programming-language semantics.

---

## 1. Research claim under test

Experiment 001 tested semantic roles around a money transfer. Experiment 002 deliberately removes the `source/destination/amount` advantage and asks a different question:

> Can Aytham represent **facts established progressively about the same value**, preserve where those facts came from, and allow later computations only when the required facts are valid?

Test flow:

```text
raw text
   ↓ ParseEmail
email-shaped value
   ↓ VerifyEmail
verified email
   ↓ Send
message delivery
```

The key hypothesis is that validation should not necessarily create an unrelated nominal type at every stage.

Instead, Aytham may model:

```text
peyar
  + uri established by vinai
  + evidence/provenance
  + composition rules
```

so that semantic state is explicit and inspectable.

---

## 2. Conventional solutions already solve much of this

Aytham must not pretend this problem is unsolved.

A typed language can use branded/newtypes:

```text
RawText
EmailAddress
VerifiedEmailAddress
```

Conceptually:

```text
parseEmail : RawText -> Result<EmailAddress>
verifyEmail : EmailAddress -> Result<VerifiedEmailAddress>
send : VerifiedEmailAddress -> Message -> Result<Delivery>
```

A TypeScript-style design could use brands:

```text
type EmailAddress = string & { readonly __email: unique symbol }
type VerifiedEmail = EmailAddress & { readonly __verified: unique symbol }
```

Rust can use distinct wrapper types or typestate-like structures.

Refinement/dependent/proof-oriented systems can express still stronger guarantees.

Therefore Experiment 002 passes only if Aytham offers a coherent advantage beyond renaming these techniques in Tamil.

---

## 3. First Aytham model

### 3.1 Peyar — the value/entity

Start with a value:

```text
"person@example.org"
```

Its initial semantic record might be:

```yaml
peyar: input_17
value_domain: Text
uri:
  - unicode_well_formed
```

At this stage Aytham must **not** claim that the text is an email address.

### 3.2 Vinai — a transformation that may establish facts

`ParseEmail` examines the text.

On success, it establishes a qualification such as:

```text
email_syntax_valid
```

The important research question is whether the value should become a completely different nominal identity, or whether Aytham should preserve a semantic lineage:

```text
input_17@v0
   │ ParseEmail
   ▼
input_17@v1
  uri += email_syntax_valid
```

`@v0` / `@v1` are explanatory notation only, not proposed syntax.

### 3.3 Uri — not merely a label

For this experiment, `uri` is stronger than a Boolean tag.

A qualification record may need:

```text
claim       — what is asserted
subject     — the exact semantic value the claim applies to
established_by — which vinai established it
evidence    — optional proof/runtime evidence
scope       — where/for what purpose the claim is valid
freshness   — whether time/context can invalidate it
strength    — static, runtime-checked, externally attested, etc.
```

Example:

```yaml
uri:
  claim: email_syntax_valid
  subject: input_17@v1
  established_by: ParseEmail#84
  evidence: parser-rule-set-v1
  scope: email-address-syntax
  freshness: stable-while-value-unchanged
```

This is an Aytham hypothesis, not accepted language semantics.

---

## 4. Validation is a change in known meaning, not necessarily representation

A conventional pipeline often changes types:

```text
String
  ↓
EmailAddress
  ↓
VerifiedEmail
```

Aytham should test a different view:

```text
same logical value / traced lineage

Text
  uri: none

      ↓ ParseEmail

Text-like value
  uri: email_syntax_valid

      ↓ VerifyEmail

same logical address lineage
  uri:
    email_syntax_valid
    ownership_verified
```

The runtime representation may remain the same UTF-8 string throughout.

The **resolved meaning** changes because evidence has accumulated.

This separation may be useful when one value can carry many independent facts without requiring a combinatorial family of wrapper types.

---

## 5. Verification must mean something precise

The word `verified` is dangerously vague.

Experiment 002 therefore does **not** define one universal `verified` property.

Possible claims include:

```text
email_syntax_valid
mail_domain_resolves
mailbox_challenge_completed
account_ownership_verified
verified_for_password_reset
verified_at_time(T)
```

These facts are not interchangeable.

For example:

```text
mail_domain_resolves
```

does not imply:

```text
account_ownership_verified
```

Aytham should require qualifications to be semantically specific enough that accidental substitution is impossible.

---

## 6. Proposed flow

### Stage A — Raw input

```yaml
peyar: candidate
value_domain: Text
value: "person@example.org"
```

### Stage B — ParseEmail vinai

Requires:

```text
candidate -> Text
```

May produce success:

```text
candidate@v1
  uri:
    email_syntax_valid
```

or failure:

```text
EmailSyntaxError
```

### Stage C — VerifyOwnership vinai

Requires:

```text
candidate
  uri:
    email_syntax_valid
```

and may require effects/capabilities such as:

```text
verification_store_read
challenge_validation
clock_read
```

On success it establishes:

```text
candidate@v2
  uri:
    email_syntax_valid
    account_ownership_verified
```

with evidence tied to the exact value/version.

### Stage D — SendSensitiveMessage vinai

Requires a recipient relation whose subject carries:

```text
email_syntax_valid
account_ownership_verified
```

It may also require:

```text
network_send capability
```

and produce:

```text
DeliveryReceipt
```

---

## 7. Meaning graph

One possible resolved graph:

```text
                           ┌─────────────────┐
                           │  candidate@v0   │
                           │      Text       │
                           └────────┬────────┘
                                    │ input
                                    ▼
                           ┌─────────────────┐
                           │   ParseEmail    │
                           │     vinai       │
                           └────────┬────────┘
                                    │ establishes
                                    ▼
                           ┌─────────────────┐
                           │  candidate@v1   │
                           │ email syntax ✓  │
                           └────────┬────────┘
                                    │ subject
                                    ▼
                         ┌─────────────────────┐
                         │  VerifyOwnership    │
                         │       vinai         │
                         └─────────┬───────────┘
                                   │ establishes
                                   ▼
                           ┌─────────────────┐
                           │  candidate@v2   │
                           │ syntax ✓        │
                           │ ownership ✓     │
                           └────────┬────────┘
                                    │ recipient
                                    ▼
                         ┌─────────────────────┐
                         │ SendSensitiveMessage│
                         │        vinai        │
                         └─────────┬───────────┘
                                   ▼
                           ┌─────────────────┐
                           │ DeliveryReceipt │
                           └─────────────────┘
```

The graph records not only the final properties but **how each property became justified**.

---

## 8. Punarcci-inspired composition rule

Experiment 002 tests whether checked composition can operate on established meaning rather than just output/input nominal types.

Conceptually:

```text
producer poruḷ
   ↓
composition boundary
   ↓
consumer requirements
```

For direct composition:

```text
ParseEmail
   ↓
SendSensitiveMessage
```

reject because ParseEmail establishes only:

```text
email_syntax_valid
```

while SendSensitiveMessage requires:

```text
email_syntax_valid
account_ownership_verified
```

Expected diagnostic concept:

```text
SendSensitiveMessage cannot compose after ParseEmail.

Recipient requirement not established:
  account_ownership_verified

Available evidence:
  email_syntax_valid
    established by ParseEmail

A transformation that establishes ownership verification is required.
```

This is more useful than a diagnostic whose only explanation is:

```text
Expected VerifiedEmail, found EmailAddress.
```

Whether that improvement is large enough to justify the model remains under test.

---

## 9. Evidence-bearing uri

This is the strongest new hypothesis in Experiment 002.

A qualification may be treated as a **claim with provenance** rather than a free-floating property.

Possible semantic shape:

```text
UriClaim {
    subject
    predicate
    provenance
    evidence
    scope
    validity
}
```

Example:

```text
subject:
  candidate@v2

predicate:
  account_ownership_verified

provenance:
  VerifyOwnership#212

evidence:
  challenge-id: CH-8842

scope:
  account: user-42

validity:
  established-at: T
  expires-at: T + 30 days
```

Not every uri must contain runtime evidence. Some properties can be statically established.

Aytham should distinguish at least conceptually between:

```text
static fact
runtime-checked fact
externally attested fact
contextual/temporary fact
```

The exact categories remain open.

---

## 10. Why version/lineage matters

A dangerous implementation would attach `verified` to a variable name and then allow its value to change:

```text
email = "person@example.org"   // verified
email = "attacker@example.org" // changed
send(email)                     // must NOT inherit verification
```

Aytham's qualification must apply to the **semantic subject/value**, not to an incidental mutable identifier.

Therefore this experiment introduces a provisional lineage rule:

> A vinai that changes meaning-relevant content produces a new semantic version. Evidence applies only to the version/value for which it was established unless an explicit preservation rule proves otherwise.

Conceptually:

```text
candidate@v2
  ownership_verified

      ↓ ReplaceDomain

candidate@v3
  ownership_verified ?  NO
```

The compiler/runtime should either invalidate the qualification or require a declared preservation theorem/rule.

This resembles ideas from SSA, typestate, refinement systems, proof-carrying data, and functional immutability. Aytham must compare itself honestly with them.

---

## 11. Preservation rules

Not every transformation should destroy every fact.

Example:

```text
NormalizeDisplayCase
```

might preserve an ownership fact if it provably does not change the canonical address identity.

A vinai could therefore declare, conceptually:

```text
preserves:
  account_ownership_verified

establishes:
  display_normalized
```

But a stronger transformation:

```text
ReplaceDomain
```

would not preserve ownership verification.

Research question:

> Can Aytham make preservation/invalidation rules explicit and tractable without becoming a theorem prover for ordinary programs?

---

## 12. Branching and failure

Validation operations can fail.

Aytham therefore cannot model the flow as an unconditional chain.

Conceptual graph:

```text
              ParseEmail
             /          \
       success          failure
          │                │
          ▼                ▼
 email_syntax_valid   EmailSyntaxError
          │
          ▼
   VerifyOwnership
      /        \
 success       failure
    │             │
    ▼             ▼
verified      VerificationError
```

The eventual semantic core needs an explicit model for alternatives/results.

This may become an `idai` research case: a connector that relates successful and failed continuations. No decision is made yet that `idai` should mean branching.

---

## 13. Error experiments

### E1 — Raw text sent directly

```text
Text -> SendSensitiveMessage
```

Expected:

```text
Recipient value is Text.
Required qualification not established:
  email_syntax_valid
  account_ownership_verified
```

### E2 — Parsed but not verified

```text
ParseEmail -> SendSensitiveMessage
```

Expected:

```text
Recipient has email_syntax_valid,
but account_ownership_verified has not been established.
```

### E3 — Verification belongs to another value

```text
A = person@example.org  // verified
B = other@example.org   // unverified
send(B)
```

Expected:

```text
Ownership evidence is attached to A's semantic value/lineage,
not to B.
```

### E4 — Verified value mutated

```text
A@v2: ownership_verified
A@v3: domain replaced
```

Expected:

```text
The transformation changed identity-relevant content.
`account_ownership_verified` was established for A@v2
and is not preserved for A@v3.
```

### E5 — Stale verification

If a consumer requires freshness:

```text
verified_within(30 days)
```

but evidence is older:

```text
Ownership was verified, but the evidence does not satisfy
the consumer's freshness requirement.
```

### E6 — Wrong verification scope

Evidence proves ownership for account `user-42`, while the action requires authorization for `user-77`.

Expected:

```text
The verification claim is valid, but its scope does not match
this operation's required account context.
```

### E7 — Forged qualification

Source code must not be allowed simply to assert:

```text
uri: account_ownership_verified
```

unless the module possesses an explicitly trusted authority/capability to establish that claim.

This raises a future distinction between:

- declaring a requirement;
- proving/establishing a fact;
- trusting an external attestation;
- unsafely asserting a fact.

---

## 14. Static vs runtime knowledge

Some uri facts can be known at compile time:

```text
literal "person@example.org"
```

might permit compile-time syntax parsing.

Other facts fundamentally require runtime evidence:

```text
account_ownership_verified
```

Aytham's semantic graph therefore needs to distinguish:

```text
known statically
will be checked at runtime
established only on a successful runtime path
externally attested
unknown/unproven
```

A qualification is not equivalent to a compile-time Boolean.

---

## 15. Path sensitivity

After:

```text
result = VerifyOwnership(candidate)
```

only the success path may use the `ownership_verified` fact.

Conceptually:

```text
if verification succeeded:
    candidate@v2 has ownership_verified
else:
    candidate retains no such established fact
```

Any executable Aytham semantics must prevent evidence from leaking from the success branch into failure or unrelated paths.

This is a major soundness requirement.

---

## 16. Effects belong to the vinai

`ParseEmail` can be pure.

`VerifyOwnership` may require external effects:

```text
clock_read
challenge_store_read
cryptographic_check
```

`SendSensitiveMessage` may require:

```text
network_send
```

Possible graph:

```text
ParseEmail
  effect: none

VerifyOwnership
  effects:
    challenge_store_read
    clock_read

SendSensitiveMessage
  effect:
    network_send
```

The composition checker could eventually reason about both **facts** and **effects**.

For example, a pure context must not silently invoke `network_send` merely because the semantic qualifications match.

---

## 17. A provisional poruḷ record

For an expression/value at one point in the flow:

```yaml
identity: candidate@v2
category: peyar
value_domain: Text
canonical_domain: EmailAddress
relations:
  - role: recipient_candidate
uri:
  - claim: email_syntax_valid
    established_by: ParseEmail#84
    validity: stable_while_identity_unchanged
  - claim: account_ownership_verified
    established_by: VerifyOwnership#212
    scope: user-42
    established_at: T
    expires_at: T+30d
capabilities: []
lineage:
  previous: candidate@v1
```

This is explanatory data, not final Aytham syntax or representation.

---

## 18. Candidate formal judgment

Experiment 001 proposed:

```text
Γ ⊢ p : T ▷ R ▷ U ▷ E
```

Experiment 002 suggests `U` may need evidence/provenance:

```text
Γ ⊢ p : T ▷ R ▷ U[P] ▷ E
```

where:

- `T` = value domain/type;
- `R` = role relations;
- `U` = qualifications;
- `P` = provenance/evidence for established qualifications;
- `E` = effects/capabilities.

A transformation may then be modeled as:

```text
required meaning graph
        ↓ vinai
produced meaning graph
+ newly established claims
+ invalidated/preserved claims
+ effects
```

This notation is deliberately provisional.

---

## 19. What would make this better than `VerifiedEmail`

Aytham does **not** pass merely because this works:

```text
EmailAddress -> VerifiedEmailAddress
```

It must show value in one or more of these harder cases:

1. **Multiple orthogonal facts** without exponential wrapper-type combinations.
2. **Provenance** — tooling can explain which computation established a fact.
3. **Scope** — the same predicate can be valid only for a particular context.
4. **Freshness** — time-sensitive qualifications can expire.
5. **Lineage** — transformations preserve or invalidate facts explicitly.
6. **Path sensitivity** — runtime-established facts exist only on successful paths.
7. **Composition discovery** — tooling can suggest what vinai is missing between producer and consumer.
8. **Effects + facts in one composition model** rather than disconnected systems.
9. **Diagnostics** that describe missing meaning/evidence rather than nominal-type mismatch alone.

If none of these advantages survive implementation complexity, the experiment should fail.

---

## 20. Comparison obligations

Before accepting evidence-bearing `uri`, compare against:

- refinement types;
- dependent types;
- branded/newtypes;
- typestate;
- phantom types;
- proof-carrying code/data;
- liquid types;
- contracts;
- effect systems;
- capability systems;
- SSA/value versioning;
- dataflow languages;
- logic programming;
- provenance-aware databases;
- taint analysis;
- authorization logics;
- session/protocol types.

The likely originality, if any, will come from the **unified user-facing semantic model**, not from inventing each ingredient independently.

---

## 21. Pass/fail criteria

### PASS if

- one logical value can accumulate several independent qualifications without wrapper-type explosion;
- each established fact is tied soundly to the correct semantic value/version;
- mutations/transformations invalidate or preserve facts predictably;
- runtime-established facts are path-sensitive;
- provenance/scope/freshness materially improve safety or diagnostics;
- composition can explain exactly why a consumer cannot follow a producer;
- the model remains understandable for ordinary programmers;
- the same machinery can be reused for non-email domains.

### FAIL / REVISE if

- `uri` reduces to a branded type under another name;
- evidence/provenance creates excessive annotation burden;
- versioning makes ordinary mutable programming unusable;
- the compiler must solve undecidable/general theorem-proving problems for routine code;
- path-sensitive facts become unsound;
- established type/refinement/typestate systems express the same model substantially more simply;
- Tamil terminology adds more cognitive load than semantic value.

---

## 22. Preliminary result

**Status: promising, not accepted.**

Experiment 002 strengthens one aspect of the Aytham hypothesis:

> `uri` may be most useful when modeled as an **evidence-bearing contextual qualification** rather than merely a property or refinement predicate.

It also introduces a potentially important principle:

> Semantic facts attach to a value's meaning/lineage, not to a mutable variable name.

These ideas need formal comparison before becoming part of the language specification.

---

## 23. Implication for the Aytham semantic model

If this experiment survives comparison, a more precise provisional model becomes:

```text
peyar
  = denotable semantic subject/value

vinai
  = transformation that may consume, establish,
    preserve, invalidate, or require facts

uri
  = contextual qualification + optional evidence/provenance

vēṟṟumai-inspired relation
  = semantic role between subjects/actions

punarchi-inspired composition
  = compatibility of produced and required meaning graphs

idai
  = still-open research space for flow/connection/choice

poruḷ
  = resolved contextual meaning record/graph
    (Aytham borrowing, not historical equivalence)
```

---

## 24. Next experiment

Experiment 003 should test **state/protocol transition**, because it stresses a different question:

```text
Connection: Closed
   ↓ Open
Connection: Open
   ↓ Authenticate
Connection: Authenticated
   ↓ Send
Connection: Authenticated
   ↓ Close
Connection: Closed
```

This will reveal whether Aytham's lineage + uri + vinai model merely recreates typestate, or whether relation/provenance/composition adds enough value to justify a distinct model.

Before Experiment 003 is accepted as architecture, deepen source study for:

- வேற்றுமையியல்;
- பெயரியல்;
- வினையியல்;
- இடையியல்;
- உரியியல்;
- புணரியல்.
