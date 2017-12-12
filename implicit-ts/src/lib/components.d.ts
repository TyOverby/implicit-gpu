/**
 * Creates a circle centered at `(x, y)` with radius `r`.
 */
declare var Circle: Impl.SFC<{ x: number, y: number, r: number }>;

/**
 * Creates a rectangle with a top left corner at `(x, y)`, and a
 * width and height of `w` and `h` respectively.
 */
declare var Rect: Impl.SFC<{ x: number, y: number, w: number, h: number }>;

/**
 * Creates a new shape out of the parts shared by all of its children.
 *
 * ![Image](https://upload.wikimedia.org/wikipedia/commons/9/99/Venn0001.svg)
 */
declare var And: Impl.SFC;

/**
 * Creates a new shape by combining all of its children.
 */
declare var Or: Impl.SFC;

/**
 * Creates a new shape that is the inverse of its grouped children.
 */
declare var Not: Impl.SFC;

/**
 * TODO: Document
 */
declare var Break: Impl.SFC;

/**
 * TODO: Document
 */
declare var Freeze: Impl.SFC;

/**
 * Moves its child nodes by `(dx, dy)`.
 */
declare var Translate: Impl.SFC<{ dx: number, dy: number }>;
/**
 * Grows the shape by `by`.
 */
declare var Modulate: Impl.SFC<{ by: number }>;
