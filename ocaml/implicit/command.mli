type t [@@deriving sexp]
val compile: Stages.propagated -> t
