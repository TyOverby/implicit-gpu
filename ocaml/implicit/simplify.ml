open Core
open Shape

type simplified =
  | SNothing
  | SEverything
  | SShape of Stages.simplified

let rec expand: Stages.user -> Stages.expanded = function
  (* circle *)
  | Terminal Circle { r; _ } when r <= 0.0  -> Terminal Nothing
  | Terminal Circle c -> Terminal (Circle c)

  (* rect *)
  | Terminal Rect { w; h; _ } when w <= 0.0 || h <= 0.0 -> Terminal Nothing
  | Terminal Rect r -> Terminal (Rect r)

  (* poly *)
  | Terminal Poly { points = []; _ } -> Terminal Nothing
  | Terminal Poly p -> Terminal (Poly p)

  (* freeze *)
  | Freeze t -> (match expand t with
      | Terminal Nothing -> Terminal Nothing
      | Terminal Everything -> Terminal Everything
      | other -> Freeze other
    )

  (* not *)
  | Not Not x -> expand x
  | Not inner -> (
      match expand inner with
      | Terminal Nothing -> Terminal Everything
      | Terminal Everything -> Terminal Nothing
      | rest -> Not rest
    )

  (* modulate *)
  | Modulate(Modulate(target, a), b) -> Modulate(expand target, a +. b)
  | Modulate (target, how_much)  -> (
      match expand target with
      | Terminal Nothing -> Terminal Nothing
      | Terminal Everything -> Terminal Everything
      | target -> Modulate (target, how_much)
    )

  (* scale *)
  | Transform Scale(target, vec)  -> (
      match expand target with
      | Terminal Nothing -> Terminal Nothing
      | Terminal Everything -> Terminal Everything
      | target -> Transform (Scale(target, vec))
    )

  (* translate *)
  | Transform Translate(target, vec)  -> (
      match expand target with
      | Terminal Nothing -> Terminal Nothing
      | Terminal Everything -> Terminal Everything
      | target -> Transform (Translate(target, vec))
    )

  (* union *)
  | Union list -> let list = expand_all list in
    if List.exists list ~f:(phys_equal (Terminal Everything))
    then Terminal Everything
    else expand_easy_lists (Union (remove list (Terminal Nothing)))

  (* intersection *)
  | Intersection list -> let list = expand_all list in
    if List.exists list ~f:(phys_equal (Terminal Nothing))
    then Terminal Nothing
    else expand_easy_lists (Intersection (remove list (Terminal Everything)))

and expand_all = List.map ~f:expand
and expand_easy_lists = function
  | Intersection []  | Union [] -> Terminal Nothing
  | Intersection [a] | Union [a] -> a
  | other -> other
and remove list target =
  let filter a = phys_equal a target |> Core.not in
  List.filter ~f:filter list

let rec simplify (shape: Stages.user) : simplified = match expand shape with
  | Terminal Everything -> SEverything
  | Terminal Nothing -> SNothing
  | other -> SShape(simplify_bot other)
and simplify_bot (shape: Stages.expanded) : Stages.simplified = shape |> Shape.map (function
    | Everything -> failwith "Everything found after simplification"
    | Nothing -> failwith "Nothing found after simplification"
    | Circle c -> Circle c
    | Rect r -> Rect r
    | Poly p -> Poly p
  ) (function
    | Scale (target, v) -> Scale (simplify_bot target, v)
    | Translate (target, v) -> Translate (simplify_bot target, v))

module SimplifyExpectTests = struct
  let rec e_to_u (e: Stages.expanded) : Stages.user = match e with
    | Terminal Nothing -> Terminal (Circle {x = 0.0; y = 0.0; r = 0.0; mat = Matrix.id})
    | Terminal Everything -> Not (Terminal (Circle {x=0.0; y= 0.0; r= 0.0; mat= Matrix.id}))
    | Terminal Circle c -> Terminal (Circle c)
    | Terminal Rect r -> Terminal (Rect r)
    | Terminal Poly p -> Terminal (Poly p)
    | Freeze t -> Freeze (e_to_u t)
    | Not t -> Not (e_to_u t)
    | Modulate(t, k) -> Modulate(e_to_u t, k)
    | Transform Scale(t, v) -> Transform(Scale(e_to_u t, v))
    | Transform Translate(t, v) -> Transform(Translate(e_to_u t, v))
    | Union lst -> Union(List.map ~f:e_to_u lst)
    | Intersection lst -> Intersection (List.map ~f:e_to_u lst)

  let simplify_test a =
    a
    |> Sexp.of_string
    |> Stages.expanded_of_sexp
    |> e_to_u
    |> expand
    |> Stages.sexp_of_expanded
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
