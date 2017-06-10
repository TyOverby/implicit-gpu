"use strict";
exports.__esModule = true;
var implicit_1 = require("./implicit");
console.log(implicit_1.circle(5.0, 10.0, 30.0));
var circle_and_rect = [
    implicit_1.circle(5.0, 10.0, 30.0),
    implicit_1.rect(5.0, 10.0, 30.0, 40.0)
];
var twice_times = circle_and_rect.concat(circle_and_rect);
console.log(implicit_1.or.apply(void 0, twice_times.concat(twice_times)));
