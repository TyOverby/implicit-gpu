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

type value =
  | BasicShape of exportShape
  | Polygon of Shape.poly
[@@deriving sexp]


type t =
  | Concurrently of t list
  | Serially of t list
  | Define of id * value
  | Freeze of { target: id; id: id }
  | Export of id
[@@deriving sexp]

type id_gen = {
  next: int Ref.t
}

let get_id gen =
  let next = !(gen.next) in
  gen.next := (next + 1);
  next

let _compile_commands shape =
  let commands = ref [] in
  let id_gen = { next = ref 0 } in
  let result_shape =
    let map_terminal = function
      | Poly p ->
        let id = get_id id_gen in
        commands := (Define(id, Polygon p) :: (!commands));
        Field id
      | Circle c -> Circle c
      | Rect r -> Rect r in
    let map_shape: exportShape -> exportShape = function
      | Freeze shape ->
        let shape_id = get_id id_gen in
        let freeze_id = get_id id_gen in
        let def = Define(shape_id, BasicShape shape) in
        let frz = Freeze { target = shape_id; id = freeze_id } in
        commands := List.concat [!commands; [def; frz]];
        Terminal (Field freeze_id)
      | other -> other
    in
    shape |> Shape.visit map_shape map_terminal in
  let last_id = get_id id_gen in
  let last = Define(last_id, BasicShape result_shape) in
  let commands = !commands in
  if List.is_empty commands
  then Serially [last; Export last_id]
  else Serially [Serially commands; last; Export last_id]

let compile shape = match compute_bounding_box shape with
  | Everything | Nothing -> None
  | Positive bb ->
    let expanded = bb |> Bbox.grow_by 0.1 in
    let repositioned = shape |> translate ~dx: (-. expanded.x) ~dy: (-. expanded.y) in
    let simplified = Simplify.simplify repositioned in
    (match simplified with
     | SEverything | SNothing -> None
     | SShape s -> Some (_compile_commands s, (expanded.w, expanded.h)))
  | Negative bb ->
    let expanded = bb |> Bbox.grow_by 0.1 in
    let expanded_twice = expanded |> Bbox.grow_by 0.1 in
    let surrounding = rect ~x: expanded.x ~y: expanded.y ~w: expanded.w ~h:expanded.h in
    let surrounded = intersection [surrounding; shape] in
    let repositioned = surrounded |> translate ~dx: (-. expanded_twice.x) ~dy: (-. expanded_twice.y) in
    let simplified = Simplify.simplify repositioned in
    (match simplified with
     | SEverything | SNothing -> None
     | SShape s -> Some (_compile_commands s, (expanded_twice.w, expanded_twice.h)))

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
           ((Define 0 (BasicShape (Terminal (Circle ((x 1) (y 1) (r 1))))))
            (Freeze (target 0) (id 1))
            (Define 2 (BasicShape (Terminal (Rect ((x 1) (y 1) (w 2) (h 2))))))
            (Freeze (target 2) (id 3))))
          (Define 4
           (BasicShape
            (Transform (Union ((Terminal (Field 1)) (Terminal (Field 3))))
             ((m11 1) (m12 0) (m21 0) (m22 1) (m31 2) (m32 2)))))
          (Export 4)))
        (7 7)))"]
end
