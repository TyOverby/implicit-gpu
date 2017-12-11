import { render } from './index'
import Error from './types/error';

export var history: State[] = [];

export var current: State = {
    source: "import { circle} from 'implicit';\nexport default circle(0, 0, 100);",
    prev_ok: [],
    output: {
        kind: 'ok',
        figures: []
    }
};

export type Figure = {
    svg: string,
    left: number,
    top: number,
    width: number,
    height: number,
}

export type Errors = {
    syntax: Error[],
    semantic: Error[],
    runtime: Error[],
}
export type OutputState =
    {
        kind: 'ok',
        figures: Figure[]
    } |
    {
        kind: 'err',
        errors: Errors
    }


export type State = {
    source: string,
    prev_ok: Figure[],
    output: OutputState,
}

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

export function changeSource(source: string) {
    history.push(current);
    current = clone(current);
    current.source = source;
    setDirty();
}

export function changeOutput(output: OutputState) {
    const previous = current;
    history.push(current);

    current = clone(current);
    output = clone(output);
    current.output = output;

    if (output.kind == 'ok') {
        current.prev_ok = output.figures;
    }

    setDirty();
}

export function changeError(errors: Errors) {
    changeOutput({ 'kind': 'err', errors: errors });
}
