# Nannūl Commentary Notebook 002

Status: **commentary comparison in progress**  
Purpose: add commentarial evidence to the Aytham research model before any Nannūl-derived terminology or semantic feature is frozen.

## Source set

### A. Mayilaināthar commentary

Tamil Virtual Academy source supplied by the project owner:

- `https://www.tamilvu.org/library/l0901/html/l0901ind.htm`
- title displayed by TVA: **நன்னூல் மூலமும் மயிலைநாதருரையும்**
- edition identity displayed by TVA: **மகாமகோபாத்தியாய டாக்டர் உ.வே.சாமிநாதையர்**
- TVA contents page places:
  - உயிரீற்றுப்புணரியல் at printed pp. 61–87;
  - மெய்யீற்றுப்புணரியல் at pp. 87–102;
  - உருபு புணரியல் at pp. 102–112;
  - வினையியல் at pp. 161–180;
  - பொதுவியல் at pp. 180–229;
  - இடைச் சொல்லியல் at pp. 229–239;
  - உரிச்சொல்லியல் at pp. 240–249.

TVA's bibliographic corpus records the U. Vē. Cāminātaiyar first edition as Chennai, Vaijayanti Press, 1918.

### B. Śaṅkara Namaccivāyar / Sivajñāna Munivar Virutti commentary

Tamil Virtual Academy source supplied by the project owner:

- `https://www.tamilvu.org/library/l0902/html/l0902ind.htm`
- title displayed by TVA: **பவணந்தி முனிவர் இயற்றிய நன்னூல் மூலமும் சங்கர நமச்சிவாயர் செய்து சிவஞான முனிவரால் திருத்தப்பட்ட புத்தம் புத்துரை என்னும் விருத்தியுரையும்**

TVA's bibliographic corpus separately records an early printed edition of the Śaṅkara Namaccivāyar commentary revised by Sivajñāna Munivar, published by Āṟumuka Nāvalar at Yāḻppāṇam in 1851.

The legacy `l0902` frame navigation is not consistently exposed through TVA's present searchable endpoints. Therefore this notebook distinguishes:

- **edition identity** from the supplied `l0902` TVA page;
- **specific searchable nūṟpā/commentary text** from TVA's indexed Nannūl corpus where available;
- later TVA teaching pages only as secondary explanatory aids, not as substitutes for the supplied commentaries.

---

# 1. Numbering must be edition-aware

The action-frame rule beginning:

```text
செய்பவன் கருவி நிலம் செயல் காலம்
செய்பொருள் ஆறும் தருவது வினையே
```

appears as **(319)** on the Mayilaināthar/U. Vē. Cā. page at the beginning of வினையியல், while multiple other TVA presentations number the same rule **320**.

### Aytham archival decision

Never cite this rule as simply `Nannūl 320` without an edition field.

Use a source key such as:

```text
NAN-MAYILAI-UVE-1918: vinai opening rule = 319
NAN-VIRUTTI-TVA: corresponding rule = 320
```

until a normalized cross-edition concordance is built.

This is not a trivial bibliographic detail: Aytham's source-to-design provenance must survive edition-number differences.

---

# 2. Mayilaināthar on வினை — strong support for an action frame, not for a function signature

At the opening of வினையியல், Mayilaināthar explains the rule through six associated meanings:

```text
கருத்தா / செய்பவன்
கருவி
நிலம்
தொழில் / செயல்
காலம்
செயப்படுபொருள்
```

His explanation explicitly says that the verb/action expression makes these six meanings intelligible.

### Source consequence

This is stronger evidence than the bare Nannūl mūlam for treating `vinai` as something understood through a **structured semantic frame**.

It does **not** mean:

```text
Nannūl action frame == Aytham ActionFrame
```

and it does not prescribe a universal six-slot programming API.

### Aytham consequence

The paper semantics should prefer:

```text
ActionFrame {
    action
    participant_relations
    circumstantial_relations
    temporal_context
}
```

rather than treating an operation's primary meaning as:

```text
Function(T1, T2, T3) -> R
```

The six Nannūl dimensions become a historical comparison point, not fixed Aytham role names.

### Additional insight

Mayilaināthar's commentary also treats the six as meanings revealed by the verbal form rather than necessarily six overt adjacent arguments. That is relevant to Aytham's distinction between:

```text
surface syntax
vs
resolved semantic relations
```

A programmer need not always spell every relation in one positional list if the semantic graph can establish it unambiguously.

---

# 3. General vs partial realization — do not require every frame edge to be overt

TVA's searchable Nannūl commentary tradition notes that the six-fold action meaning need not always appear as six explicit surface components; some can be absent/implicit according to the construction.

### Aytham question

Should an ActionFrame distinguish:

```text
required semantic relation
explicitly supplied relation
inferred relation
contextually unavailable relation
not-applicable relation
```

rather than equating "not written" with "not part of the meaning"?

This is now a legitimate research problem for Aytham, but it must be compared against:

- implicit arguments;
- argument structure;
- semantic-role labelling;
- frame semantics;
- record/row inference;
- logic-variable inference.

No implicit-relation feature is accepted yet.

---

# 4. புணர்ச்சி — commentary strengthens boundary-sensitive composition

Mayilaināthar's commentary on the opening உயிரீற்றுப்புணரியல் rule explains joining in terms of two forms meeting:

```text
தன்னொடும் / பிறிதொடும்
அல்வழிப் பொருள் / வேற்றுமைப் பொருள்
நிலைமொழி / வருமொழி
இயல்பு / விகாரம்
```

The critical design lesson is not phonological imitation.

### Source-level pattern

The interpretation of a join depends on:

1. properties of the left participant;
2. properties of the right participant;
3. the semantic relation under which they meet;
4. whether the participants remain unchanged or undergo a rule-governed transformation.

### Aytham consequence

This gives a better historical analogy for **boundary-sensitive composition** than a simple pipeline operator.

A provisional Aytham composition judgment should therefore be able to inspect:

```text
producer meaning
consumer requirement
relationship/context of joining
preserved facts
derived facts
invalidated facts
required transformation/adaptor
```

Possible outcomes:

```text
DIRECT
TRANSFORMED
REQUIRES_ADAPTOR
AMBIGUOUS
REJECTED
```

The word `punarcci` remains a research inspiration, not yet the specification term.

---

# 5. இடை — the Virutti tradition strongly supports mediation/dependence

The searchable TVA Nannūl commentary for the இடை rule (commonly numbered 420) describes several kinds of `idai` and emphasizes that they do not operate independently; one or many may join with noun/verb forms internally or externally.

The rule family includes functions associated with:

- case marking;
- verbal morphology;
- connective increments;
- comparison;
- contextual meaning;
- prosodic/rhythmic filling;
- indication.

### Design correction confirmed

The earlier hypothesis:

```text
idai = control-flow connector
```

remains rejected.

The more defensible research interpretation is:

> **idai-inspired semantics concerns mediation: relational material whose meaning depends on the forms/structures it connects or modifies.**

### Aytham questions

A semantic connector may eventually carry one or more of:

```text
relation kind
ordering/causality
evidence flow
temporal relation
role remapping
branch condition
composition authority
```

But `idai` should not become a generic bucket for `if`, `;`, `|>`, `await`, and punctuation.

---

# 6. உரி — Nannūl narrows and reorganizes the source tradition

Nannūl's rule commonly numbered 442 characterizes `uriccol` through `paṇpu`/quality, one or multiple qualities, and its relation to peyar/vinai and poetic usage.

This is notably different in organization and emphasis from the opening of Tolkāppiyam Uriyiyal, where lexical-semantic multiplicity, indication, quality, contextual interpretation, and relation to peyar/vinai are foregrounded.

### Important comparative conclusion

`uri` is **not a timeless invariant technical category with one obvious computational equivalent**.

Across the two grammatical systems, the material is reorganized and characterized differently.

### Aytham consequence

Do not restore:

```text
uri = refinement type
```

as a formal language term merely because Nannūl discusses `paṇpu`.

For Experiment 002 continue using the neutral modern term:

```text
qualification claim
```

until we decide whether a Tamil technical term genuinely improves the model.

Potential future separation:

```text
property/quality
established claim
contextual interpretation
lexical qualifier
```

These may not belong to one programming construct.

---

# 7. பொதுவியல் — cross-cutting rules become more important

Mayilaināthar introduces பொதுவியல் as addressing matters that look across the four word families rather than belonging only to one of them.

This supports the conclusion from Comparative Notebook 001:

> Aytham may need a **cross-cutting semantic-law layer** rather than forcing every language rule into Peyar/Vinai/Idai/Uri-inspired categories.

Candidate modern separation:

```text
local action/value/relationship rules
        +
cross-cutting well-formedness laws
        +
domain extension laws
```

Possible cross-cutting concerns:

- ambiguity;
- admissible variation;
- relation completion;
- preservation/invalidation;
- temporal consistency;
- trust-boundary consistency;
- composition coherence.

Do not name this layer `potuviyal` yet.

---

# 8. Commentary comparison changes the Aytham ActionFrame

Before commentarial study, the provisional frame was approximately:

```text
ActionFrame {
    action
    participants
    circumstances
    temporal_context
    effects
    established_claims
}
```

The commentary evidence suggests a stronger research representation:

```text
ActionFrame {
    action

    relations: [
        {
            role
            subject
            origin: explicit | inferred | derived | contextual
            evidence?
        }
    ]

    temporal_context

    requires_claims
    establishes_claims
    preserves_claims
    invalidates_claims

    effects
    capabilities

    output_relations
}
```

Why the addition of `origin`?

Because grammatical action meaning need not be reducible to a flat list of overt surface arguments. Aytham should test whether semantic relations can be explicit, inferred, or derived while remaining inspectable and unambiguous.

This is **AYTHAM DESIGN**, not a claim about either commentary.

---

# 9. New falsification tests

The commentaries raise stricter tests for Aytham.

## F1 — Ordered arguments versus relation frame

Implement the same operation as:

```text
transfer(source, destination, amount)
```

and as an ActionFrame.

Aytham passes only if the frame gives measurable benefit in:

- role safety;
- partial information handling;
- diagnostics;
- composition;
- tooling/discovery.

## F2 — Missing but inferable relation

Can a relation be omitted from the surface while still being uniquely recoverable?

If yes, tooling must show why it was inferred.

If two valid frames exist, compilation must require disambiguation.

## F3 — Boundary transformation

Given two individually valid frames, classify their boundary as:

```text
DIRECT
TRANSFORMED
REQUIRES_ADAPTOR
AMBIGUOUS
REJECTED
```

and produce domain-level reasons.

## F4 — Mediation has semantic content

If an `idai`-inspired connector is removed, does program meaning change in a way not reducible to punctuation/evaluation order?

If not, the concept should not enter the core language.

---

# 10. Source provenance policy for Nannūl

Every future Nannūl-derived research note should record:

```text
work: Nannūl
nūṟpā incipit
nūṟpā number in this edition
edition/commentary
TVA source URL
printed page where available
source text
commentary observation
Aytham interpretation
status: SOURCE | INTERPRETATION | AYTHAM DESIGN
```

Never silently reconcile conflicting nūṟpā numbering.

---

# 11. Current conclusions

### Strengthened

- action-centred relation/frame research;
- surface form versus resolved semantic relations;
- boundary-sensitive checked composition;
- `idai` as mediation/dependence rather than generic control;
- need for a cross-cutting semantic-law layer.

### Weakened / kept provisional

- `uri` as the name of Aytham refinement/qualification semantics;
- a flat `Peyar | Vinai | Idai | Uri` semantic enum;
- fixed six-role programming APIs;
- direct equation of punarcci with software function composition.

### New mandatory prior-art comparisons

- Fillmore-style case grammar;
- thematic/semantic roles;
- frame semantics / FrameNet-like representations;
- semantic-role labelling;
- knowledge graphs;
- graph rewriting;
- row/record polymorphism;
- implicit argument inference;
- type-directed synthesis/planning.

---

# 12. Next source work

Before adding Sangam literature:

1. build a Nannūl cross-edition nūṟpā concordance for Aytham-relevant rules;
2. deepen the two commentaries around:
   - action-frame rule (319/320 discrepancy);
   - 151 புணர்ச்சி;
   - 420 இடை;
   - 442 உரி;
   - selected பொதுவியல் rules;
3. then run the ActionFrame against non-Tamil modern prior art.

Sangam literature should enter after this as an **attested-usage stress-test layer**, not as another source of terminology.
