module Shape = struct
  include Shape
end
module Point = struct
  include Point
end
module Bbox = struct
  include Bbox
end
module Operations = struct
  let simplify = Simplify.simplify
  let remove_transformations = Prop.remove_transformations
end

module Ops = struct
  let ( && ) a b = Shape.intersection [ a; b; ]
  let ( || ) a b = Shape.union [ a; b; ]
  let ( -- ) a b = Shape.intersection [ a; Shape.not b; ]
  let ( ++ ) a v = Shape.Modulate (a, v)
  let ( !! ) t = Shape.not t
end
