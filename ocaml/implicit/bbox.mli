type t = { x: float; y: float; w: float; h: float} [@@deriving sexp]

type bounding =
  | Everything
  | Nothing
  | Positive of t
  | Negative of t
[@@deriving sexp]

val union: bounding -> bounding -> bounding
val intersection: bounding -> bounding -> bounding
val inverse: bounding -> bounding

val grow_by: float -> t -> t

val bbox_of_points: Point.t list -> t option
val grow: bounding -> float -> bounding
