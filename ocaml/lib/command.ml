type value =
  | Shape of Shape.allTerminals Shape.t
  | Polygon of Shape.polygon
  | Fetch of string

type command =
  | LetField of string * value
  | LetPoly of string * value
  | Concurrently of command list
  | Output of string * string
