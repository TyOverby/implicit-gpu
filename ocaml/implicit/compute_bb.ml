open Core
open Shape
open Bbox

let rec compute_bounding_box = function
  | Transform n -> Nothing.unreachable_code n
  | Intersection [] -> Nothing
  | Union [] -> Nothing
  | Terminal Poly { points = []; _ } -> Nothing
  | Terminal Circle { r=0.0; _ } -> Nothing

  | Terminal Circle { x; y; r; mat } ->
    let box = { x=x -. r; y=y -. r; h=2.0 *. r; w=2.0 *. r; } in
    let box = box |> Matrix.apply_to_rect mat in
    Positive box
  | Terminal Rect { x; y; w; h; mat } ->
    let box = { x=x; y=y; w=w; h=h; } in
    let box = box |> Matrix.apply_to_rect mat in
    Positive box
  | Terminal Poly { points; mat } ->
    let box  =
      points
      |> List.map ~f:(Matrix.apply_to_point mat)
      |> bbox_of_points
      |> Option.value_exn in
    Positive box
  | Not target -> target |> compute_bounding_box |> Bbox.inverse
  | Freeze target -> target |> compute_bounding_box
  | Union targets -> targets |> compute_all_bounding_box |> List.reduce_exn ~f:Bbox.union
  | Intersection targets -> targets |> compute_all_bounding_box |> List.reduce_exn ~f:Bbox.intersection
  | Modulate (target, how_much) -> compute_bounding_box target |> (Fn.flip Bbox.grow) how_much
and compute_all_bounding_box list = list |> List.map ~f: compute_bounding_box

let%expect_test _ =
  "(Terminal (Rect ((x 0) (y 0) (w 10) (h 20))))"
  |> Sexp.of_string
  |> Stages.propagated_of_sexp
  |> compute_bounding_box
  |> sexp_of_bounding
  |> Sexp.to_string_hum
  |> print_endline;
  [%expect "(Positive ((x 0) (y 0) (w 10) (h 20)))"]

let%expect_test _ =
  "
  (Transform (Scale
    (Terminal (Rect ((x 0) (y 0) (w 10) (h 10))))
    ((dx 3.0) (dy 3.0))
  ))
  "
  |> Sexp.of_string
  |> Stages.user_of_sexp
  |> Prop.remove_transformations
  |> compute_bounding_box
  |> sexp_of_bounding
  |> Sexp.to_string_hum
  |> print_endline;
  [%expect "(Positive ((x 0) (y 0) (w 30) (h 30)))"]


let%expect_test _ =
  "
  (Transform (Translate
    (Transform (Scale
      (Terminal (Rect ((x 0) (y 0) (w 10) (h 10))))
      ((dx 3.0) (dy 3.0))
    ))
    ((dx 5.0) (dy 5.0))
  ))
  "
  |> Sexp.of_string
  |> Stages.user_of_sexp
  |> Prop.remove_transformations
  |> compute_bounding_box
  |> sexp_of_bounding
  |> Sexp.to_string_hum
  |> print_endline;
  [%expect "(Positive ((x 5) (y 5) (w 30) (h 30)))"]

let%expect_test _ =
  "
    (Transform (Scale
      (Terminal (Circle ((x 0) (y 0) (r 10))))
      ((dx 3.0) (dy 3.0))
    ))
  "
  |> Sexp.of_string
  |> Stages.user_of_sexp
  |> Prop.remove_transformations
  |> compute_bounding_box
  |> sexp_of_bounding
  |> Sexp.to_string_hum
  |> print_endline;
  [%expect "(Positive ((x -30) (y -30) (w 60) (h 60)))"]

let%expect_test _ =
  "
    (Transform (Scale
      (Terminal (Rect ((x -10) (y -10) (w 20) (h 20))))
      ((dx 3.0) (dy 3.0))
    ))
  "
  |> Sexp.of_string
  |> Stages.user_of_sexp
  |> Prop.remove_transformations
  |> compute_bounding_box
  |> sexp_of_bounding
  |> Sexp.to_string_hum
  |> print_endline;
  [%expect "(Positive ((x -30) (y -30) (w 60) (h 60)))"]


let%expect_test _ =
  "
    (Transform (Scale
      (Intersection (
        (Terminal (Circle ((x 0) (y 0) (r 10))))
        (Not (Terminal (Circle ((x 0) (y 0) (r 5)))))
      ))
      ((dx 3.0) (dy 3.0))
    ))
  "
  |> Sexp.of_string
  |> Stages.user_of_sexp
  |> Prop.remove_transformations
  |> compute_bounding_box
  |> sexp_of_bounding
  |> Sexp.to_string_hum
  |> print_endline;
  [%expect "(Positive ((x -30) (y -30) (w 60) (h 60)))"]

let%expect_test _ =
  "
  (Transform (Translate
    (Transform (Scale
      (Intersection (
        (Terminal (Circle ((x 0) (y 0) (r 10))))
        (Not (Terminal (Circle ((x 0) (y 0) (r 5)))))
      ))
      ((dx 3.0) (dy 3.0))
    ))
    ((dx 30) (dy 30))
    ))
  "
  |> Sexp.of_string
  |> Stages.user_of_sexp
  |> Prop.remove_transformations
  |> compute_bounding_box
  |> sexp_of_bounding
  |> Sexp.to_string_hum
  |> print_endline;
  [%expect "(Positive ((x 0) (y 0) (w 60) (h 60)))"]

let%expect_test _ =
  "
  (Transform (Translate
    (Transform (Scale
      (Not
        (Intersection (
          (Terminal (Circle ((x 0) (y 0) (r 10))))
          (Not (Terminal (Circle ((x 0) (y 0) (r 5)))))
        ))
      )
      ((dx 3.0) (dy 3.0))
    ))
    ((dx 30) (dy 30))
    ))
  "
  |> Sexp.of_string
  |> Stages.user_of_sexp
  |> Prop.remove_transformations
  |> compute_bounding_box
  |> sexp_of_bounding
  |> Sexp.to_string_hum
  |> print_endline;
  [%expect "(Negative ((x 0) (y 0) (w 60) (h 60)))"]

let%expect_test _ =
  "
    (Transform (Scale
        (Modulate
          (Terminal (Circle ((x 0) (y 0) (r 10))))
          10
        )
      ((dx 3.0) (dy 3.0))
    ))
  "
  |> Sexp.of_string
  |> Stages.user_of_sexp
  |> Prop.remove_transformations
  |> compute_bounding_box
  |> sexp_of_bounding
  |> Sexp.to_string_hum
  |> print_endline;
  [%expect "(Positive((x -60) (y -60) (w 120) (h 120)))"]

let%expect_test _ =
  "
        (Modulate
          (Terminal (Circle ((x 0) (y 0) (r 10))))
          10
        )
  "
  |> Sexp.of_string
  |> Stages.user_of_sexp
  |> Prop.remove_transformations
  |> compute_bounding_box
  |> sexp_of_bounding
  |> Sexp.to_string_hum
  |> print_endline;
  [%expect "(Positive((x -20) (y -20) (w 40) (h 40)))"]
