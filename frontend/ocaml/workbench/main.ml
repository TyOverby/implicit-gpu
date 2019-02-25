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

type point = {x: float; y: float}

let width = 10.0 ;;
let gap = 1.0;;

let count = 5;;

let corner {x; y} = {
  x = x *. width +. x *. gap;
  y = y *. width +. y *. gap
}

let center {x; y} =
  let {x; y} = corner {x; y} in
  let w2 = width /. 2.0 in
  {
    x = x +. w2;
    y = y +. w2;
  }

let progress y =
  y /. (Float.of_int count)

let rectangles =
  let rotation {x=_; y}  = (Float.pi /. 4.0) *. (progress y) in

  let x_y =
    let open List.Let_syntax in
    let%bind x = List.range 0 count in
    let%bind y = List.range 0 count in
    let x, y = Float.of_int x, Float.of_int y in
    return {x; y}
  in
  x_y
  |> List.map ~f:(fun pt ->
      let {x=xc; y=yc} = corner pt in
      let {x=xm; y=ym} = center pt in
      let r = rotation pt in
      rect ~x:xc ~y:yc ~w:width ~h:width |> rotate_around ~x:xm ~y:ym ~r)
  |> union
  (*|> scale ~dx:10.0 ~dy:10.0*)
;;
put_test rectangles Core.stdout
