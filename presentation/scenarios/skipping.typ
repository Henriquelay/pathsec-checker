#import "@preview/fletcher:0.5.0" as fletcher: diagram, node, edge, shapes
#set page(width: auto, height: auto, margin: (x: 0pt, y: 0pt))
#set text(font: "New Computer Modern")

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
    if i != 5 and i != 4 {
      edge(label("s" + str(i)), label("s" + str(i + 1)), "<->")
    }
  }
  edge((4 / 2, 0), ((6) / 2, 0), "<->", bend: 35deg)
  // node((0 / 2, 1), "h11", stroke: 0.5pt, name: "h11", shape: "rect")
  // edge(auto, <e1>, "<->")
})
