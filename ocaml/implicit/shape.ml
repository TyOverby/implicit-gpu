open Core

type vec = { dx: float; dy: float} [@@deriving sexp]

type circle = {
  x: float;
  y: float;
  r: float;
} [@@deriving sexp]

type rect ={
  x: float;
  y: float;
  w: float;
  h: float;
} [@@deriving sexp]

type poly = {
  points: Point.t list;
} [@@deriving sexp]

type simplex = {
  cutoff: float;
} [@@deriving sexp]

type allTerminals =
  | Circle of circle
  | Rect of rect
  | Poly of poly
  | Simplex of simplex
  | Nothing
  | Everything
[@@deriving sexp]

type justConcreteTerminals =
  | Circle of circle
  | Rect of rect
  | Poly of poly
  | Simplex of simplex
[@@deriving sexp]

type 'term t =
  (* terminals *)
  | Terminal of 'term

  (* transformations *)
  | Transform of 'term t * Matrix.t

  (* combinators *)
  | Not of 'term t
  | Union of 'term t list
  | Intersection of 'term t list

  (* modifiers *)
  | Modulate of 'term t * float
  | Freeze of 'term t
  | Drag of 'term t * float * float
[@@deriving sexp, map]

let rec visit (f: 'term_b t -> 'term_b t) (g: 'term_a -> 'term_b)  = function
  | Terminal t -> Terminal (g t)
  | Transform (t, m) -> f(Transform((visit f g) t, m))
  | Not target -> f(Not ((visit f g ) target))
  | Union targets -> f(Union (List.map ~f:(visit f g) targets))
  | Intersection targets -> f(Intersection (List.map ~f:(visit f g) targets))
  | Modulate(target, v) -> f(Modulate((visit f g ) target, v))
  | Freeze (target) -> f(Freeze((visit f g ) target))
  | Drag (target, dx, dy) -> f(Drag((visit f g) target, dx, dy))
