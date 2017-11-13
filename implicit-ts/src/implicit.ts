export type Implicit = Circle | Rect | And | Or | Not | Modulate | Break | Freeze | Translate;

export interface Scene {
    figures: Figure[],
    unit: string,
    simplify: boolean,
}

export interface Figure {
    shapes: Shape[]
}

export type DrawMode = "filled" | { "line": "solid" }

export interface Shape {
    implicit: Implicit,
    color: [number, number, number],
    draw_mode: DrawMode
}

interface Circle {
    kind: "circle",
    x: number,
    y: number,
    r: number,
}

interface Rect {
    kind: "rect",
    x: number,
    y: number,
    w: number,
    h: number,
}

interface And {
    kind: "and",
    children: Implicit[],
}

interface Or {
    kind: "or",
    children: Implicit[],
}

interface Translate {
    kind: "translate",
    dx: number,
    dy: number,
    target: Implicit,
}

interface Not {
    kind: "not",
    target: Implicit,
}

// TODO: polygons

interface Modulate {
    kind: "modulate",
    how_much: number,
    target: Implicit,
}

interface Break {
    kind: "break",
    target: Implicit,
}

interface Freeze {
    kind: "freeze",
    target: Implicit,
}

type ShapeOpts = {
    color: [number, number, number],
    draw_mode: DrawMode
};

export function shape(implicit: Implicit, opts?: Partial<ShapeOpts>): Shape {
    let default_opts: ShapeOpts = {
        color: [0, 0, 0],
        draw_mode: "filled",
    };

    let new_opts: ShapeOpts;

    if (opts === null || opts === undefined) {
        new_opts = default_opts;
    } else {
        new_opts = extend(default_opts, opts)
    }

    return {
        implicit: implicit,
        color: new_opts.color,
        draw_mode: new_opts.draw_mode,
    };
}

export function figure(...shapes: Shape[]): Figure {
    return {
        shapes: shapes
    }
}

type scene_opts = {
    unit: "px",
    simplify: boolean,
};

export function scene(figures: Figure[], opts?: scene_opts): Scene {
    let default_opts: scene_opts = {
        unit: "px",
        simplify: true,
    };

    if (opts === null || opts === undefined) {
        opts = default_opts;
    } else {
        opts = extend(default_opts, opts);
    }

    return {
        figures: figures,
        unit: opts.unit,
        simplify: opts.simplify,
    };
}

export function singleton_scene(implicit: Implicit) {
    return scene([figure(shape(implicit))]);
}

export function circle(x: number, y: number, r: number): Implicit {
    return {
        kind: "circle",
        x: x,
        y: y,
        r: r
    };
}

export function rect(x: number, y: number, w: number, h: number): Implicit {
    return {
        kind: "rect",
        x: x,
        y: y,
        w: w,
        h: h
    };
}

export function and(...children: Implicit[]): Implicit {
    return {
        kind: "and",
        children: children
    };
}

export function or(...children: Implicit[]): Implicit {
    return {
        kind: "or",
        children: children
    };
}

export function not(target: Implicit): Implicit {
    return {
        kind: "not",
        target: target
    };
}

export function subtract(a: Implicit, b: Implicit): Implicit {
    return and(a, not(b));
}

export function modulate(how_much: number, target: Implicit): Implicit {
    return {
        kind: "modulate",
        how_much: how_much,
        target: target,
    };
}

export function translate(dx: number, dy: number, target: Implicit): Implicit {
    return {
        kind: "translate",
        dx: dx,
        dy: dy,
        target: target,
    }
}

export function break_here(target: Implicit): Implicit {
    return {
        kind: "break",
        target: target,
    };
}

export function freeze(target: Implicit): Implicit {
    return {
        kind: "freeze",
        target: target,
    };
}

export function smooth_outer(how_much: number, target: Implicit): Implicit {
    return modulate(-how_much, freeze(modulate(how_much, target)))
}

export function smooth_inner(how_much: number, target: Implicit): Implicit {
    return modulate(how_much, freeze(modulate(-how_much, target)))
}

function extend<A, B>(a: A, b: B): A & B {
    let is_object = typeof b === "object" && b !== null && !Array.isArray(b);

    if (!is_object) {
        return b as any;
    }

    let result = JSON.parse(JSON.stringify(a));

    for (let key in b) {
        let already = result[key];
        if (already === null || already === undefined) {
            result[key] = b[key];
        } else {
            result[key] = extend(result[key], b[key]);
        }
    }
    return result;
}

export function write_scene(scene: Scene) {
    console.log(JSON.stringify(scene, null, 2));
}
