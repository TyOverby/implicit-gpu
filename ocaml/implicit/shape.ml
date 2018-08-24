open Core

type vec = { dx: float; dy: float} [@@deriving sexp]
type polygon = {
  points: Point.t list;
  mat: Matrix.t [@default Matrix.id] [@sexp_drop_default];
} [@@deriving sexp]

type circle = {
  x: float;
  y: float;
  r: float;
  mat: Matrix.t [@default Matrix.id] [@sexp_drop_default];
} [@@deriving sexp]

type rect ={
  x: float;
  y: float;
  w: float;
  h: float;
  mat: Matrix.t [@default Matrix.id] [@sexp_drop_default];
} [@@deriving sexp]

type poly = {
  points: Point.t list;
  mat: Matrix.t [@default Matrix.id] [@sexp_drop_default];
} [@@deriving sexp]

type allTerminals =
  | Circle of circle
  | Rect of rect
  | Poly of poly
  | Nothing
  | Everything
[@@deriving sexp]

type justConcreteTerminals =
  | Circle of circle
  | Rect of rect
  | Poly of poly
[@@deriving sexp]

type ('term, 'trans) t =
  (* terminals *)
  | Terminal of 'term

  (* transformations *)
  | Transform of 'trans

  (* combinators *)
  | Not of ('term, 'trans) t
  | Union of ('term, 'trans) t list
  | Intersection of ('term, 'trans) t list
  | Modulate of ('term, 'trans) t * float
  | Freeze of ('term, 'trans) t
[@@deriving sexp, map]

type 'a allTransforms =
  | Translate of ('a, 'a allTransforms) t * vec
  | Scale of ('a, 'a allTransforms) t * vec
[@@deriving sexp, map]

type 'a allTShape = ('a, 'a allTransforms) t
[@@deriving sexp]

let rec visit (f: ('term_b, 'trans_b) t -> ('term_b, 'trans_b) t) (g: 'term_a -> 'term_b) (h: 'trans_a -> 'trans_b)  = function
  | Terminal t -> Terminal (g t)
  | Transform t -> Transform(h t)
  | Not target -> f(Not ((visit f g h) target))
  | Union targets -> f(Union (List.map ~f:(visit f g h ) targets))
  | Intersection targets -> f(Intersection (List.map ~f:(visit f g h) targets))
  | Modulate(target, v) -> f(Modulate((visit f g h) target, v))
  | Freeze(target) -> f(Freeze((visit f g h) target))
