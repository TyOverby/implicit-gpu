type shape [@@deriving sexp]
type point [@@deriving sexp]
type bbox [@@deriving sexp]

val circle: x:float ->  y:float ->  r:float -> shape
val rect: x:float ->  y:float ->  w:float -> h:float -> shape
val poly: point list -> shape
val not: shape -> shape
val union: shape list -> shape
val intersection: shape list -> shape

val simplify: shape -> shape
val shape_eq: shape -> shape -> bool
