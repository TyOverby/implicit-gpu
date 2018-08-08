open Core
open Shape
open Bbox

let rec compute_bounding_box: (Shape.allTerminals, Nothing.t) Shape.t -> Bbox.bounding = function
  | Transform n -> Nothing.unreachable_code n
  | Intersection [] -> Nothing
  | Union [] -> Nothing
  | Terminal (Everything: Shape.allTerminals) -> Everything
  | Terminal Nothing -> Nothing
  | Terminal Poly { points = []; _ } -> Nothing
  | Terminal Circle { r=0.0; _ } -> Nothing

  | Terminal Circle { x; y; r; mat } ->
    let box = { x=x -. r; y=y -. r; h=2.0 *. r; w=2.0 *. r; } |> Matrix.apply_to_rect mat in
    Positive box
  | Terminal Rect { x; y; w; h; mat } ->
    let box = { x=x; y=y; w=w; h=h; } |> Matrix.apply_to_rect mat in
    Positive box
  | Terminal Poly { points; mat } ->
    let box  =
      points
      |> List.map ~f:(Matrix.apply_to_point mat)
      |> bbox_of_points
      |> Option.value_exn in
    Positive box
  | Not target -> target |> compute_bounding_box |> Bbox.inverse
  | Union targets -> targets |> compute_all_bounding_box |> List.reduce_exn ~f:Bbox.union
  | Intersection targets -> targets |> compute_all_bounding_box |> List.reduce_exn ~f:Bbox.intersection
  | Modulate (target, how_much) -> compute_bounding_box target |> (Fn.flip Bbox.grow) how_much
and compute_all_bounding_box list = list |> List.map ~f: compute_bounding_box
