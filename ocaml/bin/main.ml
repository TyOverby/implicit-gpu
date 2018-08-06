open Core

let () =
  print_endline "Hello, world!";
  Math.Circle {x= 1.0; y= 2.0; r= 0.0}
  |> Math.simplify
  |> Math.sexp_of_shape
  |> Sexp.to_string_hum
  |> print_endline
