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

type 'term t =
  | Terminal of 'term

  (* Combinators *)
  | Not of 'term t
  | Union of 'term t list
  | Intersection of 'term t list
  | Modulate of 'term t * float
  | Translate of 'term t * vec
  | Scale of 'term t * vec
[@@deriving sexp, map]

let rec fold_shape shape init f =
  let next = f init shape in
  match shape with
  | Not target -> fold_shape target next f
  | Union c | Intersection c -> List.fold c ~init:next ~f:(fun i n -> fold_shape n i f)
  | Modulate (target, _) -> fold_shape target next f
  | Translate (target, _) -> fold_shape target next f
  | Scale (target, _) -> fold_shape target next f
  | _ -> next

let _contains f shape =
  fold_shape shape false (fun cur shape -> cur || f shape)

let circle ~x ~y ~r = Circle { x; y; r; mat = Matrix.id }
let rect ~x ~y ~w ~h = Rect { x; y; w; h; mat = Matrix.id }
let poly points = Poly { points; mat = Matrix.id }
let not a = Not a
let union children = Union children
let intersection children = Intersection children
