"use strict";
exports.__esModule = true;
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
function modulate(how_much, target) {
    return {
        kind: "modulate",
        how_much: how_much,
        target: target
    };
}
exports.modulate = modulate;
function break_here(target) {
    return {
        kind: "break",
        target: target
    };
}
exports.break_here = break_here;
function freeze(target) {
    return {
        kind: "freeze",
        target: target
    };
}
exports.freeze = freeze;
