# Experiment 011 — Minimal Aytham Notation Prototype v0.1

## Status

Experimental notation study only. **This is not Aytham syntax.**

No keyword, punctuation rule, block rule, file extension, or Tamil term in this document is frozen.

## Objective

Test whether the semantic core defined by the current Aytham research can be written naturally and compactly enough to support a future executable notation.

The semantic core under test is:

- Entity
- Relation
- Action
- Claim
- Transformation

Supporting semantic information:

- Evidence
- Context
- Authority
- confidence / epistemic status
- lineage

The primary question is:

> Can Aytham express semantic roles, requirements, established meaning, evidence, and transformations more clearly than an ordinary positional function notation without becoming verbose or pseudo-natural-language?

This experiment deliberately tests notation before grammar.

---

## Constraints

A successful notation should satisfy all of the following:

1. **Meaning before syntax** — semantic roles must remain visible.
2. **No one-to-one keyword translation** — do not recreate Python/C/Java with Tamil words.
3. **Tamil-native possibility** — Tamil should be able to serve as a natural surface without requiring English identifiers.
4. **Machine resolvability** — notation must have one inspectable semantic graph after resolution.
5. **Explainability** — requirements, missing claims, provenance, and ambiguity must remain available to tooling.
6. **Compactness** — ordinary programming must not require a paragraph of metadata for every operation.
7. **Progressive disclosure** — evidence/context/authority should be expressible when needed without overwhelming simple cases.
8. **No historical overclaim** — Tamil grammatical terms, where tested, are Aytham design terms unless explicitly documented as source quotations.

---

# Candidate notation A — Structured semantic blocks

This is the most explicit candidate.

The English labels below are semantic placeholders, not intended final keywords.

```text
entity Customer {
    email = "person@example.org"
}

claim EmailSyntaxValid {
    subject = Customer.email
    status = established
    by = ParseEmail
}

action SendSensitiveMessage {
    recipient = Customer.email

    requires {
        EmailSyntaxValid
        OwnershipVerified
    }
}
```

A Tamil-surface sketch of the same structure might look like:

```text
உருவம் வாடிக்கையாளர் {
    மின்னஞ்சல் = "person@example.org"
}

உறுதி மின்னஞ்சல்_வடிவம் {
    சார்பு = வாடிக்கையாளர்.மின்னஞ்சல்
    நிலை = நிறுவப்பட்டது
    வழி = மின்னஞ்சல்_பகுப்பு
}

வினை பாதுகாப்புச்_செய்தி_அனுப்பு {
    பெறுநர் = வாடிக்கையாளர்.மின்னஞ்சல்

    தேவை {
        மின்னஞ்சல்_வடிவம்
        உரிமை_உறுதி
    }
}
```

### Strengths

- explicit semantic categories;
- easy to parse;
- diagnostics can point to individual semantic clauses;
- roles are visible rather than positional;
- evidence and context can be added as optional blocks.

### Weaknesses

- risks looking like a conventional declarative DSL;
- can become verbose;
- `entity`, `claim`, `action` may become only renamed schema constructs if semantics do not differ at runtime;
- Tamil labels could become decorative if the underlying model is not distinctive.

### Result

**PASS as a baseline notation.**

Useful for machine clarity, but not yet sufficient to establish an Aytham identity.

---

# Candidate notation B — Relation-first / role-labelled action notation

This candidate makes semantic relations primary.

```text
Transfer {
    source       AccountA
    destination  AccountB
    amount       Money100

    requires
        AccountA : debit_authorized
        Money100 : positive

    establishes
        transfer_completed

    changes
        AccountA.balance
        AccountB.balance

    evidence
        TransactionRecord
}
```

Tamil-surface sketch:

```text
பணமாற்றம் {
    மூலம்       கணக்கு_அ
    இலக்கு      கணக்கு_ஆ
    தொகை        ரூ100

    தேவை
        கணக்கு_அ : பற்று_அனுமதி
        ரூ100    : நேர்மதிப்பு

    நிறுவு
        பரிமாற்றம்_நிறைவு

    மாற்று
        கணக்கு_அ.இருப்பு
        கணக்கு_ஆ.இருப்பு

    சான்று
        பரிமாற்றப்_பதிவு
}
```

This is not intended as final Tamil terminology. In particular, `மூலம்`, `இலக்கு`, `தேவை`, `நிறுவு`, `மாற்று`, and `சான்று` are notation probes.

### Strengths

- participant roles are immediately visible;
- strongly supports ActionFrame reasoning;
- avoids positional ambiguity;
- requirements and established meaning are near the action that uses them;
- maps naturally to explanation diagnostics.

### Weaknesses

- every action may need domain-specific role labels;
- role names need schema/type validation;
- relation vocabulary could become inconsistent across packages;
- without a role/type discipline this could become named arguments plus contracts.

### Result

**STRONG PASS for continued experimentation.**

This is currently the clearest expression of the Aytham research direction because it places semantic relationships at the centre of the notation rather than treating them as metadata around a conventional function call.

---

# Candidate notation C — Tamil sentence-like notation

This candidate asks whether computation can be written in a surface closer to Tamil clause order.

Illustrative only:

```text
கணக்கு_அ இலிருந்து
கணக்கு_ஆ க்கு
ரூ100 ஐ
பணமாற்றம் செய்
```

With requirements:

```text
கணக்கு_அ இன் பற்று_அனுமதி நிறுவப்பட்டால்
கணக்கு_ஆ செயலில் இருந்தால்
ரூ100 நேர்மதிப்பு என நிறுவப்பட்டால்

கணக்கு_அ இலிருந்து
கணக்கு_ஆ க்கு
ரூ100 ஐ
பணமாற்றம் செய்
```

The attractive idea is that role marking could potentially arise through Tamil-like relational marking rather than positional order.

### Strengths

- recognizably Tamil-oriented rather than English DSL structure;
- may expose semantic roles naturally;
- potentially connects with வேற்றுமை-inspired role research more deeply than named parameters do.

### Weaknesses

- natural-language resemblance introduces ambiguity quickly;
- case-marker-like forms and software role systems are not identical;
- sandhi/morphology/orthography could complicate parsing;
- programmer tooling needs a canonical resolved form independent of surface order;
- risks creating a controlled natural language instead of a practical programming language;
- historical Tamil grammar must not be reduced to software parameter syntax.

### Result

**HOLD.**

This is intellectually important and should remain an experimental surface direction, but it is too early to make it the primary notation.

The semantic graph must be stable first.

---

# Candidate notation D — Graph-like relation notation

A fourth candidate was added because Aytham's research repeatedly returns to semantic graphs.

```text
AccountA -[source-of]-> Transfer#1
AccountB -[destination-of]-> Transfer#1
Money100 -[amount-of]-> Transfer#1

Transfer#1 -[requires]-> DebitAuthorized(AccountA)
Transfer#1 -[requires]-> Positive(Money100)
Transfer#1 -[establishes]-> TransferCompleted
```

### Strengths

- extremely explicit resolved meaning;
- close to the internal semantic graph;
- excellent for tooling, debugging, serialization, and formal tests.

### Weaknesses

- poor everyday programming ergonomics;
- graph syntax is visually noisy;
- makes simple actions look like database triples;
- likely better as an intermediate representation than a user-facing language.

### Result

**PASS as a candidate semantic IR / debugging representation, not as the default surface syntax.**

---

# Cross-case notation tests

## Case 1 — Validated email flow

The notation must represent:

```text
raw input
  -> ParseEmail
  -> email_syntax_valid
  -> VerifyOwnership
  -> ownership_verified
  -> SendSensitiveMessage
```

A compact role/block candidate:

```text
ParseEmail {
    input rawEmail

    establishes
        rawEmail : email_syntax_valid
}

VerifyOwnership {
    subject rawEmail

    requires
        rawEmail : email_syntax_valid

    establishes
        rawEmail : ownership_verified

    evidence
        OwnershipChallenge
}

SendSensitiveMessage {
    recipient rawEmail

    requires
        rawEmail : ownership_verified
}
```

### Diagnostic target

If `VerifyOwnership` is omitted, tooling should be able to report:

```text
SendSensitiveMessage cannot execute.

Required:
    rawEmail : ownership_verified

Available:
    rawEmail : email_syntax_valid

Candidate bridge:
    VerifyOwnership
```

The notation therefore must preserve enough structure for semantic bridge discovery.

---

## Case 2 — Banking transfer

```text
Transfer {
    source AccountA
    destination AccountB
    amount Rs100

    requires
        AccountA : debit_authorized
        AccountB : active
        Rs100 : positive

    establishes
        Transfer#1 : completed

    evidence
        LedgerEntry#991
}
```

The important feature is not the block syntax. It is that `AccountA`, `AccountB`, and `Rs100` participate through explicit semantic roles.

The same `Account` entity may be `source` in one action and `destination` in another without changing nominal type.

---

## Case 3 — Interpretation with uncertainty

```text
InterpretPoem {
    subject Poem001

    observes
        MountainImagery

    context
        AkamReadingContext

    proposes
        Claim#KurinjiAssociation

    evidence
        PoemText
        CommentaryA

    confidence
        inferred
}
```

This case exposes a limitation: `proposes` / `observes` are not yet part of the formal five-element core. They may be special semantic relations/actions rather than new primitives.

The notation should therefore avoid inventing a new top-level primitive every time a domain needs a verb.

---

# Surface vs resolved meaning

Experiment 011 makes an important distinction:

```text
Surface notation
      ↓
Semantic resolution
      ↓
Canonical semantic graph
```

Two future surface forms could potentially resolve to the same graph.

For example:

```text
Transfer {
    source A
    destination B
}
```

and a future Tamil-oriented role-marked form could resolve to:

```text
A -[source-of]-> Transfer#1
B -[destination-of]-> Transfer#1
```

This separation is important because it allows Aytham to explore genuinely Tamil-oriented notation without making natural-language surface form the semantic authority.

---

# Progressive disclosure test

Aytham must not require full provenance blocks for trivial code.

Minimum useful action:

```text
CalculateTotal {
    invoice Order42
    establishes Order42 : total_calculated
}
```

Expanded regulated/audited action:

```text
ApproveLoan {
    applicant Customer42

    requires
        Customer42 : identity_verified
        Customer42 : income_verified
        Application42 : risk_assessed

    establishes
        Application42 : approved

    evidence
        IdentityCheck#7
        IncomeCheck#9
        RiskAssessment#12

    context
        jurisdiction = IN
        policy = RetailLoanPolicy2026
}
```

### Finding

**Progressive disclosure is mandatory.**

Evidence, authority, context, confidence, and detailed lineage must be available but should not be compulsory ceremony when semantics do not require them.

---

# Key findings

## F-011-1 — Role-labelled notation is the strongest current surface candidate

A role-labelled action block best exposes the current semantic model while remaining parseable and relatively compact.

This does **not** freeze block syntax or any labels.

## F-011-2 — Sentence-like Tamil notation remains valuable but must resolve to a canonical graph

Tamil relational/case-inspired surface notation is worth later experimentation, especially because Aytham's research is interested in semantic roles rather than positional arguments.

However, natural-language resemblance must not introduce hidden compiler guesses.

## F-011-3 — The graph form is better treated as semantic IR

A graph/triple representation is excellent for:

- compiler inspection;
- diagnostics;
- provenance visualization;
- conformance tests;
- serialization.

It is too verbose for ordinary source code.

## F-011-4 — Aytham needs a distinction between core semantics and domain verbs

Words such as `observes`, `proposes`, `verifies`, `authorizes`, and `approves` should not automatically become new language primitives.

They should normally resolve through Action, Relation, Claim, and Transformation.

## F-011-5 — Surface syntax must not be the semantic authority

The canonical meaning of a program should be its resolved semantic structure, not the exact surface ordering of words.

This is necessary if Aytham later supports more than one surface style, bilingual tooling, or Tamil role-marked notation.

---

# Provisional notation architecture

The best current direction is:

```text
Human-facing notation
    role-labelled semantic blocks
            |
            v
Semantic resolver
            |
            v
Canonical semantic graph / IR
            |
       +----+----+
       |         |
   Validator   Explanation engine
       |
   Runtime / execution adapter
```

A future Tamil-native notation can be designed over the same semantic graph.

---

# Falsification criteria

Experiment 011 should be considered a failure if later tests show that:

1. role-labelled blocks are indistinguishable in practice from named arguments plus contracts;
2. the canonical semantic graph adds no useful diagnostics or validation;
3. semantic metadata makes ordinary programming too verbose;
4. Tamil-oriented surface forms require unsafe ambiguity resolution;
5. programmers must understand the full graph model to write simple code;
6. the notation cannot express ordinary control/computation without continuously inventing new semantic primitives.

---

# Outcome

**Experiment 011: provisional PASS.**

The semantic model appears expressible without immediately becoming a conventional Tamil-keyword language.

The strongest current direction is:

- role-labelled action notation for human-facing experiments;
- a canonical relation graph as semantic IR;
- progressive disclosure for evidence/context/authority;
- Tamil-native sentence/role marking retained as a later surface experiment rather than prematurely frozen syntax.

No final syntax is authorized by this experiment.

---

# Recommended next experiment

## Experiment 012 — Semantic Resolution and Canonical Graph

Define precisely how a notation such as:

```text
Transfer {
    source A
    destination B
    amount M
}
```

resolves into a canonical semantic graph.

The experiment should specify:

1. identity allocation;
2. role resolution;
3. claim attachment;
4. explicit vs inferred relation provenance;
5. requirement matching;
6. transformation/lineage edges;
7. ambiguity rejection;
8. canonical graph serialization;
9. explanation queries such as `why?`, `how-known?`, and `what-missing?`.

Only after canonical semantic resolution is stable should Aytham perform a serious Tamil surface-syntax experiment.