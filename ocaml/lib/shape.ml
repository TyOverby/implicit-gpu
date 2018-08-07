open Core

type polygon = Point.t list [@@deriving sexp]

type shape =
  (* Terminals *)
  | Circle of { x: float; y: float; r: float }
  | Rect of { x: float; y: float; w: float; h: float }
  | Poly of polygon
  | Nothing
  | Everything

  (* Combinators *)
  | Not of shape
  | Union of shape list
  | Intersection of shape list
  | Modulate of shape * float
  | Translate of shape * vec
  | Scale of shape * vec
[@@deriving sexp]

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

let circle ~x ~y ~r = Circle { x; y; r }
let rect ~x ~y ~w ~h = Rect { x; y; w; h }
let poly points = Poly points
let not a = Not a
let union children = Union children
let intersection children = Intersection children

let shape_eq (a: shape) (b: shape) = a = b
