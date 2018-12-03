open Core
open Implicit
open Implicit.Creator

let put_test shape oc =
  let export_float_tuple = (Tuple2.sexp_of_t Float.sexp_of_t Float.sexp_of_t) in
  shape
  |> compile
  |> Option.sexp_of_t (Tuple2.sexp_of_t Command.sexp_of_t export_float_tuple)
  |> Sexp.to_string_hum
  |> Out_channel.output_string oc
;;

let disk ~ri ~ro =
  subtract
    (circle ~x:0.0 ~y:0.0 ~r:ro)
    (circle ~x:0.0 ~y:0.0 ~r:ri)

let notch ~ri ~ro ~iw ~r =
  let dist = (ro -. ri) in
  union [
    rotate_around ~x:0.0 ~y:ri ~r @@
    rect ~x:(-. iw /. 2.) ~y:(ri -. dist /. 2.0) ~w:iw ~h:(ro -. ri+. dist);
    rotate_around ~x:0.0 ~y:ri ~r:(-.r) @@
    rect ~x:(-. iw /. 2.) ~y:(ri -. dist /. 2.0) ~w:iw ~h:(ro -. ri+. dist);
  ]

let letter_to_rotation l =
  let idx = (Char.to_int l) - (Char.to_int 'a') in
  Float.pi -. 2.0 *. Float.pi *. (Float.of_int idx) /. 26.0

let ring ~ri ~ro ~iw ~r =
  subtract
    (disk ~ri ~ro)
    (notch ~ri ~ro ~iw ~r)


let gen_logo chars =
  List.fold chars ~init:(10.0, []) ~f:(fun (start, prev) chr ->
      let next = ring ~ri:start ~ro:(start +. 5.0)  ~iw:5.0 ~r:0.1 |> rotate ~r:(letter_to_rotation chr) in
      start +. 10.0,  next :: prev
    ) |> Tuple2.get2 |> union;;

let _js_logo = gen_logo ['j'; 's'; 'c'];;
let _js_logo = gen_logo ("abcdefghijklmnopqrstuvwxyz" |> String.to_list);;
let js_logo = gen_logo ("toverby" |> String.to_list);;
put_test (js_logo |> scale ~dx:2.0 ~dy:2.0 |> freeze |> modulate 1.0) Core.Out_channel.stdout
