import { render } from './index'
import { ErrorStructure } from './components/Editor'
import Error from './types/error';

export var history: State[] = [];

export var current: State = {
    source: "import { circle} from 'implicit';\nexport default circle(0, 0, 100);",
    prev_ok: [],
    output: {
        kind: 'ok',
        figures_svg: []
    }
};

export type OutputState =
    {
        kind: 'ok',
        figures_svg: string[]
    } |
    {
        kind: 'err', errors: {
            syntax: Error[],
            semantic: Error[],
            runtime: Error[],
        }
    }

export type State = {
    source: string,
    prev_ok: string[],
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
        current.prev_ok = output.figures_svg;
    }

    setDirty();
}

