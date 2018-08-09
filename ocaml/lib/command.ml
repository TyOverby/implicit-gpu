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

type command =
  | Concurrently of command list
  | Serially of command list
  | Define of id * value
  | Freeze of id * id
  | Export of id
[@@deriving sexp]

type id_gen = {
  next: int Ref.t
}

let get_id gen =
  let next = !(gen.next) in
  gen.next := (next + 1);
  next

let compile shape =
  let values = ref [] in
  let id_gen = { next = ref 0 } in
  let result_shape =
    shape |> Shape.map
      (function
        | Poly p ->
          let id = get_id id_gen in
          values := ((id, Polygon p) :: (!values));
          Field id
        | Circle c -> Circle c
        | Rect r -> Rect r
      )
      Fn.id in
  let values = !values in
  let commands = Serially (List.map values ~f:(fun (x, y) -> Define(x, y))) in
  let last_id = get_id id_gen in
  let last = Define(last_id, BasicShape result_shape) in
  Serially [commands; last; Export last_id]
