open Core
open Shape
open Bbox

let rec compute_bounding_box = function
  | Intersection [] -> failwith "empty intersection in compute_bb"
  | Union [] -> failwith "empty union in compute_bb"
  | Everything -> failwith "Everything found in compute_bb"
  | Nothing -> failwith "Nothing found in compute_bb"
  | Translate _ -> failwith "Translate in compute_bb"
  | Scale _ -> failwith "Scale in compute_bb"
  | Poly [] -> failwith "empty polygon in compute_bb"
  | Circle { r=0.0; _ } -> failwith "zero radius circle in compute_bb"

  | Circle { x; y; r } -> Positive {
      x=x -. r;
      y=y -. r;
      h=2.0 *. r;
      w=2.0 *. r;
    }
  | Rect { x; y; w; h } -> Positive {
      x=x;
      y=y;
      w=w;
      h=h;
    }
  | Poly pts -> Positive (bbox_of_poly pts |> Option.value_exn)
  | Not target -> target |> compute_bounding_box |> Bbox.inverse
  | Union targets -> targets |> List.map ~f: compute_bounding_box |> List.reduce_exn ~f:Bbox.union
  | Intersection targets -> targets |> List.map ~f: compute_bounding_box |> List.reduce_exn ~f:Bbox.intersection
  | Modulate (target, how_much) -> compute_bounding_box target |> (Fn.flip Bbox.grow) how_much
