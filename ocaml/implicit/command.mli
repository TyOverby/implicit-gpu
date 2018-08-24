type t [@@deriving sexp]
val compile: Stages.user ->  (t * Bbox.bounding) option
