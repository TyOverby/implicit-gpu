import * as implicit from "../../../implicit-ts/implicit";

function grid(m: number, n: number, size: number, gap: number): implicit.Implicit[] {
    const out: implicit.Implicit[] = [];
    for (let i = 0; i < m; i++) {
        for (let k = 0; k < n; k++) {
            const x = i * size/2 * 2 + i * (gap / 2);
            const y = k * size/2 * 2 + k * gap / 2;
            out.push(implicit.circle(x, y, size / 2));
        }
    }
    return out;
}

const count = 4;
const size = 20;

const holes = implicit.or(... grid(count, count, size, size));
const rect = implicit.rect(-5, -5, count * size * 2, count * size * 2);
const shape = implicit.subtract(rect, holes);

var scene = implicit.singleton_scene(shape);
implicit.write_scene(scene);
