#set page(width: auto, height: auto, margin: (x: 0pt, y: 0pt))
#set text(font: "DejaVu Sans Mono")

#let myswitch(name, digest, expected) = {
  box[
    #table(
      columns: 2,
      fill: if (digest == expected) {
        lime
      } else {
        red
      },
      align: (right, left),
      [name], [#name],
      [digest], [#digest],
      [expected], [#expected],
    )
  ]
}
#table(
  columns: 4,
  stroke: none,
  inset: 3pt,
  myswitch([e1], [0x61E8D6E7], [0x61E8D6E7]),
  myswitch([s1], [0xAE91434C], [0xAE91434C]),
  myswitch([s2], [0x08C97F5F], [0x08C97F5F]),
  myswitch([s3], [0xEFF1AAD2], [0xEFF1AAD2]),

  myswitch([s4], [0x08040C89], [0x08040C89]),
  myswitch([s5], [0xAA99AE2E], [0xAA99AE2E]),
  myswitch([s6], [0x7669685E], [0x7669685E]),
  myswitch([s7], [0x03E1E388], [0x03E1E388]),

  myswitch([s8], [0x2138FFD3], [0x2138FFD3]),
  myswitch([s9], [0x1EF2CBBE], [0x1EF2CBBE]),
  myswitch([s10], [0x99C5FE05], [0x99C5FE05]),
)
#pagebreak()
#table(
  columns: 4,
  stroke: none,
  inset: 3pt,
  myswitch([e10], [0x61E8D6E7], [0x61E8D6E7]),
  myswitch([s10], [0xCFFABC9F], [0xCFFABC9F]),
  myswitch([s9], [0x69409E70], [0x69409E70]),
  myswitch([s8], [0xF3E992E0], [0xF3E992E0]),

  myswitch([s7], [0x8DDE192B], [0x8DDE192B]),
  myswitch([s6], [0x92B098FA], [0x92B098FA]),
  myswitch([s5], [0x1115A62C], [0x1115A62C]),
  myswitch([s4], [0x41E1B5E0], [0x41E1B5E0]),

  myswitch([s3], [0x227F0B72], [0x227F0B72]),
  myswitch([s2], [0x82FC6346], [0x82FC6346]),
  myswitch([s1], [0xD01E3E0F], [0xD01E3E0F]),
)
