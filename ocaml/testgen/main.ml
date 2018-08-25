open Core
open Implicit.Shape
open Implicit

let write_tests tests oc =
  let put_test shape oc =
    shape
    |> compile
    |> Option.sexp_of_t (Tuple2.sexp_of_t Command.sexp_of_t (Tuple2.sexp_of_t Float.sexp_of_t Float.sexp_of_t))
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

let expanded_poly = modulate 45.0 basic_poly

let overlay_test =
  let overlay d a b  =
    let grown_a = modulate d a in
    union [a ; subtract b grown_a]
  in
  let outline d a = subtract a @@ modulate (-.d) a  in
  let overlay_all d = List.reduce_exn ~f: (overlay d) in
  [
    (circle ~x: 13.0 ~y: 13.0 ~r: 10.0);
    (circle ~x: 23.0 ~y: 13.0 ~r: 10.0);
    (circle ~x: 13.0 ~y: 23.0 ~r: 10.0);
    (circle ~x: 23.0 ~y: 23.0 ~r: 10.0);
  ]
  |> List.rev
  |> List.map ~f:(outline 4.0)
  |> overlay_all 2.0

let rounded_rect ~x ~y ~w ~h ~r =
  modulate r @@ rect ~x:(x +. r) ~y:(y +. r) ~w:(w -. r *. 2.0) ~h:(h -. r *. 2.0)
let overlay_test_sub =
  subtract (rounded_rect ~r:10.0 ~x:1.0 ~y: 1.0 ~w:34.0 ~h:34.0) overlay_test

let scaled_circle =
  circle ~x:11.0 ~y:11.0 ~r:10.0
  |> scale ~dx: 2.0 ~dy: 1.0

let translated_circle =
  circle ~x:0.0 ~y:0.0 ~r:10.0
  |> translate ~dx: 10.0 ~dy: 10.0

let translated_and_scaled_circle =
  circle ~x:0.0 ~y:0.0 ~r:10.0
  |> scale ~dx: 2.0 ~dy: 1.0
  |> translate ~dx: 22.0 ~dy: 11.0

let inverted_circle =
  circle ~x:0.0 ~y:0.0 ~r:10.0
  |> not

let basic_rect=
  rect ~x:0.0 ~y:0.0 ~w:10.0 ~h:20.0

let ring =
  let larger  = circle ~x:0.0 ~y:0.0 ~r:10.0 in
  let smaller = larger |> modulate (-. 4.0) in
  subtract larger smaller

let easy_ring =
  let larger  =  circle ~x:10.0 ~y:10.0 ~r:10.0 in
  let smaller = circle ~x:10.0 ~y:10.0 ~r:6.0 in
  subtract larger smaller

let scaled_ring = ring |> scale ~dx:3.0 ~dy:3.0

let easy_scaled_ring = easy_ring |> scale ~dx:3.0 ~dy:3.0

let rr = rounded_rect ~r:10.0 ~x:1.0 ~y: 1.0 ~w:34.0 ~h:34.0

let rr_scaled = scale ~dx:3.0 ~dy:3.0 rr

let tests = [
  "rr", rr;
  "rr_scaled", rr_scaled;
  "ring", ring;
  "easy_ring", easy_ring;
  "scaled_ring", scaled_ring;
  "easy_scaled_ring", easy_scaled_ring;
  "basic_rect", basic_rect;
  "small_circle", small_circle;
  "displaced_circle", displaced_circle;
  "circles_union", circles_union;
  "circles_intersection", circles_intersection;
  "circles_intersection_freeze", circles_intersection_freeze;
  (*
  "basic_poly", basic_poly;
  "expanded_poly", expanded_poly;
  *)
  "overlay_test", overlay_test;
  "overlay_test_sub", overlay_test_sub |> scale ~dx:3.0 ~dy:3.0;
  "scaled_circle", scaled_circle;
  "translated_circle", translated_circle;
  "translated_and_scaled_circle", translated_and_scaled_circle;
  "inverted_circle", inverted_circle;
]

let () = Out_channel.with_file "../testsuite/tests.txt" ~f:(write_tests tests)
