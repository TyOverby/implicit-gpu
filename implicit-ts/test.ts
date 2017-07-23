import { circle, or, rect, subtract, singleton_scene} from './implicit';

let sub =
    subtract(
        circle(5.0, 10.0, 30.0),
        rect(5.0, 10.0, 30.0, 40.0));


//let out = scene([figure(shape(sub))])
let out = singleton_scene(sub);

console.log(JSON.stringify(out, null, 2))
