open Core
open Implicit.Shape
open Implicit

let write_tests tests oc =
  let put_test shape oc =
    shape
    |> compile
    |> Option.sexp_of_t (Tuple2.sexp_of_t Command.sexp_of_t Bbox.sexp_of_bounding)
    |> Sexp.to_string_hum
    |> Out_channel.output_string oc
  in
  let each_test name shape =
    Out_channel.with_file (sprintf "../testsuite/tests/%s.shape" name) ~f:(put_test shape);
    ()
  in
  let test_names = List.map tests ~f:Tuple2.get1 in
  ignore (Out_channel.output_lines oc test_names);
  List.iter tests ~f:(Tuple2.uncurry each_test)


let small_circle = circle ~x:11.0 ~y:11.0 ~r:10.0

let displaced_circle = circle ~x:20.0 ~y:20.0 ~r:15.0

let circles_union = union [
    circle ~x:11.0 ~y: 11.0 ~r: 10.0;
    circle ~x:21.0 ~y: 21.0 ~r: 10.0;
  ]

let circles_intersection = intersection [
    circle ~x:11.0 ~y: 11.0 ~r: 10.0;
    circle ~x:21.0 ~y: 21.0 ~r: 10.0;
  ]

let circles_intersection_freeze = freeze @@ intersection [
    circle ~x:11.0 ~y: 11.0 ~r: 10.0;
    circle ~x:21.0 ~y: 21.0 ~r: 10.0;
  ]

let basic_poly = poly [
    { x = 50.0;  y = 50.0  };
    { x = 200.0; y = 50.0  };
    { x = 200.0; y = 50.0  };
    { x = 250.0; y = 200.0 };
    { x = 250.0; y = 200.0 };
    { x = 150.0; y = 100.0 };
    { x = 150.0; y = 100.0 };
    { x = 50.0;  y = 50.0  };
  ]

let expanded_poly = modulate basic_poly 45.0

let tests = [
  "small_circle", small_circle;
  "displaced_circle", displaced_circle;
  "circles_union", circles_union;
  "circles_intersection", circles_intersection;
  "circles_intersection_freeze", circles_intersection_freeze;
  "basic_poly", basic_poly;
  "expanded_poly", expanded_poly;
]

let () = Out_channel.with_file "../testsuite/tests.txt" ~f:(write_tests tests)
