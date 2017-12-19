import * as implicit from "../../../implicit-ts/src/lib/implicit"

const rect = implicit.rect(0, 0, 50, 50);
const circle = implicit.circle(20, 20, 10);
const shape = implicit.and(circle, rect);

var scene = implicit.singleton_scene(shape);
implicit.write_scene(scene);
