# Experiment 001 — Role Graph: Safe Money Transfer

Status: **paper experiment**  
Purpose: test whether a வேற்றுமை-inspired relational model offers more than named parameters or Tamil terminology.

## 1. Research claim under test

Aytham may treat the meaning of a computation as a **semantic relation graph** rather than primarily as an ordered list of statements.

Working categories:

- **பெயர் · peyar** → entity/value nodes;
- **வினை · vinai** → transformation/action nodes;
- **வேற்றுமை-inspired roles** → labelled semantic relationships between entities and actions;
- **உரி · uri** → constraints/refinements on nodes or relationships;
- **இடை · idai** → relationships among actions/computations;
- **புணர்ச்சி-inspired rules** → laws governing whether two semantic subgraphs may compose;
- **பொருள் · poruḷ** → Aytham's provisional name for the fully resolved contextual meaning graph.

This is **AYTHAM DESIGN**, not a claim that Tolkāppiyam specifies graph computation.

---

## 2. Conventional problem

A conventional API might expose:

```text
transfer(accountA, accountB, 100)
```

If both `accountA` and `accountB` have type `Account`, the type system alone cannot explain which account plays which role.

Named parameters improve readability:

```text
transfer(from=accountA, to=accountB, amount=100)
```

Wrapper/newtypes can improve static safety:

```text
transfer(SourceAccount(accountA), DestinationAccount(accountB), Money(100))
```

Aytham must therefore demonstrate something beyond these established techniques.

---

## 3. Aytham hypothesis: role is a relation, not a wrapper type

Instead of changing the identity/type of an account, Aytham models its role **relative to a particular action**.

Semantic graph:

```text
                 ┌───────────────┐
                 │   Transfer    │
                 │    (vinai)    │
                 └───────┬───────┘
                         │
        ┌────────────────┼────────────────┐
        │                │                │
     source          destination        amount
        │                │                │
        ▼                ▼                ▼
   ┌─────────┐       ┌─────────┐      ┌─────────┐
   │Account A│       │Account B│      │ 100 INR │
   │ (peyar) │       │ (peyar) │      │ (peyar) │
   └─────────┘       └─────────┘      └─────────┘
```

The important distinction is:

```text
Account A is still an Account.

"source" is not its permanent type.
"source" is its semantic relation to this Transfer.
```

The same account could simultaneously be:

- destination of one transfer;
- source of another transfer;
- owner of a standing instruction;
- subject of an audit;

without creating multiple wrapper identities.

---

## 4. Syntax-neutral semantic record

Do not treat this notation as proposed Aytham syntax.

```yaml
vinai: transfer
participants:
  - peyar: account_a
    type: Account
    role: source
  - peyar: account_b
    type: Account
    role: destination
  - peyar: 100_INR
    type: Money<INR>
    role: amount
    uri:
      - positive
      - non_zero
effects:
  - ledger_write
  - balance_change
result:
  type: TransferReceipt
  role: receipt
```

The compiler resolves this into a meaning graph independent of incidental source ordering.

---

## 5. Role schema

The transfer `vinai` declares relationships rather than only positional parameters.

Conceptual schema:

```text
vinai Transfer

requires relation:
    source      -> Account
    destination -> Account
    amount      -> Money<C>

requires uri:
    amount.positive
    amount.non_zero
    source != destination

produces relation:
    receipt -> TransferReceipt

causes:
    balance_change(source, -amount)
    balance_change(destination, +amount)
    ledger_write
```

A semantic role therefore has:

```text
role name
expected value domain
cardinality
constraints
relationship to the vinai
possibly provenance/capability rules
```

---

## 6. Order independence experiment

If meaning is resolved from relationships, these source serializations should be semantically equivalent *if the eventual syntax permits them*:

```text
source=A destination=B amount=100 transfer
```

```text
amount=100 transfer destination=B source=A
```

```text
destination=B source=A transfer amount=100
```

Aytham should not force order independence merely to imitate natural-language word order. The value is that **semantic identity comes from role relationships rather than position**.

The eventual surface syntax may still impose a canonical order for readability.

---

## 7. Error experiments

### E1 — Missing role

Graph:

```text
A --source--> Transfer
100 INR --amount--> Transfer
```

Expected diagnostic concept:

```text
Transfer is incomplete.
Required relationship missing: destination -> Account.
```

Not merely:

```text
Expected 3 arguments, got 2.
```

### E2 — Same value in incompatible roles

Graph:

```text
A --source------> Transfer
A --destination-> Transfer
```

If the Transfer rule requires `source != destination`, reject:

```text
The same Account participates as both source and destination,
but Transfer requires those roles to refer to distinct accounts.
```

### E3 — Correct type, wrong qualification

```text
-100 INR --amount--> Transfer
```

Underlying type is valid `Money<INR>`, but `uri: positive` fails.

Expected diagnostic:

```text
The value satisfies the Money<INR> domain but does not satisfy
Transfer's `positive` qualification for the amount role.
```

### E4 — Correct value, unavailable capability

If `source` requires debit capability and the account is read-only:

```text
The Account can participate in a read relationship,
but the source role of Transfer requires debit capability.
```

This tests whether roles can carry operational requirements rather than being labels only.

---

## 8. Why this may be more than named parameters

Named parameters are local labels on a call site.

Aytham's proposed roles are intended to be **semantic relations that survive beyond one syntactic call**.

For example, an earlier computation may produce:

```text
Account A --eligible_source_for--> TransferDomain
```

or a workflow may establish:

```text
UserChoice --selected_source--> Account A
```

The transfer compiler/tooling can use that semantic information without wrapping `Account A` in a new nominal type.

Roles may therefore participate in:

- inference;
- composition;
- permissions;
- API discovery;
- diagnostics;
- state/protocol checks;
- visualization.

This is the first place the hypothesis becomes meaningfully different from syntax sugar.

---

## 9. Why this may be more than branded/newtypes

A branded type commonly turns one representation into multiple static identities:

```text
SourceAccount
DestinationAccount
```

That is useful but can create combinatorial wrapper types when roles are contextual.

A role graph instead says:

```text
entity type: Account
relation now: source-of Transfer#123
```

The relation is contextual and can be scoped to the semantic graph rather than permanently altering the entity's type.

Research questions:

- Can roles remain statically safe without becoming hidden wrapper types internally?
- Can role inference be sound?
- How are role scopes represented?
- Can one entity hold multiple roles simultaneously?
- Can conflicting roles be declared by a vinai-specific rule?

---

## 10. Uri experiment

`uri` is modeled as a property/refinement that may attach to:

- a peyar node;
- a role edge;
- a vinai node;
- possibly an idai/composition edge.

Example:

```text
100 INR
  type: Money<INR>
  uri: positive, non_zero

source edge
  uri: debit_allowed

Transfer
  uri: atomic
```

This allows Aytham to ask whether constraints should live exactly where their meaning applies instead of being forced into one global type.

---

## 11. Vinai/effect experiment

Transfer is not only a function from inputs to receipt.

Its meaning graph includes observable consequences:

```text
Transfer
  ├─ changes balance(source)
  ├─ changes balance(destination)
  └─ writes ledger
```

A future Aytham effect model may therefore treat effects as part of the vinai's semantic edges.

This can support questions such as:

- May this vinai occur inside an atomic composition?
- Can this vinai be retried?
- Does this vinai require network/database capability?
- Can a pure computation compose before/after it freely?

---

## 12. Punarchi/composition experiment

Suppose a second vinai sends a receipt:

```text
send_receipt
requires:
    receipt -> VerifiedTransferReceipt
```

But Transfer produces only:

```text
receipt -> TransferReceipt
```

Then direct composition fails unless a verification vinai exists:

```text
Transfer
   ↓ receipt: TransferReceipt
VerifyReceipt
   ↓ receipt: VerifiedTransferReceipt
SendReceipt
```

A composition rule checks the boundary between subgraphs:

```text
producer output poruḷ
    ↕ compatibility law
consumer required poruḷ
```

The desired diagnostic is relational:

```text
Transfer produces a receipt without the `verified` uri.
SendReceipt requires the receipt relation to carry `verified`.
Insert or compose a vinai that establishes that qualification.
```

---

## 13. Possible formal representation

One provisional judgment form:

```text
Γ ⊢ p : T ▷ R ▷ U ▷ E
```

where:

- `p` = computational form;
- `T` = value domain/type;
- `R` = role relations;
- `U` = uri/refinements;
- `E` = effects/capabilities.

A vinai is then not simply:

```text
A × B → C
```

but closer to a graph transformation:

```text
required semantic graph
        ↓
      vinai
        ↓
produced semantic graph + effects
```

This representation is provisional and must be compared against established type/effect/graph calculi.

---

## 14. Compiler implication if hypothesis survives

A possible architecture becomes:

```text
source syntax
    ↓
எழுத்து/source-form analysis
    ↓
சொல்/form construction
    ↓
semantic graph building
    ├─ peyar nodes
    ├─ vinai nodes
    ├─ role edges
    ├─ uri constraints
    └─ idai/composition edges
    ↓
poruḷ resolution
    ├─ type/domain checking
    ├─ role checking
    ├─ refinement checking
    ├─ effect checking
    └─ composition checking
    ↓
execution graph / IR
    ↓
interpreter or compiler backend
```

This is significantly stronger than simply naming lexer/parser/semantic-analysis stages in Tamil.

---

## 15. Comparison obligations

Before accepting this feature, compare it against:

- named arguments;
- nominal/newtype wrappers;
- structural and branded types;
- refinement types;
- dependent types;
- effect systems;
- capability systems;
- session/protocol types;
- logic programming;
- semantic networks;
- dataflow languages;
- graph IRs;
- workflow DSLs.

The question is not whether similar ingredients exist. They certainly do.

The question is whether Aytham's **unified role + qualification + action + composition meaning graph** offers a coherent, learnable, useful model.

---

## 16. Pass/fail criteria

### PASS if

- role errors are prevented statically without proliferating nominal wrapper types;
- role information composes across multiple operations;
- uri constraints attach naturally to the relevant semantic relation;
- effect information improves safe composition;
- diagnostics are materially clearer;
- the model can be specified formally without special cases.

### FAIL / REVISE if

- roles reduce to named parameters;
- internal implementation is only hidden wrapper types with no additional semantics;
- graph notation makes simple code harder to reason about;
- inference becomes ambiguous or unsound;
- conventional refinement/effect systems express the same model more clearly;
- Tamil grammatical terminology adds explanation cost without computational benefit.

---

## 17. Next experiment

If Experiment 001 remains promising, Experiment 002 should test **validated data flow** rather than finance:

```text
raw text
  ↓ parse
email-shaped text
  ↓ verify
verified email
  ↓ send
```

This will test whether `uri`, `vinai`, and `punarchi` work outside the role-heavy transfer example.
