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
  myswitch([e1], [0xBADDC0DE], [0xBADDC0DE]),
  myswitch([s1], [0x3EF96770], [0x3EF96770]),
  myswitch([s2], [0x2DCA9942], [0x2DCA9942]),
  myswitch([s3], [0x11797334], [0x11797334]),

  myswitch([s4], [0x98081E3E], [0x98081E3E]),
  myswitch([s5], [0x3332E012], [0x3332E012]),
  myswitch([s555], [0x90A0DF94], [0x22996AFD]),
  myswitch([s7], [0xBEBE4372], [0x8FA3987D]),
  myswitch([s8], [0x5AAFA7F2], [0xF4B50950]),

  myswitch([s9], [0x649B8554], [0xD0C29E67]),
  myswitch([s10], [0xF46427BF], [0x13FF41C1]),
)
#pagebreak()
#table(
  columns: 4,
  stroke: none,
  inset: 3pt,
  myswitch([e10], [0xDEADBEEF], [0xDEADBEEF]),
  myswitch([s10], [0x5F45C4E5], [0x5F45C4E5]),
  myswitch([s9], [0x4D34AD25], [0x4D34AD25]),
  myswitch([s8], [0x602BAA4E], [0x602BAA4E]),

  myswitch([s7], [0x96F1275B], [0x96F1275B]),
  myswitch([s555], [0xF247A607], [0x377923F8]),
  myswitch([s5], [0x1871A1A6], [0x1CE1F48B]),
  myswitch([s4], [0x311B656F], [0xC179BFAC]),

  myswitch([s3], [0x0D2C0646], [0xB9A3B130]),
  myswitch([s2], [0x804DC63F], [0xEAD6AF39]),
  myswitch([s1], [0x4422E397], [0xBCA3D63A]),
)
