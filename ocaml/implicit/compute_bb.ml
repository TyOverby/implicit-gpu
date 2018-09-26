open Core
open Shape
open Bbox

let both_everything = {
  positive = Everything;
  negative = Everything;
}

let rec compute_bounding_box = function
  | Transform (target, matrix) ->
    let bb = compute_bounding_box target in
    let positive = match bb.positive with
      | Something b -> Something (Matrix.apply_to_rect matrix b)
      | Hole b -> Hole (Matrix.apply_to_rect matrix b)
      | Everything -> Everything
      | Nothing -> Nothing
    in
    let negative = match bb.negative with
      | Something b -> Something (Matrix.apply_to_rect matrix b)
      | Hole b -> Hole (Matrix.apply_to_rect matrix b)
      | Everything -> Everything
      | Nothing -> Nothing
    in
    { positive; negative }
  | Intersection []
  | Union []
  | Terminal Poly { points = []; _ }
  | Terminal Circle { r=0.0; _ } ->
    { positive = Nothing
    ; negative = Everything; }
  | Terminal Simplex _ -> both_everything

  | Terminal Circle { x; y; r; } ->
    let bb = { x=x -. r; y=y -. r; h=2.0 *. r; w=2.0 *. r; } in
    {
      positive = Something bb;
      negative = Hole bb;
    }
  | Terminal Rect { x; y; w; h; } ->
    let bb = { x=x; y=y; w=w; h=h; } in
    {
      positive = Something bb;
      negative = Hole bb;
    }
  | Terminal Poly { points; matrix; } ->
    let box  =
      points
      |> List.map ~f:(Matrix.apply_to_point matrix)
      |> bbox_of_points
      |> Option.value_exn in
    { positive = Something box
    ; negative = Everything
    }
  | Not target -> target |> compute_bounding_box |> Bbox.inverse
  | Freeze target -> target |> compute_bounding_box
  | Union targets -> targets |> compute_all_bounding_box |> Bbox.union_all
  | Intersection targets -> targets |> compute_all_bounding_box |> Bbox.intersection_all
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
    [%expect "
      ((positive (Something ((x 0) (y 0) (w 10) (h 20))))
       (negative (Hole ((x 0) (y 0) (w 10) (h 20)))))"]

  let%expect_test _ =
    rect ~x:0.0 ~y:0.0 ~w:10.0 ~h:10.0
    |> scale ~dx:3.0 ~dy: 5.0
    |> run_bb_test;
    [%expect "
      ((positive (Something ((x 0) (y 0) (w 30) (h 50))))
       (negative (Hole ((x 0) (y 0) (w 30) (h 50)))))"]


  let%expect_test _ =
    rect ~x:0.0 ~y:0.0 ~w:10.0 ~h:10.0
    |> scale ~dx:3.0 ~dy: 3.0
    |> translate ~dx:5.0 ~dy:5.0
    |> run_bb_test;
    [%expect "
      ((positive (Something ((x 5) (y 5) (w 30) (h 30))))
       (negative (Hole ((x 5) (y 5) (w 30) (h 30)))))"]

  let%expect_test _ =
    circle ~x:0.0 ~y:0.0 ~r:10.0
    |> scale ~dx:3.0 ~dy: 3.0
    |> run_bb_test;
    [%expect "
      ((positive (Something ((x -30) (y -30) (w 60) (h 60))))
       (negative (Hole ((x -30) (y -30) (w 60) (h 60)))))"]

  let%expect_test _ =
    rect ~x:(-.10.0) ~y:(-.10.0) ~w:20.0 ~h:20.0
    |> scale ~dx:3.0 ~dy: 3.0
    |> run_bb_test;
    [%expect "
      ((positive (Something ((x -30) (y -30) (w 60) (h 60))))
       (negative (Hole ((x -30) (y -30) (w 60) (h 60)))))"]


  let%expect_test _ =
    let outer = circle ~x:0.0 ~y:0.0 ~r:10.0 in
    let inner = circle ~x:0.0 ~y:0.0 ~r:5.0 in
    let ring = intersection [outer; not inner] in
    ring
    |> scale ~dy:3.0 ~dx:3.0
    |> run_bb_test;
    [%expect "
      ((positive (Something ((x -30) (y -30) (w 60) (h 60))))
       (negative (Hole ((x -30) (y -30) (w 60) (h 60)))))"]

  let%expect_test _ =
    let outer = circle ~x:0.0 ~y:0.0 ~r:10.0 in
    let inner = circle ~x:0.0 ~y:0.0 ~r:5.0 in
    let ring = intersection [outer; not inner] in
    ring
    |> scale ~dy:3.0 ~dx:3.0
    |> translate ~dy:30.0 ~dx:30.0
    |> run_bb_test;
    [%expect "
      ((positive (Something ((x 0) (y 0) (w 60) (h 60))))
       (negative (Hole ((x 0) (y 0) (w 60) (h 60)))))"]

  let%expect_test _ =
    let outer = circle ~x:0.0 ~y:0.0 ~r:10.0 in
    let inner = circle ~x:0.0 ~y:0.0 ~r:5.0 in
    let ring = intersection [outer; not inner] in
    ring
    |> scale ~dy:3.0 ~dx:3.0
    |> translate ~dy:30.0 ~dx:30.0
    |> not
    |> run_bb_test;
    [%expect "
      ((positive (Hole ((x 0) (y 0) (w 60) (h 60))))
       (negative (Something ((x 0) (y 0) (w 60) (h 60)))))"]

  let%expect_test _ =
    circle ~x:0.0 ~y:0.0 ~r:10.0
    |> scale ~dy:3.0 ~dx:3.0
    |> run_bb_test;
    [%expect "
      ((positive (Something ((x -30) (y -30) (w 60) (h 60))))
       (negative (Hole ((x -30) (y -30) (w 60) (h 60)))))"]

  let%expect_test _ =
    circle ~x:0.0 ~y:0.0 ~r:10.0
    |> modulate 10.0
    |> run_bb_test;
    [%expect "
      ((positive (Something ((x -210) (y -210) (w 420) (h 420))))
       (negative (Hole ((x -210) (y -210) (w 420) (h 420)))))"]
end
