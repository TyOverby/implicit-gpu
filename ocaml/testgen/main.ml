open Core
open Implicit.Creator
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

let rotated_square = rect ~x:0.0 ~y:0.0 ~w:10.0 ~h:10.0 |> rotate ~r:(3.14 /. 4.0)

let grid_of_circles =
  let z_to_100 =
    List.range ~stride:10 0 100
    |> List.map ~f:Float.of_int in
  let grid = begin
    let open List.Let_syntax in
    let%bind x = z_to_100 in
    let%bind y = z_to_100 in
    return (x, y)
  end in
  let circle_at (x, y) = circle ~x:x ~y:y ~r:5.0 in
  grid |> List.map ~f:circle_at |> union

let rotated_grid =
  grid_of_circles
  |> rotate ~r:(3.14 /. 4.0)
  |> scale ~dx:2.0 ~dy:2.0

let invert_area target area =
  let outside = subtract target area in
  let inside = intersection [not target; area] in
  union [outside; inside]

let inverted_grid =
  circle ~x:50.0 ~y:50.0 ~r:25.0
  |> invert_area grid_of_circles

let rotate_around_test =
  let c = circle ~x:10.0 ~y:10.0 ~r:5.0 in
  let r = rect ~x:0.0 ~y:0.0 ~w:10.0 ~h:10.0 in
  let r = r |> rotate_around ~x:10.0 ~y:10.0 ~r:(3.14 /. 4.0) in
  union [ c; r ]

let x: Point.t = {x=10.; y=11.}

let poly_from_points pts =
  pts
  |> List.bind ~f:(fun e -> [e; e])
  |> List.tl_exn
  |> (fun l -> List.append l @@ [List.hd_exn l])
  |> poly

let regular_poly points r =
  List.range 0 points
  |> List.rev
  |> List.map ~f:(fun idx -> (Float.of_int idx) /. (Float.of_int points) *. Float.pi *. 2.0)
  |> List.map ~f:(fun rad -> ({x = (Float.sin rad) *. r; y = (Float.cos rad) *. r}: Point.t))
  |> poly_from_points

let star tips out_r in_r =
  let points = tips * 2 in
  List.range 0 points
  |> List.rev
  |> List.map ~f:(fun idx -> (idx, (Float.of_int idx) /. (Float.of_int points) *. Float.pi *. 2.0))
  |> List.map ~f:(fun (idx, rad) ->
      let r = if idx % 2 = 0 then out_r else in_r in
      ({x = (Float.sin rad) *. r; y = (Float.cos rad) *. r}: Point.t))
  |> poly_from_points

let tri = regular_poly 3 10.0
let quad = regular_poly 4 10.0
let pent = regular_poly 5 10.0
let hex = regular_poly 6 10.0

let three_star = star 3 20.0 5.0
let four_star = star 4 20.0 10.0
let five_star = star 5 20.0 10.0
let six_star = star 6 20.0 10.0

let huge_star = star 10 200.0 100.0

let tests = [
  "huge_star_BAD", huge_star;
  "three_star", three_star;
  "four_star", four_star;
  "five_star", five_star;
  "six_star", six_star;
  "tri", tri;
  "quad", quad;
  "pent", pent;
  "hex", hex;
  "rotate_around_test", rotate_around_test;
  "inverted_grid", inverted_grid;
  "grid_of_circles", grid_of_circles;
  "rotated_grid", rotated_grid;
  "rotated_square", rotated_square;
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
  "basic_poly", basic_poly;
  "expanded_poly", expanded_poly;
  "overlay_test", overlay_test;
  "overlay_test_sub", overlay_test_sub |> scale ~dx:3.0 ~dy:3.0;
  "scaled_circle", scaled_circle;
  "translated_circle", translated_circle;
  "translated_and_scaled_circle", translated_and_scaled_circle;
  "inverted_circle", inverted_circle;
]

let () = Out_channel.with_file "../testsuite/tests.txt" ~f:(write_tests tests)
