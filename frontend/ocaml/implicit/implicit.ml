module Creator = Creator
module Point = Point
module Command = Command
module Bbox = Bbox

module Ops = struct
  let ( && ) a b = Creator.intersection [ a; b; ]
  let ( || ) a b = Creator.union [ a; b; ]
  let ( -- ) a b = Creator.intersection [ a; Creator.not b; ]
  let ( ++ ) a v = Creator.modulate a v
  let ( !! ) t = Creator.not t
end

let compile = Command.compile
