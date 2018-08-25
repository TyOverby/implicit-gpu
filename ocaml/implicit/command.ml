open Core
open Shape

type id = int
[@@deriving sexp]

type basicTerminals =
  | Circle of Shape.circle
  | Rect of Shape.rect
  | Field of id
[@@deriving sexp]

type exportShape = (basicTerminals, Nothing.t) Shape.t
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

let compile_commands shape =
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
    let map_transform = Fn.id in
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
    shape |> Shape.visit map_shape map_terminal map_transform in
  let last_id = get_id id_gen in
  let last = Define(last_id, BasicShape result_shape) in
  let commands = !commands in
  if List.is_empty commands
  then Serially [last; Export last_id]
  else Serially [Serially commands; last; Export last_id]

let compile shape =
  let simplified = Simplify.simplify shape in
  match simplified with
  | Simplify.SShape shape ->
    let propagated = Prop.remove_transformations shape in
    (match Compute_bb.compute_bounding_box propagated with
     | Everything | Nothing -> failwith "unreachable"
     | Positive bb ->
       let bb = Bbox.grow_by 0.1 bb in
       let simplified = Transform (Translate(shape, {dx = -. bb.x; dy = -. bb.y})) in
       let propagated = Prop.remove_transformations simplified in
       let compiled = compile_commands propagated in
       Some (compiled, (bb.w, bb.h))
     | Negative bb ->
       let bb = Bbox.grow_by 0.1 bb in
       let box_shape: Stages.user = Terminal (Rect {x = bb.x; y = bb.y; w = bb.w; h=bb.h; mat = Matrix.id}) in
       let bb = Bbox.grow_by 0.1 bb in
       let simplified = Intersection [box_shape; shape] in
       let simplified = Transform (Translate(simplified, {dx = -. bb.x; dy = -. bb.y})) in
       let propagated = Prop.remove_transformations simplified in
       let compiled = compile_commands propagated in
       Some (compiled, (bb.w, bb.h)))
  | Simplify.SNothing
  | Simplify.SEverything -> None

module Command_Tests = struct
  let test_shape_compile shape =
    shape
    |> Sexp.of_string
    |> Stages.user_of_sexp
    |> compile
    |> Option.sexp_of_t (Tuple2.sexp_of_t sexp_of_t (Tuple2.sexp_of_t Float.sexp_of_t Float.sexp_of_t))
    |> Sexp.to_string_hum
    |> print_endline

  let%expect_test _ =
    test_shape_compile "(Terminal (Circle ((x 0) (y 0) (r 0))))";
    [%expect "()"]

  let%expect_test _ =
    test_shape_compile "(Terminal (Circle ((x 0) (y 0) (r 1))))";
    [%expect "
      (((Serially
         ((Define 0
           (BasicShape
            (Terminal
             (Circle
              ((x 0) (y 0) (r 1)
               (mat ((m11 1) (m12 0) (m21 0) (m22 1) (m31 3) (m32 3))))))))
          (Export 0)))
        (6 6)))"]

  let%expect_test _ =
    test_shape_compile "(Not (Terminal (Circle ((x 0) (y 0) (r 1)))))";
    [%expect "
      (((Serially
         ((Define 0
           (BasicShape
            (Intersection
             ((Terminal
               (Rect
                ((x -3) (y -3) (w 6) (h 6)
                 (mat ((m11 1) (m12 0) (m21 0) (m22 1) (m31 5) (m32 5))))))
              (Not
               (Terminal
                (Circle
                 ((x 0) (y 0) (r 1)
                  (mat ((m11 1) (m12 0) (m21 0) (m22 1) (m31 5) (m32 5)))))))))))
          (Export 0)))
        (10 10)))"]

  let%expect_test _ =
    test_shape_compile "(Terminal (Circle ((x 1) (y 1) (r 1))))";
    [%expect "
      (((Serially
         ((Define 0
           (BasicShape
            (Terminal
             (Circle
              ((x 1) (y 1) (r 1)
               (mat ((m11 1) (m12 0) (m21 0) (m22 1) (m31 2) (m32 2))))))))
          (Export 0)))
        (6 6)))"]

  let%expect_test _ =
    test_shape_compile "(Union ((Terminal (Circle ((x 1) (y 1) (r 1))))
                                (Terminal (Circle ((x 2) (y 2) (r 1))))))";
    [%expect "
      (((Serially
         ((Define 0
           (BasicShape
            (Union
             ((Terminal
               (Circle
                ((x 1) (y 1) (r 1)
                 (mat ((m11 1) (m12 0) (m21 0) (m22 1) (m31 2) (m32 2))))))
              (Terminal
               (Circle
                ((x 2) (y 2) (r 1)
                 (mat ((m11 1) (m12 0) (m21 0) (m22 1) (m31 2) (m32 2))))))))))
          (Export 0)))
        (7 7)))"]

  let%expect_test _ =
    test_shape_compile "(Intersection ((Terminal (Circle ((x 0) (y 0) (r 1))))
                                       (Terminal (Rect   ((x 0) (y 0) (w 1) (h 1))))))";
    [%expect "
      (((Serially
         ((Define 0
           (BasicShape
            (Intersection
             ((Terminal
               (Circle
                ((x 0) (y 0) (r 1)
                 (mat ((m11 1) (m12 0) (m21 0) (m22 1) (m31 2) (m32 2))))))
              (Terminal
               (Rect
                ((x 0) (y 0) (w 1) (h 1)
                 (mat ((m11 1) (m12 0) (m21 0) (m22 1) (m31 2) (m32 2))))))))))
          (Export 0)))
        (5 5)))"]

  let%expect_test _ =
    test_shape_compile "(Intersection ((Freeze (Terminal (Circle ((x 0) (y 0) (r 1)))))
                                       (Freeze (Terminal (Rect   ((x 0) (y 0) (w 1) (h 1)))))))";
    [%expect "
      (((Serially
         ((Serially
           ((Define 0
             (BasicShape
              (Terminal
               (Circle
                ((x 0) (y 0) (r 1)
                 (mat ((m11 1) (m12 0) (m21 0) (m22 1) (m31 2) (m32 2))))))))
            (Freeze (target 0) (id 1))
            (Define 2
             (BasicShape
              (Terminal
               (Rect
                ((x 0) (y 0) (w 1) (h 1)
                 (mat ((m11 1) (m12 0) (m21 0) (m22 1) (m31 2) (m32 2))))))))
            (Freeze (target 2) (id 3))))
          (Define 4
           (BasicShape (Intersection ((Terminal (Field 1)) (Terminal (Field 3))))))
          (Export 4)))
        (5 5)))"]
end
