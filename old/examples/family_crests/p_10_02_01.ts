import * as impl from 'implicit';

const scaling = 50;
const long = 10 * scaling;
const short = 3 * scaling;
const gap_top = 0.5 * scaling;

const a = <Rect x={0} y={gap_top} w={long} h={short}/>
const b = <Rect x={gap_top} y={0} w={short} h={long}/>
const c = <Rect x={long - gap_top - short} y={0} w={short} h={long}/>
const d = <Rect x={0} y={long - gap_top - short} w={long} h={short}/>

const scene = <Or>{a}{b}{c}{d}</Or>

export default impl.singleton_scene(scene as any);
