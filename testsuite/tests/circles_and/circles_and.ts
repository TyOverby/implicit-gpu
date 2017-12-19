import * as impl from "../../../implicit-ts/src/lib/implicit"

const shape =
    impl.and(
        impl.circle(50, 50, 50),
        impl.circle(100, 100, 50)
    );
const scene = impl.singleton_scene(shape);

impl.write_scene(scene)
