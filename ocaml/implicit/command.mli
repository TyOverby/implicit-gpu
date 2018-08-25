type t [@@deriving sexp]
val compile: Stages.user ->  (t * (float * float)) option
