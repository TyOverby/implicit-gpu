open Core
open Point

type bbox = { x: float; y: float; w: float; h: float} [@@deriving sexp]

type bounding =
  | Everything
  | Nothing
  | Positive of bbox
  | Negative of bbox
[@@deriving sexp]

let intersects (a: bbox) (b: bbox)  =
  a.x < b.x +. b.w
  && b.x < a.x +. a.w
  && a.y < b.y +. b.h
  && b.y < a.y +. a.h

let left_side { x; w; _ } = x +. w
let bottom_side { y; h; _ } = y +. h

let box_union a b =
  let {x=xa; y=ya; _}: bbox = a in
  let {x=xb; y=yb; _}: bbox = b in
  let min_x = Float.min_inan xa xb in
  let min_y = Float.min_inan ya yb in
  let max_x = Float.max_inan (left_side a) (left_side b) in
  let max_y = Float.max_inan (bottom_side a) (bottom_side b) in
  {
    x = min_x;
    y = min_y;
    w = max_x -. min_x;
    h = max_y -. min_y;
  }


let box_intersection a b =
  let  {x=xa; y=ya; _}: bbox = a in
  let  {x=xb; y=yb; _}: bbox = b in
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

let bbox_of_poly (poly: Point.t list) =
  List.fold poly ~init:None ~f:(fun a b -> match (a, b) with
      | (None, { x; y }) -> Some({ x=x; y=y; w=0.0; h=0.0 })
      | (Some prev, {x; y}) -> let new_box = { x=x; y=y; w=0.0; h=0.0 } in
        Some(box_union prev new_box)
    )

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
    let decode a = a |> Sexp.of_string |>  bbox_of_sexp in
    let a = decode a in
    let b = decode b in
    let ab = f a b in
    let ba = f b a in
    assert(ab = ba);
    ab
    |> sexp_of_bbox
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
    let convert s = s |> Sexp.of_string |> bbox_of_sexp in
    let a = "((x 0) (y 0) (w 10) (h 10))" |> convert in
    let b = "((x 10) (y 10) (w 10) (h 10))" |> convert in
    (box_intersection a b)
    |> sexp_of_option sexp_of_bbox
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
