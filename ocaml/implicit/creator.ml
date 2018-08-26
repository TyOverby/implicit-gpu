type t = Stages.user

let intersection (lst: t list) : t = Shape.Intersection lst
let union (lst: t list) : t = Shape.Union lst
let not (target: t ) : t = Shape.Not target
let subtract (a: t) (b: t): t = intersection [a; (not b)]
let freeze (target: t ) : t = Shape.Freeze target
let modulate (v: float) (target: t) : t = Shape.Modulate(target, v)

let circle ~x ~y ~r :t = Shape.Terminal (Shape.Circle { x; y; r; })
let rect ~x ~y ~w ~h :t = Shape.Terminal( Shape.Rect { x; y; w; h; })
let poly points: t = Shape.Terminal( Shape.Poly { points; })

let scale ~dx ~dy target :t = Shape.Transform(target, Matrix.create_scale dx dy)
let translate ~dx ~dy target :t = Shape.Transform (target, Matrix.create_translation dx dy)
