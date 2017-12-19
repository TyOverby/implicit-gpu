import * as impl from "../../../implicit-ts/src/lib/implicit"

const shape =
    impl.or(
        impl.and(
            impl.circle(50, 50, 50),
            impl.not(impl.circle(50, 50, 25))
        ),
        impl.circle(50, 50, 10)
    )
const scene = impl.singleton_scene(shape);

impl.write_scene(scene)
