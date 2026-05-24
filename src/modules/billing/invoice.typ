#import sys: inputs

#let invoice = inputs.invoice
#let accent = rgb("#334155")
#let muted = rgb("#64748b")

#let render-lines(lines) = {
  for line in lines [
    #line \
  ]
}

#let value-row(label, value) = {
  if value != "" [
    *#label:* #value \
  ]
}

#set page(
  paper: "a4",
  margin: (top: 20mm, bottom: 18mm, x: 18mm),
)
#set text(size: 10pt)
#set par(leading: 0.75em)
#show heading.where(level: 1): set text(fill: accent)
#show heading.where(level: 2): set text(fill: accent)
#show table.cell.where(y: 0): set text(weight: "bold")

#align(right)[
  #text(size: 8.5pt, fill: muted)[#invoice.sender_compact]
]

#v(10pt)

#table(
  columns: (2fr, 1fr),
  gutter: 18pt,
  stroke: none,
  [
    = Rechnung
    #text(size: 10.5pt, fill: muted)[#invoice.project_name]
  ],
  align(right)[
    *Rechnungsnummer* \
    #invoice.invoice_number \
    #v(6pt)
    *Rechnungsdatum* \
    #invoice.issued_at
    #if invoice.due_on != "" [
      #v(6pt)
      *Faellig am* \
      #invoice.due_on
    ]
  ],
)

#v(14pt)

#table(
  columns: (1fr, 1fr),
  gutter: 18pt,
  stroke: none,
  [
    == Rechnung von
    #invoice.sender_name \
    #render-lines(invoice.sender_address_lines)
  ],
  [
    == Rechnung an
    #invoice.customer_name \
    Projekt: #invoice.project_name \
    #if invoice.project_location != "" [
      Ort: #invoice.project_location
    ]
  ],
)

#v(10pt)

#table(
  columns: (1fr, 1fr),
  gutter: 18pt,
  stroke: none,
  [
    #value-row("Abrechnungsreferenz", invoice.billing_reference)
    #value-row("Angebot", invoice.quote_reference)
  ],
  [
    *Arbeitszeit gesamt:* #invoice.labor_total_hours \
    *Materialbewegungen:* #invoice.material_withdrawal_count
    #if invoice.budget_amount != "" [
      \
      *Budgetrahmen:* #invoice.budget_amount
    ]
  ],
)

#v(16pt)

#table(
  columns: (3.2fr, 0.9fr, 1.1fr, 1.4fr, 1.4fr, 1.4fr),
  align: (left, right, right, right, right, right),
  inset: 6pt,
  table.header(
    [Leistung],
    [Menge],
    [Einheit],
    [Erfasst],
    [Satz],
    [Summe],
  ),
  ..invoice.line_items.map(item => (
    [#item.description],
    [#item.quantity],
    [#item.unit],
    [#item.source_count],
    [#item.unit_price],
    [#item.line_total],
  )).flatten(),
)

#if invoice.total_amount != "" [
  #v(10pt)
  #align(right)[
    *Gesamtbetrag:* #invoice.total_amount
  ]
]

#if invoice.billing_notes != "" [
  #v(16pt)
  == Notizen
  #invoice.billing_notes
]

#v(18pt)

#text(size: 8.5pt, fill: muted)[
  Diese Rechnung wurde digital aus dem Projektbericht erzeugt.
]
