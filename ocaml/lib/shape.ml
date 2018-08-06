open Core
open Math

type point = Math.point
type vec = Math.vec
type polygon = point list [@@deriving sexp]

let point_of_sexp = Math.point_of_sexp
let sexp_of_point = Math.sexp_of_point
let vec_of_sexp = Math.vec_of_sexp
let sexp_of_vec = Math.sexp_of_vec

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

let circle ~x ~y ~r = Circle { x; y; r }
let rect ~x ~y ~w ~h = Rect { x; y; w; h }
let poly points = Poly points
let not a = Not a
let union children = Union children
let intersection children = Intersection children

let shape_eq (a: shape) (b: shape) = a = b

let rec simplify = function
  (* circle *)
  | Circle { r; _ } when r <= 0.0  -> Nothing
  | Circle _ as a -> a

  (* rect *)
  | Rect { w; h; _ } when w <= 0.0 || h <= 0.0 -> Nothing
  | Rect _ as a -> a

  (* poly *)
  | Poly [] -> Nothing
  | Poly _ as a -> a

  (* everything and nothing *)
  | Everything -> Everything
  | Nothing -> Nothing

  (* not *)
  | Not Not x -> simplify x
  | Not inner -> (
      match simplify inner with
      | Nothing -> Everything
      | Everything -> Nothing
      | rest -> Not rest
    )

  (* modulate *)
  | Modulate (target, how_much)  -> (
      match simplify target with
      | Nothing -> Nothing
      | Everything -> Everything
      | target -> Modulate (target, how_much)
    )

  (* scale *)
  | Scale(target, vec)  -> (
      match simplify target with
      | Nothing -> Nothing
      | Everything -> Everything
      | target -> Scale(target, vec)
    )

  (* translate *)
  | Translate(target, vec)  -> (
      match simplify target with
      | Nothing -> Nothing
      | Everything -> Everything
      | target -> Translate(target, vec)
    )

  (* union *)
  | Union list -> let list = simplify_all list in
    if List.exists list ~f:(phys_equal Everything)
    then Everything
    else simplify_easy_lists (Union (remove list Nothing))

  (* intersection *)
  | Intersection list -> let list = simplify_all list in
    if List.exists list ~f:(phys_equal Nothing)
    then Nothing
    else simplify_easy_lists (Intersection (remove list Everything))

and simplify_all = List.map ~f:simplify
and simplify_easy_lists = function
  | Intersection [] -> Nothing
  | Intersection [a] -> a
  | Union [] -> Nothing
  | Union [a] -> a
  | other -> other
and remove list target =
  let filter a = phys_equal a target |> Core.not in
  List.filter ~f:filter list

let simplify_test a =
  a
  |> Sexp.of_string
  |> shape_of_sexp
  |> simplify
  |> sexp_of_shape
  |> Sexp.to_string_hum
  |> print_endline

let%expect_test _ =
  simplify_test "Nothing";
  [%expect "Nothing"]

let%expect_test _ =
  simplify_test "Everything";
  [%expect "Everything"]

let%expect_test _ =
  simplify_test "(Circle (x 1) (y 1) (r 1))";
  [%expect "(Circle (x 1) (y 1) (r 1))"]

let%expect_test _ =
  simplify_test "(Circle (x 1) (y 1) (r 0))";
  [%expect "Nothing"]

let%expect_test _ =
  simplify_test "(Not Everything)";
  [%expect "Nothing"]

let%expect_test _ =
  simplify_test "(Not Nothing)";
  [%expect "Everything"]

let%expect_test _ =
  simplify_test "(Not (Not (Circle (x 1) (y 1) (r 1))))";
  [%expect "(Circle (x 1) (y 1) (r 1))"]

let%expect_test _ =
  simplify_test "(Poly ())";
  [%expect "Nothing"]

let%expect_test _ =
  simplify_test "(Union (Everything Nothing))";
  [%expect "Everything"]

let%expect_test _ =
  simplify_test "(Intersection (Everything Nothing))";
  [%expect "Nothing"]

let%expect_test _ =
  simplify_test "(Intersection ((Circle (x 10) (y 10) (r 10))))";
  [%expect "(Circle (x 10) (y 10) (r 10))"]

let%expect_test _ =
  simplify_test "(Union ((Circle (x 10) (y 10) (r 10))))";
  [%expect "(Circle (x 10) (y 10) (r 10))"]

let%expect_test _ =
  simplify_test "(Intersection ((Circle (x 20) (y 20) (r 20)) (Circle (x 10) (y 10) (r 10))))";
  [%expect "(Intersection ((Circle (x 20) (y 20) (r 20)) (Circle (x 10) (y 10) (r 10))))"]

let%expect_test _ =
  simplify_test "(Union ((Circle (x 20) (y 20) (r 20)) (Circle (x 10) (y 10) (r 10))))";
  [%expect "(Union ((Circle (x 20) (y 20) (r 20)) (Circle (x 10) (y 10) (r 10))))"]

let%expect_test _ =
  simplify_test "(Union (Nothing (Circle (x 20) (y 20) (r 20)) (Circle (x 10) (y 10) (r 10))))";
  [%expect "(Union ((Circle (x 20) (y 20) (r 20)) (Circle (x 10) (y 10) (r 10))))"]

let%expect_test _ =
  simplify_test "(Intersection (Everything (Circle (x 20) (y 20) (r 20)) (Circle (x 10) (y 10) (r 10))))";
  [%expect "(Intersection ((Circle (x 20) (y 20) (r 20)) (Circle (x 10) (y 10) (r 10))))"]

let%expect_test _ =
  simplify_test "(Union (Everything (Circle (x 10) (y 10) (r 10))))";
  [%expect "Everything"]

let%expect_test _ =
  simplify_test "(Union (Nothing (Circle (x 10) (y 10) (r 10))))";
  [%expect "(Circle (x 10) (y 10) (r 10))"]
