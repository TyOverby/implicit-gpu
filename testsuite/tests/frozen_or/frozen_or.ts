import * as impl from "../../../implicit-ts/src/lib/implicit"

const shape =
    impl.freeze(impl.modulate(10, impl.or(
        impl.circle(30, 0, 30),
        impl.circle(60, 0, 30)
    )));
const scene = impl.singleton_scene(shape);

impl.write_scene(scene)
