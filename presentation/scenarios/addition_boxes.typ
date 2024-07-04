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
  myswitch([e1], [0XABADCAFE], [0XABADCAFE]),
  myswitch([s1], [0X432CF798], [0X432CF798]),
  myswitch([s2], [0XE04DF688], [0XE04DF688]),
  myswitch([s3], [0XE8F0142C], [0XE8F0142C]),

  myswitch([s4], [0XB452022A], [0XB452022A]),
  myswitch([s5], [0X4450D2D2], [0X4450D2D2]),
  myswitch([s555], [0X5B0FCE3E], [0XE9367B57]),
  myswitch([s6], [0X5B0FCE3E], [0XE9367B57]),

  myswitch([s7], [0X03E1E388], [0X991182C1]),
  myswitch([s8], [0X2138FFD3], [0X35E72E11]),
  myswitch([s9], [0X1EF2CBBE], [0XAA152EB9]),
  myswitch([s10], [0X99C5FE05], [0X1A1573E7]),
)
#pagebreak()
#table(
  columns: 4,
  stroke: none,
  inset: 3pt,
  myswitch([e10], [0XBADDC0DE], [0XBADDC0DE]),
  myswitch([s10], [0X5F9298A3], [0X5F9298A3]),
  myswitch([s9], [0X4C43786D], [0X4C43786D]),
  myswitch([s8], [0X0D614B06], [0X0D614B06]),

  myswitch([s7], [0X1DD20B9C], [0X1DD20B9C]),
  myswitch([s6], [0X0B1BD6C6], [0X0B1BD6C6]),
  myswitch([s5], [0X45E5A48F], [0X45E5A48F]),
  myswitch([s4], [0XCDA3CE15], [0XCDA3CE15]),

  myswitch([s3], [0XF7F2FDF1], [0XF7F2FDF1]),
  myswitch([s2], [0X4CCC369F], [0X4CCC369F]),
  myswitch([s1], [0X5A2481CB], [0X5A2481CB]),
)
