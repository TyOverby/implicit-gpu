import { write_scene, circle, modulate, subtract, Implicit, or, singleton_scene } from '../../../implicit-ts/src/lib/implicit';

const scale = 200;

function overlap(weight: number, ...impls: Implicit[]): Implicit {
    if (impls.length == 0) { return or(); }
    const [first, ...rest] = impls;
    return rest.reduce((prev, cur) => {
        let outline = modulate(-weight, prev);
        return or(prev, subtract(cur, outline));
    }, first)
}

const circles = [2, 2.4, 3, 4].map(i => {
    const disp = i * scale / 2 ;
    const r = scale / (Math.log(i) * (Math.log(i) * 2))
    return circle(disp, 0, r);
});
circles.reverse();

const scene = overlap(10, ...circles);

write_scene(singleton_scene(scene));
