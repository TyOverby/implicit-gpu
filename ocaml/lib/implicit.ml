module Shape = struct
  include Shape
end
module Point = struct
  include Point
end
module Bbox = struct
  include Bbox
end
module Operations = struct
  let compile shape =
    let simplified = Simplify.simplify_top shape in
    match simplified with
    | Simplify.SNothing
    | Simplify.SEverything -> None
    | Simplify.SShape shape ->
      let propagated = Prop.remove_transformations shape in
      let bb = Compute_bb.compute_bounding_box propagated in
      let compiled = Command.compile propagated in
      Some (compiled, bb)
end

module Ops = struct
  let ( && ) a b = Shape.intersection [ a; b; ]
  let ( || ) a b = Shape.union [ a; b; ]
  let ( -- ) a b = Shape.intersection [ a; Shape.not b; ]
  let ( ++ ) a v = Shape.Modulate (a, v)
  let ( !! ) t = Shape.not t
end
