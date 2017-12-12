import Impl from "react_like";
declare global {
    export var Circle: Impl.SFC<{ x: number, y: number, r: number }>;
    export var Rect: Impl.SFC<{ x: number, y: number, w: number, h: number }>;
    export var And: Impl.SFC;
    export var Or: Impl.SFC;
    export var Not: Impl.SFC;
    export var Break: Impl.SFC;
    export var Freeze: Impl.SFC;
    export var Translate: Impl.SFC<{ dx: number, dy: number }>;
    export var Modulate: Impl.SFC<{ by: number }>;
}
