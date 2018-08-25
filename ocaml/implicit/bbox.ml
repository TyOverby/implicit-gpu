open Core
open Point

type t = { x: float; y: float; w: float; h: float} [@@deriving sexp]

type bounding =
  | Everything
  | Nothing
  | Positive of t
  | Negative of t
[@@deriving sexp]

let from_extrema min_x min_y max_x max_y = {
  x = min_x;
  y = min_y;
  w = max_x -. min_x;
  h = max_y -. min_y;
}

let grow_by factor box =
  let dx = Float.max_inan 2.0 (box.w *. factor) in
  let dy = Float.max_inan 2.0 (box.h *. factor) in
  {
    x = box.x -. dx;
    y = box.y -. dy;
    w = box.w +. dx *. 2.0;
    h = box.h +. dy *. 2.0;
  }

let bbox_of_points = function
  | [] -> None
  | points ->
    let extract f g = points |> List.map ~f:f |> g |> Option.value_exn in
    let get_x (p: Point.t) = p.x in
    let get_y (p: Point.t) = p.y in
    let minimum l = List.min_elt l ~compare:Float.compare in
    let maximum l = List.max_elt l ~compare:Float.compare in
    let min_x = extract get_x minimum in
    let min_y = extract get_y minimum in
    let max_x = extract get_x maximum in
    let max_y = extract get_y maximum in
    Some (from_extrema min_x min_y max_x max_y)

let intersects (a: t) (b: t)  =
  a.x < b.x +. b.w
  && b.x < a.x +. a.w
  && a.y < b.y +. b.h
  && b.y < a.y +. a.h

let left_side { x; w; _ } = x +. w
let bottom_side { y; h; _ } = y +. h

let box_union a b =
  let {x=xa; y=ya; _}: t = a in
  let {x=xb; y=yb; _}: t = b in
  let min_x = Float.min_inan xa xb in
  let min_y = Float.min_inan ya yb in
  let max_x = Float.max_inan (left_side a) (left_side b) in
  let max_y = Float.max_inan (bottom_side a) (bottom_side b) in
  from_extrema min_x min_y max_x max_y

let box_intersection a b =
  let  {x=xa; y=ya; _}: t = a in
  let  {x=xb; y=yb; _}: t = b in
  if not (intersects a b) then
    None
  else
    let min_x = Float.max_inan xa xb in
    let min_y = Float.max_inan ya yb in
    let max_x = Float.min_inan (left_side a) (left_side b) in
    let max_y = Float.min_inan (bottom_side a) (bottom_side b) in
    Some {
      x = min_x;
      y = min_y;
      w = max_x -. min_x;
      h = max_y -. min_y;
    }

let inverse = function
  | Everything -> Nothing
  | Nothing -> Everything
  | Positive a -> Negative a
  | Negative a -> Positive a

let rec union a b = match (a, b) with
  | (Everything, _) | (_, Everything) -> Everything
  | (Nothing, o) | (o, Nothing) -> o

  | (Positive a, Positive b) -> Positive (box_union a b)
  | (Negative a, Negative b) -> (match box_intersection a b with
      | Some box ->  Negative box
      | None -> Everything)
  | (Positive _, Negative b) -> Negative b
  | (Negative _, Positive _) -> union b a

let rec intersection a b = match (a, b) with
  | (o, Everything) | (Everything, o) -> o
  | (Nothing, _) | (_, Nothing) -> Nothing

  | (Positive a, Positive b) -> (match box_intersection a b with
      | Some box -> Positive box
      | None -> Nothing)
  | (Negative a, Negative b) -> Negative (box_union a b)
  (*TODO: if we had a `val cut: bbox -> bbox -> bbox`
    then we could narrow this down further by making
    the "then" branch be `Positive cut_result` *)
  | (Positive a, Negative _) -> Positive a
  | (Negative _, Positive _) -> intersection b a

let rec grow bounding how_much = match bounding with
  | Everything  -> Everything
  | Nothing -> Nothing
  | Positive b -> Positive (increase b how_much)
  | Negative b -> Negative (decrease b how_much)
and increase { x; y; w; h } how_much = {
  x=x -. how_much;
  y=y -. how_much;
  w=w +. how_much *. 2.0;
  h=h +. how_much *. 2.0;
}
and decrease a how_much = increase a (how_much *. -1.0)

module BboxExpectTests = struct
  let box_test_stub f a b =
    let decode a = a |> Sexp.of_string |>  t_of_sexp in
    let a = decode a in
    let b = decode b in
    let ab = f a b in
    let ba = f b a in
    assert(ab = ba);
    ab
    |> sexp_of_t
    |> Sexp.to_string_hum
    |> print_endline

  let bounding_test_stub f a b =
    let decode a = a |> Sexp.of_string |>  bounding_of_sexp in
    let a = decode a in
    let b = decode b in
    let ab = f a b in
    let ba = f b a in
    assert(ab = ba);
    ab
    |> sexp_of_bounding
    |> Sexp.to_string_hum
    |> print_endline

  let union_box_test = box_test_stub box_union
  let intersection_box_test_some = box_test_stub (fun a b -> (box_intersection a b) |> Option.value_exn)

  let union_test = bounding_test_stub union
  let intersection_test = bounding_test_stub intersection

  let%expect_test _ =
    union_box_test "((x 0) (y 0) (w 10) (h 10))" "((x 0) (y 0) (w 10) (h 10))";
    [%expect "((x 0) (y 0) (w 10) (h 10))"]

  let%expect_test _ =
    union_box_test "((x 0) (y 0) (w 10) (h 10))" "((x 0) (y 0) (w 20) (h 20))";
    [%expect "((x 0) (y 0) (w 20) (h 20))"]

  let%expect_test _ =
    union_box_test "((x 0) (y 0) (w 10) (h 10))" "((x 10) (y 10) (w 10) (h 10))";
    [%expect "((x 0) (y 0) (w 20) (h 20))"]

  let%expect_test _ =
    union_box_test "((x 0) (y 0) (w 10) (h 10))" "((x 15) (y 15) (w 5) (h 5))";
    [%expect "((x 0) (y 0) (w 20) (h 20))"]

  let%expect_test _ =
    union_box_test "((x 0) (y 0) (w 10) (h 10))" "((x 15) (y 15) (w 5) (h 5))";
    [%expect "((x 0) (y 0) (w 20) (h 20))"]

  let%expect_test _ =
    intersection_box_test_some "((x 0) (y 0) (w 10) (h 10))" "((x 0) (y 0) (w 10) (h 10))";
    [%expect "((x 0) (y 0) (w 10) (h 10))"]

  let%expect_test _ =
    intersection_box_test_some "((x 0) (y 0) (w 10) (h 10))" "((x 5) (y 5) (w 10) (h 10))";
    [%expect "((x 5) (y 5) (w 5) (h 5))"]

  let%expect_test _ =
    intersection_box_test_some "((x 0) (y 0) (w 10) (h 10))" "((x 5) (y 0) (w 10) (h 10))";
    [%expect "((x 5) (y 0) (w 5) (h 10))"]

  let%expect_test _ =
    let convert s = s |> Sexp.of_string |> t_of_sexp in
    let a = "((x 0) (y 0) (w 10) (h 10))" |> convert in
    let b = "((x 10) (y 10) (w 10) (h 10))" |> convert in
    (box_intersection a b)
    |> sexp_of_option sexp_of_t
    |> Sexp.to_string_hum
    |> print_endline;
    [%expect "()"]

  let%expect_test _ =
    union_test "Everything" "Everything";
    [%expect "Everything"]

  let%expect_test _ =
    union_test "Everything" "Nothing";
    [%expect "Everything"]

  let%expect_test _ =
    union_test "(Positive ((x 10) (y 10) (w 10) (h 10)))" "Nothing";
    [%expect "(Positive ((x 10) (y 10) (w 10) (h 10)))"]

  let%expect_test _ =
    union_test "(Positive ((x 10) (y 10) (w 10) (h 10)))" "(Negative ((x 50) (y 50) (w 10) (h 10)))";
    [%expect "(Negative ((x 50) (y 50) (w 10) (h 10)))"]

  let%expect_test _ =
    union_test "(Positive ((x 10) (y 10) (w 10) (h 10)))" "(Negative ((x 5) (y 5) (w 10) (h 10)))";
    [%expect "(Negative ((x 5) (y 5) (w 10) (h 10)))"]

  let%expect_test _ =
    intersection_test "Everything" "Everything";
    [%expect "Everything"]

  let%expect_test _ =
    intersection_test "Everything" "Nothing";
    [%expect "Nothing"]

  let%expect_test _ =
    intersection_test "(Positive ((x 10) (y 10) (w 10) (h 10)))" "Nothing";
    [%expect "Nothing"]

  let%expect_test _ =
    intersection_test "(Positive ((x 10) (y 10) (w 10) (h 10)))" "(Negative ((x 50) (y 50) (w 10) (h 10)))";
    [%expect "(Positive ((x 10) (y 10) (w 10) (h 10)))"]

  let%expect_test _ =
    intersection_test "(Positive ((x 10) (y 10) (w 10) (h 10)))" "(Negative ((x 5) (y 5) (w 10) (h 10)))";
    [%expect "(Positive ((x 10) (y 10) (w 10) (h 10)))"]
end
