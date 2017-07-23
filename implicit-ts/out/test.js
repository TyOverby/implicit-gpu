"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var implicit_1 = require("./implicit");
var sub = implicit_1.subtract(implicit_1.circle(5.0, 10.0, 30.0), implicit_1.rect(5.0, 10.0, 30.0, 40.0));
//let out = scene([figure(shape(sub))])
var out = implicit_1.singleton_scene(sub);
console.log(JSON.stringify(out, null, 2));
