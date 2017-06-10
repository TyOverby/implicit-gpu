import { circle, or, rect} from './implicit';

console.log(circle(5.0, 10.0, 30.0))
let circle_and_rect = [
    circle(5.0, 10.0, 30.0),
    rect(5.0, 10.0, 30.0, 40.0)
];

let twice_times = [...circle_and_rect, ...circle_and_rect]

console.log(or(...twice_times, ...twice_times))



