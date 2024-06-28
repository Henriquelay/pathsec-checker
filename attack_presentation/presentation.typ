#import "@preview/polylux:0.3.1": *
#import "@preview/fletcher:0.5.0" as fletcher: diagram, node, edge, shapes

#import themes.simple: *

#set text(font: "New Computer Modern")

// #enable-handout-mode(false)

#show: simple-theme//.with(footer: [Simple slides])

#title-slide[
  = Polka attack simulations
  #v(2em)

  Henrique Coutinho Layber #footnote[henrique.layber\@edu.ufes.br] #h(1em)

  June 27 2024
]

#slide[
  == Baseline measurements

  Before we start attacking the network and comparing digests, we need to establish a baseline for correct operation.

  #pause

  For that, we will ping host `h10` from host `h1` and measure the packet through all controlled interfaces. Always using the hardcoded timestamp `0x61E8D6E7` unless said otherwise.
]

#slide[
  In a linear topology passing though 10 core switches, this generates 88 packets:
  #pause
  - one digest for entrance edge + 10 core switches (11)
  #pause
  - two captures per switch (one for each interface) (22)
  #pause
  - two time that for the reply (44)
  #pause
  - two times that for the ping and reply the other way (88)
]

#slide[
  So in a trace, we expect the packets to appear in the following order:
  #columns(2)[
    #columns(2)[
      #set text(size: 19pt)
      #box[
        A = #stack([
        `0x61E8D6E7 e1`  \
        `0xAE91434C s1` \
        `0x08C97F5F s2` \
        `0xEFF1AAD2 s3` \
        `0x08040C89 s4` \
        `0xAA99AE2E s5` \
        `0x7669685E s6` \
        `0x03E1E388 s7` \
        `0x2138FFD3 s8` \
        `0x1EF2CBBE s9` \
        `0x99C5FE05 s10`])
      ]
      #colbreak()
      #box[
        B = #stack([
        `0x61E8D6E7 e10` \
        `0xCFFABC9F s10` \
        `0x69409E70 s9` \
        `0xF3E992E0 s8` \
        `0x8DDE192B s7` \
        `0x92B098FA s6` \
        `0x1115A62C s5` \
        `0x41E1B5E0 s4` \
        `0x227F0B72 s3` \
        `0x82FC6346 s2` \
        `0xD01E3E0F s1`])
      ]
    ]
    #colbreak()
    #set text(size: 20pt)
    #box()[
      $ "Order" = "dup"(A), "dup"(B), "dup"(B), "dup"(A) $
      / $"dup"([a, b, c])=$: $[a, a, b, b, c, c]$

      #pause
      For brevity, $"dup"$ will be ommited since it's information is irrelevant, but it will be checked regardless
    ]
  ]
]

#let myswitch(name, digest, expected) = {
  box[
    #set text(size: 12pt)
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
#slide[
  == Ping `h1` #sym.arrow.r `h10`
  #myswitch([e1], [0x61E8D6E7], [0x61E8D6E7])
  #myswitch([s1], [0xAE91434C], [0xAE91434C])
  #myswitch([s2], [0x08C97F5F], [0x08C97F5F])
  #myswitch([s3], [0xEFF1AAD2], [0xEFF1AAD2])
  #myswitch([s4], [0x08040C89], [0x08040C89])
  #myswitch([s5], [0xAA99AE2E], [0xAA99AE2E])
  #myswitch([s6], [0x7669685E], [0x7669685E])
  #myswitch([s7], [0x03E1E388], [0x03E1E388])
  #myswitch([s8], [0x2138FFD3], [0x2138FFD3])
  #myswitch([s9], [0x1EF2CBBE], [0x1EF2CBBE])
  #myswitch([s10], [0x99C5FE05], [0x99C5FE05])
]
#slide[
  == Ping `h1` #sym.arrow.r `h10` Reply
  #myswitch([e10], [0x61E8D6E7], [0x61E8D6E7])
  #myswitch([s10], [0xCFFABC9F], [0xCFFABC9F])
  #myswitch([s9], [0x69409E70], [0x69409E70])
  #myswitch([s8], [0xF3E992E0], [0xF3E992E0])
  #myswitch([s7], [0x8DDE192B], [0x8DDE192B])
  #myswitch([s6], [0x92B098FA], [0x92B098FA])
  #myswitch([s5], [0x1115A62C], [0x1115A62C])
  #myswitch([s4], [0x41E1B5E0], [0x41E1B5E0])
  #myswitch([s3], [0x227F0B72], [0x227F0B72])
  #myswitch([s2], [0x82FC6346], [0x82FC6346])
  #myswitch([s1], [0xD01E3E0F], [0xD01E3E0F])
]

#slide[
  == Base topology
  #diagram({
    for i in range(1, 11) {
      let h = "h" + str(i)
      node((i / 2, 1), h, stroke: 0.5pt, name: label(h), shape: "rect")
      edge("<->")
      let e = "e" + str(i)
      node((i / 2, 0.5), e, stroke: 0.5pt, name: label(e), shape: shapes.pill)
      edge("<->")
      let s = "s" + str(i)
      node((i / 2, 0), s, stroke: 0.5pt, name: label(s), shape: shapes.octagon)
    }
    for i in range(1, 10) {
      edge((i / 2, 0), ((i + 1) / 2, 0), "<->")
    }
    node((0 / 2, 1), "h11", stroke: 0.5pt, name: "h11", shape: "rect")
    edge(auto, <e1>, "<->")
  })

  $s{n} <-> s{n+1}$ connects through port 2:1 \
]

#slide[
  == Attacked topology
  #diagram({
    for i in range(1, 11) {
      let h = "h" + str(i)
      node((i / 2, 1), h, stroke: 0.5pt, name: label(h), shape: shapes.rect)
      edge("<->")
      let e = "e" + str(i)
      node((i / 2, 0.5), e, stroke: 0.5pt, name: label(e), shape: shapes.pill)
      edge("<->")
      let s = "s" + str(i)
      node((i / 2, 0), s, stroke: 0.5pt, name: label(s), shape: shapes.octagon)
    }
    for i in range(1, 10) {
      edge((i / 2, 0), ((i + 1) / 2, 0), "<->")
    }
    node((0 / 2, 1), "h11", stroke: 0.5pt, name: "h11", shape: shapes.rect)
    edge(auto, <e1>, "<->")
    node((5.5 / 2, -0.5), "s555", stroke: 0.5pt, name: "s555", shape: shapes.octagon)
    edge(<s555>, <s6>, "<->")
    edge(<s5>, <s555>, "<->")
  })

  $"s5" <-> "s555"$ connects through port 2:0 \
  $"s555" <-> "s6"$ connects through port 1:2 \
  $"s5" <-> "s6"$ connects through port 3:3 \
]

#slide[


  == Ping `h1` #sym.arrow.r `h10`
  #myswitch([e1], [0x61E8D6E7], [0x61E8D6E7])
  #myswitch([s1], [0xAE91434C], [0xAE91434C])
  #myswitch([s2], [0x08C97F5F], [0x08C97F5F])
  #myswitch([s3], [0xEFF1AAD2], [0xEFF1AAD2])
  #myswitch([s4], [0x08040C89], [0x08040C89])
  #myswitch([s5], [0xAA99AE2E], [0xAA99AE2E])
  #myswitch([attacer], [0xC450DD37], [0x7669685E])
  #myswitch([s6], [0x5397C754], [0x7669685E])
  #myswitch([s7], [0xE21DAB66], [0x03E1E388])
  #myswitch([s8], [0x8C375948], [0x2138FFD3])
  #myswitch([s9], [0x352F1CF8], [0x1EF2CBBE])
  #myswitch([s10], [0x3a61c724], [0x99C5FE05])
]
/*
0x61E8D6E7
0xCFFABC9F
0x69409E70
0xF3E992E0
0x8DDE192B
0x92B098FA
0x1115A62C
0x41E1B5E0
0x227F0B72
0x82FC6346
0xD01E3E0F
  */
#slide[
  == Ping `h1` #sym.arrow.r `h10` Reply
  #myswitch([e10], [0x61E8D6E7], [0x61E8D6E7])
  #myswitch([s10], [0xCFFABC9F], [0xCFFABC9F])
  #myswitch([s9], [0x69409E70], [0x69409E70])
  #myswitch([s8], [0xF3E992E0], [0xF3E992E0])
  #myswitch([s7], [0x8DDE192B], [0x8DDE192B])
  #myswitch([s6], [0x92B098FA], [0x92B098FA])
  #myswitch([s5], [0x1115A62C], [0x1115A62C])
  #myswitch([s4], [0x41E1B5E0], [0x41E1B5E0])
  #myswitch([s3], [0x227F0B72], [0x227F0B72])
  #myswitch([s2], [0x82FC6346], [0x82FC6346])
  #myswitch([s1], [0xD01E3E0F], [0xD01E3E0F])
]
#slide[
  == Ping `h10` #sym.arrow.r `h1`
  #myswitch([e10], [0x61E8D6E7], [0x61E8D6E7])
  #myswitch([s10], [0xCFFABC9F], [0xCFFABC9F])
  #myswitch([s9], [0x69409E70], [0x69409E70])
  #myswitch([s8], [0xF3E992E0], [0xF3E992E0])
  #myswitch([s7], [0x8DDE192B], [0x8DDE192B])
  #myswitch([s6], [0x92B098FA], [0x92B098FA])
  #myswitch([s5], [0x1115A62C], [0x1115A62C])
  #myswitch([s4], [0x41E1B5E0], [0x41E1B5E0])
  #myswitch([s3], [0x227F0B72], [0x227F0B72])
  #myswitch([s2], [0x82FC6346], [0x82FC6346])
  #myswitch([s1], [0xD01E3E0F], [0xD01E3E0F])
]
/*
0x61E8D6E7
0xAE91434C
0x08C97F5F
0xEFF1AAD2
0x08040C89
0xAA99AE2E
0xC450DD37
0x5397C754
0xE21DAB66
0x8C375948
0x352F1CF8
0x3A61C724
*/
#slide[
  == Ping `h10` #sym.arrow.r `h1` Reply
  #myswitch([e10], [0x61E8D6E7], [0x61E8D6E7])
  #myswitch([s10], [0xAE91434C], [0xAE91434C])
  #myswitch([s9], [0x08C97F5F], [0x08C97F5F])
  #myswitch([s8], [0xEFF1AAD2], [0xEFF1AAD2])
  #myswitch([s7], [0x08040C89], [0x08040C89])
  #myswitch([s6], [0xAA99AE2E], [0xAA99AE2E])
  #myswitch([attacker], [0xC450DD37], [0x7669685E])
  #myswitch([s5], [0x5397C754], [0x7669685E])
  #myswitch([s4], [0xE21DAB66], [0x03E1E388])
  #myswitch([s3], [0x8C375948], [0x2138FFD3])
  #myswitch([s2], [0x352F1CF8], [0x1EF2CBBE])
  #myswitch([s1], [0x3A61C724], [0x99C5FE05])
]
