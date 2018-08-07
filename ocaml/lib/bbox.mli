type bbox = { x: float; y: float; w: float; h: float} [@@deriving sexp]

type bounding =
  | Everything
  | Nothing
  | Positive of bbox
  | Negative of bbox
[@@deriving sexp]

val union: bounding -> bounding -> bounding
val intersection: bounding -> bounding -> bounding
val inverse: bounding -> bounding

val bbox_of_poly: Point.t list -> bbox option
val grow: bounding -> float -> bounding
