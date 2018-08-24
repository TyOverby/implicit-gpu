open Core
open Shape

type id = int
[@@deriving sexp]

type basicTerminals =
  | Circle of Shape.circle
  | Rect of Shape.rect
  | Field of id
[@@deriving sexp]

type value =
  | BasicShape of (basicTerminals, Nothing.t) Shape.t
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
  let values = ref [] in
  let id_gen = { next = ref 0 } in
  let result_shape =
    let map_terminal = function
      | Poly p ->
        let id = get_id id_gen in
        values := ((id, Polygon p) :: (!values));
        Field id
      | Circle c -> Circle c
      | Rect r -> Rect r in
    let map_transform = Fn.id in
    shape |> Shape.map map_terminal map_transform in
  let values = !values in
  let commands = List.map values ~f:(fun (x, y) -> Define(x, y)) in
  let last_id = get_id id_gen in
  let last = Define(last_id, BasicShape result_shape) in
  if List.is_empty commands
  then Serially [last; Export last_id]
  else Serially [Serially commands; last; Export last_id]

let compile shape =
  let simplified = Simplify.simplify shape in
  match simplified with
  | Simplify.SShape shape ->
    let propagated = Prop.remove_transformations shape in
    let bb = Compute_bb.compute_bounding_box propagated in
    let compiled = compile_commands propagated in
    Some (compiled, bb)
  | Simplify.SNothing
  | Simplify.SEverything -> None

module Command_Tests = struct
  let test_shape_compile shape =
    shape
    |> Sexp.of_string
    |> Stages.user_of_sexp
    |> compile
    |> Option.sexp_of_t (Tuple2.sexp_of_t sexp_of_t Bbox.sexp_of_bounding)
    |> Sexp.to_string_hum
    |> print_endline

  let%expect_test _ =
    test_shape_compile "(Terminal (Circle ((x 0) (y 0) (r 0))))";
    [%expect "()"]

  let%expect_test _ =
    test_shape_compile "(Terminal (Circle ((x 0) (y 0) (r 1))))";
    [%expect "
      (((Serially
         ((Define 0 (BasicShape (Terminal (Circle ((x 0) (y 0) (r 1))))))
          (Export 0)))
        (Positive ((x -1) (y -1) (w 2) (h 2)))))"]

  let%expect_test _ =
    test_shape_compile "(Not (Terminal (Circle ((x 0) (y 0) (r 1)))))";
    [%expect "
      (((Serially
         ((Define 0 (BasicShape (Not (Terminal (Circle ((x 0) (y 0) (r 1)))))))
          (Export 0)))
        (Negative ((x -1) (y -1) (w 2) (h 2)))))"]

  let%expect_test _ =
    test_shape_compile "(Terminal (Circle ((x 1) (y 1) (r 1))))";
    [%expect "
      (((Serially
         ((Define 0 (BasicShape (Terminal (Circle ((x 1) (y 1) (r 1))))))
          (Export 0)))
        (Positive ((x 0) (y 0) (w 2) (h 2)))))"]

  let%expect_test _ =
    test_shape_compile "(Union ((Terminal (Circle ((x 1) (y 1) (r 1))))
                                (Terminal (Circle ((x 2) (y 2) (r 1))))))";
    [%expect "
      (((Serially
         ((Define 0
           (BasicShape
            (Union
             ((Terminal (Circle ((x 1) (y 1) (r 1))))
              (Terminal (Circle ((x 2) (y 2) (r 1))))))))
          (Export 0)))
        (Positive ((x 0) (y 0) (w 3) (h 3)))))"]

  let%expect_test _ =
    test_shape_compile "(Intersection ((Terminal (Circle ((x 0) (y 0) (r 1))))
                                       (Terminal (Rect   ((x 0) (y 0) (w 1) (h 1))))))";
    [%expect "
      (((Serially
         ((Define 0
           (BasicShape
            (Intersection
             ((Terminal (Circle ((x 0) (y 0) (r 1))))
              (Terminal (Rect ((x 0) (y 0) (w 1) (h 1))))))))
          (Export 0)))
        (Positive ((x 0) (y 0) (w 1) (h 1)))))"]
end
