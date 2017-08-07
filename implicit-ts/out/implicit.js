"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
function shape(implicit, opts) {
    var default_opts = {
        color: [0, 0, 0],
        draw_mode: "filled",
    };
    if (opts === null || opts === undefined) {
        opts = default_opts;
    }
    else {
        opts = extend(default_opts, opts);
    }
    return {
        implicit: implicit,
        color: opts.color,
        draw_mode: opts.draw_mode,
    };
}
exports.shape = shape;
function figure() {
    var shapes = [];
    for (var _i = 0; _i < arguments.length; _i++) {
        shapes[_i] = arguments[_i];
    }
    return {
        shapes: shapes
    };
}
exports.figure = figure;
function scene(figures, opts) {
    var default_opts = {
        unit: "px",
        simplify: true,
    };
    if (opts === null || opts === undefined) {
        opts = default_opts;
    }
    else {
        opts = extend(default_opts, opts);
    }
    return {
        figures: figures,
        unit: opts.unit,
        simplify: opts.simplify,
    };
}
exports.scene = scene;
function singleton_scene(implicit) {
    return scene([figure(shape(implicit))]);
}
exports.singleton_scene = singleton_scene;
function circle(x, y, r) {
    return {
        kind: "circle",
        x: x,
        y: y,
        r: r
    };
}
exports.circle = circle;
function rect(x, y, w, h) {
    return {
        kind: "rect",
        x: x,
        y: y,
        w: w,
        h: h
    };
}
exports.rect = rect;
function and() {
    var children = [];
    for (var _i = 0; _i < arguments.length; _i++) {
        children[_i] = arguments[_i];
    }
    return {
        kind: "and",
        children: children
    };
}
exports.and = and;
function or() {
    var children = [];
    for (var _i = 0; _i < arguments.length; _i++) {
        children[_i] = arguments[_i];
    }
    return {
        kind: "or",
        children: children
    };
}
exports.or = or;
function not(target) {
    return {
        kind: "not",
        target: target
    };
}
exports.not = not;
function subtract(a, b) {
    return and(a, not(b));
}
exports.subtract = subtract;
function modulate(how_much, target) {
    return {
        kind: "modulate",
        how_much: how_much,
        target: target,
    };
}
exports.modulate = modulate;
function translate(dx, dy, target) {
    return {
        kind: "translate",
        dx: dx,
        dy: dy,
        target: target,
    };
}
exports.translate = translate;
function break_here(target) {
    return {
        kind: "break",
        target: target,
    };
}
exports.break_here = break_here;
function freeze(target) {
    return {
        kind: "freeze",
        target: target,
    };
}
exports.freeze = freeze;
function extend(a, b) {
    var is_object = typeof b === "object" && b !== null && !Array.isArray(b);
    if (!is_object) {
        return b;
    }
    var result = JSON.parse(JSON.stringify(a));
    for (var key in b) {
        var already = result[key];
        if (already === null || already === undefined) {
            result[key] = b[key];
        }
        else {
            result[key] = extend(result[key], b[key]);
        }
    }
    return result;
}
function write_scene(scene) {
    console.log(JSON.stringify(scene, null, 2));
}
exports.write_scene = write_scene;
