import * as impl from 'implicit';

let scaling = 50;
let outer_radius = 10 * scaling;
let top_radius = 1 * scaling;
let corner_radius = 2 * scaling;

let corner_d = Math.sqrt((Math.pow(outer_radius - corner_radius, 2) / 2));

let outer = <Circle x={0} y={0} r={outer_radius} />
let center = <Circle x={0} y={0} r={3*scaling} />

let north = <Circle x={0} y={-outer_radius + top_radius} r={top_radius} /> 
let south = <Circle x={0} y={outer_radius - top_radius} r={top_radius} /> 
let east = <Circle y={0} x={-outer_radius + top_radius} r={top_radius} /> 
let west = <Circle y={0} x={outer_radius - top_radius} r={top_radius} />

let a = <Circle x={corner_d} y={corner_d} r={corner_radius} />
let b = <Circle x={-corner_d} y={corner_d} r={corner_radius} />
let c = <Circle x={corner_d} y={-corner_d} r={corner_radius} />
let d = <Circle x={-corner_d} y={-corner_d} r={corner_radius} />


let subs = <Or>{center}{north}{south}{east}{west}{a}{b}{c}{d}</Or>;
let scene = impl.subtract(outer as any, subs as any);
export default impl.singleton_scene(scene);
