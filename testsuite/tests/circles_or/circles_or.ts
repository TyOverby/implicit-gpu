import * as impl from "../../../implicit-ts/src/implicit"

const shape =
    impl.or(
        impl.circle(50, 50, 100),
        impl.circle(100, 100, 100)
    );
const scene = impl.singleton_scene(shape);

impl.write_scene(scene)
