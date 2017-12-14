import { render } from './index'
import Error from './types/error';

export var history: State[] = [];

type Perf = any;

export var current: State = {
    source: "import { circle} from 'implicit';\nexport default circle(0, 0, 100);",
    figures: [],
    perf: [],
    errors: {
        syntax: [],
        semantic: [],
        runtime: [],
    },
};

export type Figure = {
    svg: string,
    left: number,
    top: number,
    width: number,
    height: number,
};

export type Errors = {
    syntax: Error[],
    semantic: Error[],
    runtime: Error[],
};

export type State = {
    source: string,
    figures: Figure[],
    errors: Errors,
    perf: Perf, // TODO: fix
};

let isDirty: boolean = false;

function clone<T>(input: T): T {
    return JSON.parse(JSON.stringify(input));
}

function setDirty() {
    if (isDirty) {
        return;
    }

    isDirty = true;

    requestAnimationFrame(() => {
        render(current);
        isDirty = false;
    });
}

function prep() {
    setDirty();
    history.push(current);
    current = clone(current);
}

export function changeSource(source: string) {
    prep();
    current.source = source;
}

export function changeError(errors: Errors) {
    prep();
    current.errors = errors;
}

export function changeFigures(figures: Figure[]) {
    prep();
    current.figures = figures;
}

export function changePerf(perf: Perf) {
    prep();
    current.perf = perf;
}
