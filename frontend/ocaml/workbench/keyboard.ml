open Core
open Implicit
open Implicit.Creator
open Yojson

let put_test shape oc =
  let export_float_tuple = (Tuple2.sexp_of_t Float.sexp_of_t Float.sexp_of_t) in
  shape
  |> compile
  |> Option.sexp_of_t (Tuple2.sexp_of_t Command.sexp_of_t export_float_tuple)
  |> Sexp.to_string_hum
  |> Out_channel.output_string oc
;;

(* Default 60% from http://www.keyboard-layout-editor.com/ *)
let keyboard_file = {|
[
    ["~\n`","!\n1","@\n2","#\n3","$\n4","%\n5","^\n6","&\n7","*\n8","(\n9",")\n0","_\n-","+\n=",{w:2},"Backspace"],
    [{w:1.5},"Tab","Q","W","E","R","T","Y","U","I","O","P","{\n[","}\n]",{w:1.5},"|\n\\"],
    [{w:1.75},"Caps Lock","A","S","D","F","G","H","J","K","L",":\n;","\"\n'",{w:2.25},"Enter"],
    [{w:2.25},"Shift","Z","X","C","V","B","N","M","<\n,",">\n.","?\n/",{w:2.75},"Shift"],
    [{w:1.25},"Ctrl",{w:1.25},"Win",{w:1.25},"Alt",{a:7,w:6.25},"",{a:4,w:1.25},"Alt",{w:1.25},"Win",{w:1.25},"Menu",{w:1.25},"Ctrl"]
]
|};;

let keyboard_json = Yojson.Basic.from_string keyboard_file;;
let sanatize (rows: Basic.json) =
  let sanatize_row (row: Basic.json) =
    let rec s_row list =
      match list with
      | [] -> []
      | (`Assoc [("a", _); ("w", `Int i)]) :: (`String _) :: rest -> (Float.of_int i) :: (s_row rest)
      | (`Assoc [("a", _); ("w", `Float i)]) :: (`String _) :: rest -> i :: (s_row rest)
      | (`Assoc [("w", `Int i)]) :: (`String _) :: rest -> (Float.of_int i) :: (s_row rest)
      | (`Assoc [("w", `Float i)]) :: (`String _) :: rest -> i :: (s_row rest)
      | `String _ :: rest -> 1.0 :: (s_row rest)
      | _ -> failwith "oh no"
    in
    match row with
    | `List l -> s_row l
    | _ -> failwith "unexpected row"
  in
  match rows with
  | `List l -> List.map l ~f:sanatize_row
  | _ -> failwith "unexpected rows"
;;

let s = sanatize keyboard_json ;;

let default_size = 10.0;;
let default_gap = 3.0;;

let layout_row y keys =
  let y = (Float.of_int y) *. (default_gap +. default_size) in
  let fold_key (cur_width, list) w =
    let shape =
      rect
        ~x:(cur_width +. (w *. default_size /. 2.0))
        ~y
        ~w:default_size
        ~h:default_size
    in
    (cur_width +. (w *. default_size) +. default_gap, shape :: list)
  in
  let (_, list) =  List.fold keys ~init:(0.0, []) ~f:fold_key in
  list

let shapes = List.mapi s ~f:layout_row |> List.concat;;

let cutouts = union shapes |> scale ~dx:3.0 ~dy:3.0 ;;
let keyboard = subtract (cutouts |> modulate 7.0) cutouts ;;

put_test keyboard Core.Out_channel.stdout
