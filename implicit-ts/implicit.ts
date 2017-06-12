export type Implicit = Circle | Rect | And | Or | Not | Modulate | Break | Freeze;

export interface Scene {
    figures: Figure[],
}

export interface Figure {
    shapes: Shape[]
}

export interface Shape {
    implicit: Implicit,
    color: [number, number, number],
    draw_style: "filled" | "line"
}

export interface Circle {
    kind: "circle",
    x: number,
    y: number,
    r: number,
}

export interface Rect {
    kind: "rect",
    x: number,
    y: number,
    w: number,
    h: number,
}

export interface And {
    kind: "and",
    children: Implicit[],
}

export interface Or {
    kind: "or",
    children: Implicit[],
}

export interface Not {
    kind: "not",
    target: Implicit,
}

// TODO: polygons

export interface Modulate {
    kind: "modulate",
    how_much: number,
    target: Implicit,
}

export interface Break {
    kind: "break",
    target: Implicit,
}

export interface Freeze {
    kind: "freeze",
    target: Implicit,
}

export function circle(x: number, y: number, r: number): Circle {
    return {
        kind: "circle",
        x: x,
        y: y,
        r: r
    };
}

export function rect(x: number, y: number, w: number, h: number): Rect {
    return {
        kind: "rect",
        x: x,
        y: y,
        w: w,
        h: h
    };
}

export function and(...children: Implicit[]): And {
    return {
        kind: "and",
        children: children
    };
}

export function or(...children: Implicit[]): Or {
    return {
        kind: "or",
        children: children
    };
}

export function not(target: Implicit): Not {
    return {
        kind: "not",
        target: target
    };
}

export function modulate(how_much: number, target: Implicit): Modulate {
    return {
        kind: "modulate",
        how_much: how_much,
        target: target,
    };
}

export function break_here(target: Implicit): Break {
    return {
        kind: "break",
        target: target,
    };
}

export function freeze(target: Implicit): Freeze {
    return {
        kind: "freeze",
        target: target,
    };
}
