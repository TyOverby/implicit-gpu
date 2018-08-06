open Core
open Shape

let () =
  print_endline "Hello, world!";
  circle ~x:1.0 ~y:2.0 ~r:3.0
  |> simplify
  |> sexp_of_shape
  |> Sexp.to_string_hum
  |> print_endline
