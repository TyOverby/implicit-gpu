import * as impl from "../../../implicit-ts/implicit"

const shape = impl.circle(50, 50, 50);
const scene = impl.singleton_scene(shape);

impl.write_scene(scene)
