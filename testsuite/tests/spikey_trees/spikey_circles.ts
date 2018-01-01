
import { write_scene, circle, Implicit, or, singleton_scene } from '../../../implicit-ts/src/lib/implicit';

const circles: Implicit[] = [];

for (let i = 0; i < 10; i ++) {
    for (let k = 0; k < 10; k++) {
        circles.push(circle(i * 20, k * 6, k/1.5));
    }
}

const scene = singleton_scene(or(...circles));
write_scene(scene)
