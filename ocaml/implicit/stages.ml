type user = (Shape.justConcreteTerminals, Shape.justConcreteTerminals Shape.allTransforms) Shape.t [@@deriving sexp]
type expanded = (Shape.allTerminals, Shape.allTerminals Shape.allTransforms) Shape.t [@@deriving sexp]
type simplified = user [@@deriving sexp]
type propagated = (Shape.justConcreteTerminals, Core.Nothing.t) Shape.t [@@deriving sexp]
