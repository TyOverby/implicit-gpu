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

let circle ~r = circle ~x:0.0 ~y:0.0 ~r;;

let disk ~ri ~ro =
  subtract (circle ~r:ro) (circle ~r:ri)

let notch ~ri ~ro ~iw ~r =
  let dist = (ro -. ri) in
  let half_notch r = rotate_around ~x:0.0 ~y:ri ~r
      (rect ~x:(-. iw /. 2.) ~y:(ri -. dist /. 2.0)
            ~w:iw ~h:(ro -. ri+. dist)) in
  union [ half_notch r ; half_notch (-. r) ]

let ring ~ri ~ro ~iw ~r =
  subtract (disk ~ri ~ro) (notch ~ri ~ro ~iw ~r)

let letter_to_rotation l =
  let idx = (Char.to_int l) - (Char.to_int 'a') in
  Float.pi -. 2.0 *. Float.pi *. (Float.of_int idx) /. 26.0

let gen_logo chars =
  let initial_radius, ring_thickness, space_between = 17.5, 4.0, 8.5 in
  let gap_width, gap_angle = 9.0, 0.1 in
  let _, logo = List.fold chars ~init:(initial_radius, []) ~f:(fun (start, prev) chr ->
      let next = ring
          ~ri:start ~ro:(start +. ring_thickness)
          ~iw:gap_width ~r:gap_angle in
      start +. space_between, (rotate ~r:(letter_to_rotation chr) next) :: prev
    )
  in logo |> union

let js_logo = gen_logo ['j'; 's'; 'c'];;

let _js_logo = gen_logo ("abcdefghijklmnopqrstuvwxyz" |> String.to_list);;
let _js_logo = gen_logo ("tanner" |> String.to_list);;
let scale_factor = 4.0;;
let rounding_factor = 2.5;;
put_test (js_logo |> scale ~dx:scale_factor ~dy:scale_factor |> freeze |> modulate rounding_factor) Core.Out_channel.stdout
