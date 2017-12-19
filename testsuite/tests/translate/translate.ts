import * as impl from "../../../implicit-ts/src/lib/implicit"

const shape = impl.translate(10, 10, impl.circle(50, 50, 50));
const scene = impl.singleton_scene(shape);

impl.write_scene(scene)
