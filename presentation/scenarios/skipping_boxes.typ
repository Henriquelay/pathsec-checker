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
  myswitch([e1], [0X61E8D6E7], [0X61E8D6E7]),
  myswitch([s1], [0XAE91434C], [0XAE91434C]),
  myswitch([s2], [0X08C97F5F], [0X08C97F5F]),
  myswitch([s3], [0XEFF1AAD2], [0XEFF1AAD2]),

  myswitch([s4], [0X08040C89], [0X08040C89]),
  myswitch([s5], [skipped], [0XAA99AE2E]),
  myswitch([s6], [0XB0437A53], [0X7669685E]),
  myswitch([s7], [0X63589D0A], [0X03E1E388]),

  myswitch([s8], [0X629B7B3B], [0X2138FFD3]),
  myswitch([s9], [0XBD53E851], [0X1EF2CBBE]),
  myswitch([s10], [0X90BDF731], [0X99C5FE05]),
)
#pagebreak()
#table(
  columns: 4,
  stroke: none,
  inset: 3pt,
  myswitch([e10], [0XABADCAFE], [0XABADCAFE]),
  myswitch([s10], [0X2247084B], [0X2247084B]),
  myswitch([s9], [0X81C417A7], [0X81C417A7]),
  myswitch([s8], [0XF4E82C1E], [0XF4E82C1E]),

  myswitch([s7], [0X31881788], [0X31881788]),
  myswitch([s6], [0X7C79E406], [0X7C79E406]),
  myswitch([s5], [skipped], [0X8E4AB525]),
  myswitch([s4], [0XD0944C2B], [0XDB11D4A9]),

  myswitch([s3], [0XBC7166C4], [0X36A0DAB0]),
  myswitch([s2], [0X27ACDB88], [0X361B8641]),
  myswitch([s1], [0X96D7328F], [0X53CEB3ED]),
)
