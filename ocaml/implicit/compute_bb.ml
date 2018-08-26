open Core
open Shape
open Bbox

let rec compute_bounding_box = function
  | Transform (target, matrix) ->
    (match compute_bounding_box target with
     | Positive bb -> Positive (Matrix.apply_to_rect matrix bb)
     | Negative bb -> Negative (Matrix.apply_to_rect matrix bb)
     | Everything -> Everything
     | Nothing -> Nothing
    )
  | Intersection [] -> Nothing
  | Union [] -> Nothing
  | Terminal Poly { points = []; _ } -> Nothing
  | Terminal Circle { r=0.0; _ } -> Nothing

  | Terminal Circle { x; y; r; } -> Positive { x=x -. r; y=y -. r; h=2.0 *. r; w=2.0 *. r; }
  | Terminal Rect { x; y; w; h; } ->
    Positive { x=x; y=y; w=w; h=h; }
  | Terminal Poly { points; } ->
    let box  =
      points
      |> bbox_of_points
      |> Option.value_exn in
    Positive box
  | Not target -> target |> compute_bounding_box |> Bbox.inverse
  | Freeze target -> target |> compute_bounding_box
  | Union targets -> targets |> compute_all_bounding_box |> List.reduce_exn ~f:Bbox.union
  | Intersection targets -> targets |> compute_all_bounding_box |> List.reduce_exn ~f:Bbox.intersection
  | Modulate (target, how_much) -> compute_bounding_box target |> (Fn.flip Bbox.grow) how_much
and compute_all_bounding_box list = list |> List.map ~f: compute_bounding_box

module ComputeBB_Test = struct
  open Creator

  let run_bb_test shape =
    shape
    |> compute_bounding_box
    |> sexp_of_bounding
    |> Sexp.to_string_hum
    |> print_endline

  let%expect_test _ =
    rect ~x:0.0 ~y:0.0 ~w:10.0 ~h:20.0
    |> run_bb_test;
    [%expect "(Positive ((x 0) (y 0) (w 10) (h 20)))"]

  let%expect_test _ =
    rect ~x:0.0 ~y:0.0 ~w:10.0 ~h:10.0
    |> scale ~dx:3.0 ~dy: 5.0
    |> run_bb_test;
    [%expect "(Positive ((x 0) (y 0) (w 30) (h 50)))"]


  let%expect_test _ =
    rect ~x:0.0 ~y:0.0 ~w:10.0 ~h:10.0
    |> scale ~dx:3.0 ~dy: 3.0
    |> translate ~dx:5.0 ~dy:5.0
    |> run_bb_test;
    [%expect "(Positive ((x 5) (y 5) (w 30) (h 30)))"]

  let%expect_test _ =
    circle ~x:0.0 ~y:0.0 ~r:10.0
    |> scale ~dx:3.0 ~dy: 3.0
    |> run_bb_test;
    [%expect "(Positive ((x -30) (y -30) (w 60) (h 60)))"]

  let%expect_test _ =
    rect ~x:(-.10.0) ~y:(-.10.0) ~w:20.0 ~h:20.0
    |> scale ~dx:3.0 ~dy: 3.0
    |> run_bb_test;
    [%expect "(Positive ((x -30) (y -30) (w 60) (h 60)))"]


  let%expect_test _ =
    let outer = circle ~x:0.0 ~y:0.0 ~r:10.0 in
    let inner = circle ~x:0.0 ~y:0.0 ~r:5.0 in
    let ring = intersection [outer; not inner] in
    ring
    |> scale ~dy:3.0 ~dx:3.0
    |> run_bb_test;
    [%expect "(Positive ((x -30) (y -30) (w 60) (h 60)))"]

  let%expect_test _ =
    let outer = circle ~x:0.0 ~y:0.0 ~r:10.0 in
    let inner = circle ~x:0.0 ~y:0.0 ~r:5.0 in
    let ring = intersection [outer; not inner] in
    ring
    |> scale ~dy:3.0 ~dx:3.0
    |> translate ~dy:30.0 ~dx:30.0
    |> run_bb_test;
    [%expect "(Positive ((x 0) (y 0) (w 60) (h 60)))"]

  let%expect_test _ =
    let outer = circle ~x:0.0 ~y:0.0 ~r:10.0 in
    let inner = circle ~x:0.0 ~y:0.0 ~r:5.0 in
    let ring = intersection [outer; not inner] in
    ring
    |> scale ~dy:3.0 ~dx:3.0
    |> translate ~dy:30.0 ~dx:30.0
    |> not
    |> run_bb_test;
    [%expect "(Negative ((x 0) (y 0) (w 60) (h 60)))"]

  let%expect_test _ =
    circle ~x:0.0 ~y:0.0 ~r:10.0
    |> scale ~dy:3.0 ~dx:3.0
    |> run_bb_test;
    [%expect "(Positive ((x -30) (y -30) (w 60) (h 60)))"]

  let%expect_test _ =
    circle ~x:0.0 ~y:0.0 ~r:10.0
    |> modulate 10.0
    |> run_bb_test;
    [%expect "(Positive ((x -20) (y -20) (w 40) (h 40)))"]
end
