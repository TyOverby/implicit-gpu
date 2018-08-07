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
