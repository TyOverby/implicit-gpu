open Core
open Implicit


let () =
  Shape.circle ~x:1.0 ~y:2.0 ~r:3.0
  |> Operations.simplify
  |> Shape.sexp_of_t
  |> Sexp.to_string_hum
  |> print_endline
