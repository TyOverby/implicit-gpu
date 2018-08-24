module Shape = struct
  type t = Stages.user

  let intersection (lst: t list) : t = Shape.Intersection lst
  let union (lst: t list) : t = Shape.Union lst
  let not (target: t ) : t = Shape.Not target
  let modulate (target: t) (v: float) : t = Shape.Modulate(target, v)

  let circle ~x ~y ~r :t = Shape.Terminal (Shape.Circle { x; y; r; mat = Matrix.id })
  let rect ~x ~y ~w ~h :t = Shape.Terminal( Shape.Rect { x; y; w; h; mat = Matrix.id })
  let poly points: t = Shape.Terminal( Shape.Poly { points; mat = Matrix.id })
end
module Point = Point
module Command = Command
module Bbox = Bbox

module Ops = struct
  let ( && ) a b = Shape.intersection [ a; b; ]
  let ( || ) a b = Shape.union [ a; b; ]
  let ( -- ) a b = Shape.intersection [ a; Shape.not b; ]
  let ( ++ ) a v = Shape.modulate a v
  let ( !! ) t = Shape.not t
end

let compile = Command.compile
