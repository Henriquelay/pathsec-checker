#import "@preview/fletcher:0.5.0" as fletcher: diagram, node, edge, shapes
#set page(width: auto, height: auto, margin: (x: 0pt, y: 0pt))

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
    // node((0 / 2, 1), "h11", stroke: 0.5pt, name: "h11", shape: shapes.rect)
    // edge(auto, <e1>, "<->")
    node((5.5 / 2, -0.6), "s555", stroke: 0.5pt, name: "s555", shape: shapes.octagon)
    edge(<s555>, <s6>, "<->")
    edge(<s5>, <s555>, "<->")
  })
