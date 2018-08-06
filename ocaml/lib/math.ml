open Core

type bbox = { x: float; y: float; w: float; h: float} [@@deriving sexp]
type point = { x: float; y: float } [@@deriving sexp]
type vec = { dx: float; dy: float } [@@deriving sexp]

let intresects (a: bbox) (b: bbox)  =
  a.x < b.x +. b.w
  && b.x < a.x +. a.w
  && a.y < b.y +. b.h
  && b.y < a.y +. a.h

let left_side { x; w; _ } = x +. w
let bottom_side { y; h; _ } = y +. h

let box_union (a: bbox) (b: bbox) = match (a, b) with | ({ x=xa; y=ya; _}, {x=xb; y=yb; _}) ->
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

let box_intersection (a: bbox) (b: bbox) = match (a, b) with | ({ x=xa; y=ya; _ }, { x=xb; y=yb; _ }) ->
  if not (intresects a b) then
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

let box_intersect_all list =
  List.fold list ~init:None ~f:(fun a b -> match (a, b) with
      | (None, _) -> None
      | (_, None) -> None
      | (Some a, Some b) -> box_intersection a b
    )

let box_union_all list =
  List.fold list ~init:None ~f:(fun a b -> match (a, b) with
      | (None, _) -> None
      | (_, None) -> None
      | (Some a, Some b) -> Some (box_union a b)
    )

let point ~x ~y = { x; y }
let vec ~dx ~dy = { dx; dy }
let bbox ~x ~y ~w ~h = { x; y; w; h }

let point_sub a b = { dx = a.x -. b.x; dy = a.y -. b.y }

let v_add a b = { dx = a.dx +. b.dx; dy = a.dy +. b.dy }
let v_sub a b = { dx = a.dx -. b.dx; dy = a.dy -. b.dy }
let v_mul a b = { dx = a.dx *. b.dx; dy = a.dy *. b.dy }
let v_div a b = { dx = a.dx /. b.dx; dy = a.dy /. b.dy }

let bbox_of_poly poly =
  List.fold poly ~init:None ~f:(fun a b -> match (a, b) with
      | (None, { x; y }) -> Some(bbox ~x:x ~y:y ~w:0.0 ~h:0.0)
      | (Some prev, {x; y}) -> let new_box = bbox ~x:x ~y:y ~w:0.0 ~h:0.0 in
        Some(box_union prev new_box)
    )

let test_stub f a b =
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

let union_test = test_stub box_union
let intersection_test_some = test_stub (fun a b -> (box_intersection a b) |> Option.value_exn)

let%expect_test _ =
  union_test "((x 0) (y 0) (w 10) (h 10))" "((x 0) (y 0) (w 10) (h 10))";
  [%expect "((x 0) (y 0) (w 10) (h 10))"]

let%expect_test _ =
  union_test "((x 0) (y 0) (w 10) (h 10))" "((x 0) (y 0) (w 20) (h 20))";
  [%expect "((x 0) (y 0) (w 20) (h 20))"]

let%expect_test _ =
  union_test "((x 0) (y 0) (w 10) (h 10))" "((x 10) (y 10) (w 10) (h 10))";
  [%expect "((x 0) (y 0) (w 20) (h 20))"]

let%expect_test _ =
  union_test "((x 0) (y 0) (w 10) (h 10))" "((x 15) (y 15) (w 5) (h 5))";
  [%expect "((x 0) (y 0) (w 20) (h 20))"]

let%expect_test _ =
  union_test "((x 0) (y 0) (w 10) (h 10))" "((x 15) (y 15) (w 5) (h 5))";
  [%expect "((x 0) (y 0) (w 20) (h 20))"]

let%expect_test _ =
  intersection_test_some "((x 0) (y 0) (w 10) (h 10))" "((x 0) (y 0) (w 10) (h 10))";
  [%expect "((x 0) (y 0) (w 10) (h 10))"]

let%expect_test _ =
  intersection_test_some "((x 0) (y 0) (w 10) (h 10))" "((x 5) (y 5) (w 10) (h 10))";
  [%expect "((x 5) (y 5) (w 5) (h 5))"]

let%expect_test _ =
  intersection_test_some "((x 0) (y 0) (w 10) (h 10))" "((x 5) (y 0) (w 10) (h 10))";
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
