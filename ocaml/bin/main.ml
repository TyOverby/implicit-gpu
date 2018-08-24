open Core
open Implicit


let () =
  Shape.circle ~x:1.0 ~y:2.0 ~r:3.0
  |> Operations.compile
  |> Option.sexp_of_t (Tuple2.sexp_of_t Command.sexp_of_t Bbox.sexp_of_t)
