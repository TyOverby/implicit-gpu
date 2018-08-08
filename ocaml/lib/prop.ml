open Core
open Shape
open Matrix

let rec propagate shape incoming = match shape with
  | Terminal Nothing -> Terminal Nothing
  | Terminal Everything -> Terminal Everything
  | Terminal Circle { x; y; r; mat} -> Terminal (Circle { x; y; r; mat=(mul incoming mat)})
  | Terminal Rect { x; y; w; h; mat} -> Terminal (Rect { x; y; w; h; mat=(mul incoming mat)})
  | Terminal Poly { points; mat} -> Terminal (Poly { points; mat=(mul incoming mat)})
  | Not target -> Not (propagate target incoming)
  | Union targets -> Union (propagate_all targets incoming)
  | Intersection targets -> Intersection (propagate_all targets incoming)
  | Modulate (target, how_much) -> Modulate (propagate target incoming, how_much)
  | Translate (target, {dx; dy}) -> propagate target (mul (Matrix.create_translation dx dy) incoming)
  | Scale (target, {dx; dy}) -> propagate target (mul (Matrix.create_scale dx dy) incoming)
and propagate_all shapes incoming =
  List.map shapes ~f:(fun s -> propagate s incoming)

let remove_transformations shape = propagate shape Matrix.id
