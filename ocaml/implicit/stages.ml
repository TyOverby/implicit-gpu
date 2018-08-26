type user = Shape.justConcreteTerminals Shape.t [@@deriving sexp]
type expanded = Shape.allTerminals Shape.t [@@deriving sexp]
type simplified = user
