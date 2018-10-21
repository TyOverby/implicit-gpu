open Core
open Shape
open Compute_bb
open Creator

type id = int
[@@deriving sexp]

type basicTerminals =
  | Circle of Shape.circle
  | Rect of Shape.rect
  | Field of id
[@@deriving sexp]

type exportShape = basicTerminals Shape.t
[@@deriving sexp]

type exportPoly = {
  points: Point.t list;
  matrix: Matrix.t;
} [@@deriving sexp]

type exportSimplex = {
  cutoff: float;
  matrix: Matrix.t;
} [@@deriving sexp]

type value =
  | BasicShape of exportShape
  | Polygon of exportPoly
[@@deriving sexp]


type t =
  | Concurrently of t list
  | Serially of t list
  | Define of id * value
  | Freeze of { target: id; id: id }
  | Drag of { target: id; id: id; dx: float; dy: float }
  | Simplex of id * exportSimplex
  | Export of id
[@@deriving sexp]

type id_gen = {
  next: int Ref.t
}

let get_id gen =
  let next = !(gen.next) in
  gen.next := (next + 1);
  next

let rec breakup_shape id_gen commands matrix = function
  | Terminal Poly p ->
    let id = get_id id_gen in
    let p = {
      points = p.points;
      matrix
    } in
    commands := (Define(id, Polygon p) :: (!commands));
    Terminal (Field id)
  | Terminal (Simplex {cutoff}) ->
    let id = get_id id_gen in
    commands := (Simplex (id, {cutoff; matrix})) :: !commands;
    Terminal (Field id)
  | Drag (t, dx, dy) ->
    let t = breakup_shape id_gen commands matrix t in
    let t = Transform (t, matrix) in (* Apply cumulative matrix up to this point *)
    let shape_id = get_id id_gen in
    let drag_id = get_id id_gen in
    let def = Define(shape_id, BasicShape t) in
    let drug = Drag {target = shape_id; id = drag_id; dx; dy} in (* TODO: APPLY MATRIX *)
    commands := List.concat [!commands; [def; drug]] ;
    Terminal (Field drag_id)
  | Freeze t ->
    let t = breakup_shape id_gen commands matrix t in
    let t = Transform (t, matrix) in (* Apply cumulative matrix up to this point *)
    let shape_id = get_id id_gen in
    let freeze_id = get_id id_gen in
    let def = Define(shape_id, BasicShape t) in
    let frz = Freeze { target = shape_id; id = freeze_id } in
    commands := List.concat [!commands; [def; frz]];
    Terminal (Field freeze_id)
  | Terminal Circle c -> Terminal (Circle c)
  | Terminal Rect r -> Terminal (Rect r)
  | Transform (t, m) -> Transform (breakup_shape id_gen commands (Matrix.mul matrix m) t, m)
  | Not t -> Not (breakup_shape id_gen commands matrix t)
  | Union all -> Union (breakup_all id_gen commands matrix all)
  | Intersection all -> Intersection (breakup_all id_gen commands matrix all)
  | Modulate (t, by) -> Modulate (breakup_shape id_gen commands matrix t, by)
and breakup_all id_gen commands matrix all =
  List.map all ~f:(breakup_shape id_gen commands matrix)

let compile_commands shape =
  let commands = ref [] in
  let id_gen = { next = ref 0 } in
  let result_shape = breakup_shape id_gen commands Matrix.id shape in
  let last_id = get_id id_gen in
  let last = Define(last_id, BasicShape result_shape) in
  let commands = !commands in
  if List.is_empty commands
  then Serially [last; Export last_id]
  else Serially [Serially commands; last; Export last_id]

let compile shape =
  let bounding = compute_bounding_box shape in
  (*bounding |> Bbox.sexp_of_bounding |> Sexp.to_string_hum |> print_endline ; *)
  match bounding with
  | { positive = Everything | Nothing ; _ } -> None
  | { positive = Something bb; _ } ->
    let expanded = bb |> Bbox.grow_by 0.1 in
    let repositioned = shape |> translate ~dx: (-. expanded.x) ~dy: (-. expanded.y) in
    let simplified = Simplify.simplify repositioned in
    (match simplified with
     | SEverything | SNothing -> None
     | SShape s -> Some (compile_commands s, (expanded.w, expanded.h)))
  | { positive = Hole bb; _ } ->
    let expanded = bb |> Bbox.grow_by 0.1 in
    let expanded_twice = expanded |> Bbox.grow_by 0.1 in
    let surrounding = rect ~x: expanded.x ~y: expanded.y ~w: expanded.w ~h:expanded.h in
    let surrounded = intersection [surrounding; shape] in
    let repositioned = surrounded |> translate ~dx: (-. expanded_twice.x) ~dy: (-. expanded_twice.y) in
    let simplified = Simplify.simplify repositioned in
    (match simplified with
     | SEverything | SNothing -> None
     | SShape s -> Some (compile_commands s, (expanded_twice.w, expanded_twice.h)))

module Command_Tests = struct
  open Creator
  let test_shape_compile shape =
    shape
    |> compile
    |> Option.sexp_of_t (Tuple2.sexp_of_t sexp_of_t (Tuple2.sexp_of_t Float.sexp_of_t Float.sexp_of_t))
    |> Sexp.to_string_hum
    |> print_endline

  let%expect_test _ =
    circle ~x:0.0 ~y:0.0 ~r:0.0
    |> test_shape_compile;
    [%expect "()"]


  let%expect_test _ =
    noise 0.5
    |> test_shape_compile;
    [%expect "()"]

  let%expect_test _ =
    let inf_big = noise 0.5 in
    let inf_small = noise 0.3 in
    let cut = circle ~x:0.0 ~y:0.0 ~r:1.0 in
    subtract (subtract inf_big inf_small) cut
    |> test_shape_compile;
    [%expect "
      (((Serially
         ((Serially
           ((Simplex 1
             ((cutoff 0.3)
              (matrix ((m11 1) (m12 0) (m21 0) (m22 1) (m31 5) (m32 5)))))
            (Simplex 0
             ((cutoff 0.5)
              (matrix ((m11 1) (m12 0) (m21 0) (m22 1) (m31 5) (m32 5)))))))
          (Define 2
           (BasicShape
            (Transform
             (Intersection
              ((Terminal (Rect ((x -3) (y -3) (w 6) (h 6))))
               (Intersection
                ((Intersection ((Terminal (Field 0)) (Not (Terminal (Field 1)))))
                 (Not (Terminal (Circle ((x 0) (y 0) (r 1)))))))))
             ((m11 1) (m12 0) (m21 0) (m22 1) (m31 5) (m32 5)))))
          (Export 2)))
        (10 10)))"]

  let%expect_test _ =
    let inf = noise 0.5 in
    let cut = circle ~x:0.0 ~y:0.0 ~r:1.0 in
    subtract inf cut
    |> test_shape_compile;
    [%expect "
      (((Serially
         ((Serially
           ((Simplex 0
             ((cutoff 0.5)
              (matrix ((m11 1) (m12 0) (m21 0) (m22 1) (m31 5) (m32 5)))))))
          (Define 1
           (BasicShape
            (Transform
             (Intersection
              ((Terminal (Rect ((x -3) (y -3) (w 6) (h 6))))
               (Intersection
                ((Terminal (Field 0))
                 (Not (Terminal (Circle ((x 0) (y 0) (r 1)))))))))
             ((m11 1) (m12 0) (m21 0) (m22 1) (m31 5) (m32 5)))))
          (Export 1)))
        (10 10)))"]

  let%expect_test _ =
    circle ~x:0.0 ~y:0.0 ~r:1.0
    |> test_shape_compile;
    [%expect "
      (((Serially
         ((Define 0
           (BasicShape
            (Transform (Terminal (Circle ((x 0) (y 0) (r 1))))
             ((m11 1) (m12 0) (m21 0) (m22 1) (m31 3) (m32 3)))))
          (Export 0)))
        (6 6)))"]

  let%expect_test _ =
    circle ~x:0.0 ~y:0.0 ~r:1.0
    |> not
    |> test_shape_compile;
    [%expect "
      (((Serially
         ((Define 0
           (BasicShape
            (Transform
             (Intersection
              ((Terminal (Rect ((x -3) (y -3) (w 6) (h 6))))
               (Not (Terminal (Circle ((x 0) (y 0) (r 1)))))))
             ((m11 1) (m12 0) (m21 0) (m22 1) (m31 5) (m32 5)))))
          (Export 0)))
        (10 10)))"]

  let%expect_test _ =
    circle ~x:1.0 ~y:1.0 ~r:1.0
    |> test_shape_compile;
    [%expect "
      (((Serially
         ((Define 0
           (BasicShape
            (Transform (Terminal (Circle ((x 1) (y 1) (r 1))))
             ((m11 1) (m12 0) (m21 0) (m22 1) (m31 2) (m32 2)))))
          (Export 0)))
        (6 6)))"]

  let%expect_test _ =
    let left = circle ~x:1.0 ~y:1.0 ~r:1.0 in
    let right = circle ~x:1.0 ~y:1.0 ~r:1.0 in
    union [left; right]
    |> test_shape_compile;
    [%expect "
      (((Serially
         ((Define 0
           (BasicShape
            (Transform
             (Union
              ((Terminal (Circle ((x 1) (y 1) (r 1))))
               (Terminal (Circle ((x 1) (y 1) (r 1))))))
             ((m11 1) (m12 0) (m21 0) (m22 1) (m31 2) (m32 2)))))
          (Export 0)))
        (6 6)))"]

  let%expect_test _ =
    let left = circle ~x:1.0 ~y:1.0 ~r:1.0 in
    let right = rect ~x:1.0 ~y:1.0 ~w:2.0 ~h:2.0 in
    union [left; right]
    |> test_shape_compile;
    [%expect "
      (((Serially
         ((Define 0
           (BasicShape
            (Transform
             (Union
              ((Terminal (Circle ((x 1) (y 1) (r 1))))
               (Terminal (Rect ((x 1) (y 1) (w 2) (h 2))))))
             ((m11 1) (m12 0) (m21 0) (m22 1) (m31 2) (m32 2)))))
          (Export 0)))
        (7 7)))"]

  let%expect_test _ =
    let left = circle ~x:1.0 ~y:1.0 ~r:1.0 in
    let right = rect ~x:1.0 ~y:1.0 ~w:2.0 ~h:2.0 in
    union [freeze left; freeze right]
    |> test_shape_compile;
    [%expect "
      (((Serially
         ((Serially
           ((Define 0
             (BasicShape
              (Transform (Terminal (Circle ((x 1) (y 1) (r 1))))
               ((m11 1) (m12 0) (m21 0) (m22 1) (m31 2) (m32 2)))))
            (Freeze (target 0) (id 1))
            (Define 2
             (BasicShape
              (Transform (Terminal (Rect ((x 1) (y 1) (w 2) (h 2))))
               ((m11 1) (m12 0) (m21 0) (m22 1) (m31 2) (m32 2)))))
            (Freeze (target 2) (id 3))))
          (Define 4
           (BasicShape
            (Transform (Union ((Terminal (Field 1)) (Terminal (Field 3))))
             ((m11 1) (m12 0) (m21 0) (m22 1) (m31 2) (m32 2)))))
          (Export 4)))
        (7 7)))"]

  let%expect_test _ =
    circle ~x:1.0 ~y:1.0 ~r:1.0
    |> drag ~dx:10.0 ~dy:5.0
    |> test_shape_compile;
    [%expect "
      (((Serially
         ((Serially
           ((Define 0
             (BasicShape
              (Transform (Terminal (Circle ((x 1) (y 1) (r 1))))
               ((m11 1) (m12 0) (m21 0) (m22 1) (m31 2) (m32 2)))))
            (Drag (target 0) (id 1) (dx 10) (dy 5))))
          (Define 2
           (BasicShape
            (Transform (Terminal (Field 1))
             ((m11 1) (m12 0) (m21 0) (m22 1) (m31 2) (m32 2)))))
          (Export 2)))
        (16 11)))"]
end
