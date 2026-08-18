# Nannūl Source Concordance — Aytham Research

Status: **active / incomplete**  
Purpose: keep Aytham's Nannūl citations stable across editions/commentaries whose nūṟpā numbering differs.

## Citation principle

Aytham should identify a Nannūl rule primarily by:

```text
incipit + edition/commentary + local number + page/source
```

not by bare nūṟpā number.

Recommended source key:

```text
NAN:<edition-key>:<local-number>:<incipit-slug>
```

Example:

```text
NAN:MAYILAI-UVE:319:seybavan-karuvi
NAN:TVA-COMMON:320:seybavan-karuvi
```

---

## Why numbering differs

The introductory material in the Mayilaināthar/U. Vē. Cāminātaiyar TVA edition notes that Virutti-commentary traditions can show **462** sūtras, while the Mayilaināthar reckoning yields **461** because `முன்னோர் மொழிபொருளே...` is treated there as a quotation rather than as an independent Nannūl rule.

This appears to create at least some one-number offsets between the Mayilaināthar numbering and other Nannūl presentations.

Do not assume the offset is globally uniform until the entire text has been aligned.

---

# Current high-priority concordance

| Research concept | Incipit / identifying text | Mayilaināthar / U. Vē. Cā. TVA | Other TVA / common numbering | Status |
|---|---|---:|---:|---|
| Punarcci definition | `மெய்யுயிர் முதலீறாம் இரு பதங்களும்... இயைவது புணர்ப்பே` | **150** (printed p. 61) | **151** | confirmed mismatch |
| Action-frame / vinai | `செய்பவன் கருவி நிலம் செயல் காலம் / செய்பொருள் ஆறும் தருவது வினையே` | **319** (printed p. 161) | **320** | confirmed mismatch |
| Idai general rule | `வேற்றுமை வினைசாரியை... ஒன்றுவது இடைச்சொல்` | pending direct Mayilai alignment | **420** in searchable TVA presentation | pending |
| Uri general rule | `பல்வகைப் பண்பும் பகர்பெயராகி... உரியன உரிச்சொல்` | pending direct Mayilai alignment | **442** in common TVA presentation | pending |

---

# Source identities

## MAYILAI-UVE

- work: Nannūl
- commentary: Mayilaināthar
- TVA legacy source: `https://www.tamilvu.org/library/l0901/html/l0901ind.htm`
- TVA title: `நன்னூல் மூலமும் மயிலைநாதருரையும்`
- edition label displayed by TVA: U. Vē. Cāminātaiyar
- first edition bibliographic record in TVA: Chennai, Vaijayanti Press, 1918

## SANKARA-SIVAJNANA-VIRUTTI

- work: Nannūl
- commentary: Śaṅkara Namaccivāyar
- revision: Sivajñāna Munivar
- TVA legacy source: `https://www.tamilvu.org/library/l0902/html/l0902ind.htm`
- TVA title identifies the text as the Virutti commentary made by Śaṅkara Namaccivāyar and revised by Sivajñāna Munivar
- TVA bibliography records an early Āṟumuka Nāvalar publication of this revised Virutti tradition at Yāḻppāṇam, 1851

## TVA-COMMON

Use this only for TVA searchable/teaching presentations where edition-specific identity is not established in the research record.

It is a **navigation/concordance aid**, not a substitute for a named commentary edition.

---

# Required fields for future rows

```yaml
work: Nannul
incipit: "..."
concept: "..."
source_key: "..."
commentator: "..."
editor: "..."
local_number: "..."
printed_page: "..."
tva_url: "..."
variant_numbers:
  - source: "..."
    number: "..."
notes: "..."
verification: confirmed | provisional | pending
```

---

# Immediate concordance tasks

1. Align the Mayilaināthar and Virutti-commentary positions for the `idai` general rule.
2. Align the `uri` general rule.
3. Align selected பொதுவியல் rules relevant to ambiguity, admissible variation, and cross-category constraints.
4. Test whether the 150/151 and 319/320 offsets are caused by the same earlier counting difference throughout the intervening range.
5. Never infer later rule numbers by simply adding one; verify by incipit.

---

# Aytham provenance consequence

Any future specification note inspired by a Tamil grammatical passage should be able to link back to a stable source record even when edition numbering changes.

Conceptually:

```text
Aytham design decision
        ↓
research interpretation
        ↓
commentary observation
        ↓
source incipit
        ↓
number/page in each edition
```

This is part of Aytham's broader commitment to explicit provenance.
