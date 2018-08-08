open Core
open Shape

type simplified =
  | SNothing
  | SEverything
  | SShape of Shape.justConcreteTerminals Shape.allTShape


let rec simplify: Shape.allTerminals Shape.allTShape -> Shape.allTerminals Shape.allTShape = function
  (* circle *)
  | Terminal Circle { r; _ } when r <= 0.0  -> Terminal Nothing
  | Terminal Circle _ as a -> a

  (* rect *)
  | Terminal Rect { w; h; _ } when w <= 0.0 || h <= 0.0 -> Terminal Nothing
  | Terminal Rect _ as a -> a

  (* poly *)
  | Terminal Poly { points = []; _ } -> Terminal Nothing
  | Terminal Poly _ as a -> a

  (* everything and nothing *)
  | Terminal Everything -> Terminal Everything
  | Terminal Nothing -> Terminal Nothing

  (* not *)
  | Not Not x -> simplify x
  | Not inner -> (
      match simplify inner with
      | Terminal Nothing -> Terminal Everything
      | Terminal Everything -> Terminal Nothing
      | rest -> Not rest
    )

  (* modulate *)
  | Modulate(Modulate(target, a), b) -> Modulate(simplify target, a +. b)
  | Modulate (target, how_much)  -> (
      match simplify target with
      | Terminal Nothing -> Terminal Nothing
      | Terminal Everything -> Terminal Everything
      | target -> Modulate (target, how_much)
    )

  (* scale *)
  | Transform Scale(target, vec)  -> (
      match simplify target with
      | Terminal Nothing -> Terminal Nothing
      | Terminal Everything -> Terminal Everything
      | target -> Transform (Scale(target, vec))
    )

  (* translate *)
  | Transform Translate(target, vec)  -> (
      match simplify target with
      | Terminal Nothing -> Terminal Nothing
      | Terminal Everything -> Terminal Everything
      | target -> Transform (Translate(target, vec))
    )

  (* union *)
  | Union list -> let list = simplify_all list in
    if List.exists list ~f:(phys_equal (Terminal Everything))
    then Terminal Everything
    else simplify_easy_lists (Union (remove list (Terminal Nothing)))

  (* intersection *)
  | Intersection list -> let list = simplify_all list in
    if List.exists list ~f:(phys_equal (Terminal Nothing))
    then Terminal Nothing
    else simplify_easy_lists (Intersection (remove list (Terminal Everything)))

and simplify_all = List.map ~f:simplify
and simplify_easy_lists = function
  | Intersection []  | Union [] -> Terminal Nothing
  | Intersection [a] | Union [a] -> a
  | other -> other
and remove list target =
  let filter a = phys_equal a target |> Core.not in
  List.filter ~f:filter list

let rec simplify_top = function
  | Terminal Everything -> SEverything
  | Terminal Nothing -> SNothing
  | other -> SShape (simplify_bot other)
and simplify_bot shape : Shape.justConcreteTerminals Shape.allTShape = shape |> Shape.map (function
    | Everything -> failwith "Everything found after simplification"
    | Nothing -> failwith "Nothing found after simplification"
    | Circle c -> Circle c
    | Rect r -> Rect r
    | Poly p -> Poly p
  ) (function
    | Scale (target, v) -> Scale (simplify_bot target, v)
    | Translate (target, v) -> Translate (simplify_bot target, v))

module SimplifyExpectTests = struct
  let simplify_test a =
    a
    |> Sexp.of_string
    |> Shape.allTShape_of_sexp Shape.allTerminals_of_sexp
    |> simplify
    |> Shape.sexp_of_allTShape Shape.sexp_of_allTerminals
    |> Sexp.to_string_hum
    |> print_endline

  let%expect_test _ =
    simplify_test "(Terminal Nothing)";
    [%expect "(Terminal Nothing)"]

  let%expect_test _ =
    simplify_test "(Terminal Everything)";
    [%expect "(Terminal Everything)"]

  let%expect_test _ =
    simplify_test "(Terminal (Circle ((x 1) (y 1) (r 1))))";
    [%expect "(Terminal (Circle ((x 1) (y 1) (r 1))))"]

  let%expect_test _ =
    simplify_test "(Terminal (Circle ((x 1) (y 1) (r 0))))";
    [%expect "(Terminal Nothing)"]

  let%expect_test _ =
    simplify_test "(Not (Terminal Everything))";
    [%expect "(Terminal Nothing)"]

  let%expect_test _ =
    simplify_test "(Not (Terminal Nothing))";
    [%expect "(Terminal Everything)"]

  let%expect_test _ =
    simplify_test "(Not (Not (Terminal (Circle ((x 1) (y 1) (r 1))))))";
    [%expect "(Terminal (Circle ((x 1) (y 1) (r 1))))"]

  let%expect_test _ =
    simplify_test "(Terminal (Poly ((points ()))))";
    [%expect "(Terminal Nothing)"]

  let%expect_test _ =
    simplify_test "(Union ((Terminal Everything) (Terminal Nothing)))";
    [%expect "(Terminal Everything)"]

  let%expect_test _ =
    simplify_test "(Intersection ((Terminal Everything) (Terminal Nothing)))";
    [%expect "(Terminal Nothing)"]

  let%expect_test _ =
    simplify_test "(Intersection ((Terminal (Circle ((x 10) (y 10) (r 10))))))";
    [%expect "(Terminal (Circle ((x 10) (y 10) (r 10))))"]

  let%expect_test _ =
    simplify_test "(Union ((Terminal (Circle ((x 10) (y 10) (r 10))))))";
    [%expect "(Terminal (Circle ((x 10) (y 10) (r 10))))"]

  let%expect_test _ =
    simplify_test "(Intersection ((Terminal (Circle ((x 20) (y 20) (r 20)))) (Terminal (Circle ((x 10) (y 10) (r 10))))))";
    [%expect "
      (Intersection
       ((Terminal (Circle ((x 20) (y 20) (r 20))))
        (Terminal (Circle ((x 10) (y 10) (r 10))))))"]

  let%expect_test _ =
    simplify_test "(Union ((Terminal (Circle ((x 20) (y 20) (r 20)))) (Terminal (Circle ((x 10) (y 10) (r 10))))))";
    [%expect "
      (Union
       ((Terminal (Circle ((x 20) (y 20) (r 20))))
        (Terminal (Circle ((x 10) (y 10) (r 10))))))"]

  let%expect_test _ =
    simplify_test "(Union ((Terminal Nothing) (Terminal (Circle ((x 20) (y 20) (r 20)))) (Terminal (Circle ((x 10) (y 10) (r 10))))))";
    [%expect "
      (Union
       ((Terminal (Circle ((x 20) (y 20) (r 20))))
        (Terminal (Circle ((x 10) (y 10) (r 10))))))"]

  let%expect_test _ =
    simplify_test "(Intersection ((Terminal Everything) (Terminal (Circle ((x 20) (y 20) (r 20)))) (Terminal (Circle ((x 10) (y 10) (r 10))))))";
    [%expect "
      (Intersection
       ((Terminal (Circle ((x 20) (y 20) (r 20))))
        (Terminal (Circle ((x 10) (y 10) (r 10))))))"]

  let%expect_test _ =
    simplify_test "(Union ((Terminal Everything) (Terminal (Circle ((x 10) (y 10) (r 10))))))";
    [%expect "(Terminal Everything)"]

  let%expect_test _ =
    simplify_test "(Union ((Terminal Nothing) (Terminal (Circle ((x 10) (y 10) (r 10))))))";
    [%expect "(Terminal (Circle ((x 10) (y 10) (r 10))))"]
end
