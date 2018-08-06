open Core

type bbox = { x: float; y: float; w: float; h: float} [@@deriving sexp]
type point = { x: float; y: float } [@@deriving sexp]
type vec = { dx: float; dy: float } [@@deriving sexp]

(*
type matrix = {
  m11: float; m12: float;
  m21: float; m22: float;
  m31: float; m32: float;
} [@@deriving sexp]
*)

let point ~x ~y = { x; y }
let vec ~dx ~dy = { dx; dy }
let bbox ~x ~y ~w ~h = { x; y; w; h }

let point_sub a b = { dx = a.x -. b.x; dy = a.y -. b.y }

let v_add a b = { dx = a.dx +. b.dx; dy = a.dy +. b.dy }
let v_sub a b = { dx = a.dx -. b.dx; dy = a.dy -. b.dy }
let v_mul a b = { dx = a.dx *. b.dx; dy = a.dy *. b.dy }
let v_div a b = { dx = a.dx /. b.dx; dy = a.dy /. b.dy }

(*
let identity = {
  m11= 1.0; m12= 0.0;
  m21= 0.0; m22= 1.0;
  m31= 0.0; m32= 0.0;
}
*)


