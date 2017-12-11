import { getResult } from './workerManager';
import * as state from './state';
import Error, { ErrorStructure } from './types/error';

export default async function run_compile(
    source: string,
    text: string,
    model: monaco.editor.IModel,
    syntaxErrors: ErrorStructure[],
    semanticErrors: ErrorStructure[]) {

    if (syntaxErrors.length != 0 || semanticErrors.length != 0) {
        state.changeError({
            syntax: syntaxErrors.map(e => es_to_err(model, e)),
            semantic: semanticErrors.map(e => es_to_err(model, e)),
            runtime: [],
        });
        return;
    }

    const result = await getResult(text);

    if (result.status === 'err') {
        state.changeError({
            syntax: [],
            semantic: [],
            runtime: [result.error],
        });
        return;
    }


    const res = await fetch("/api/process", {
        method: "POST",
        body: JSON.stringify({
            source: source,
            scene: result.exports.default
        })
    });

    if (!res.ok) {
        state.changeError({
            syntax: [],
            semantic: [],
            runtime: [{
                col_num: 0,
                line_num: 0,
                message: await res.text()
            }]
        });

        return;
    }

    const result_text = await res.text();
    const figures: state.Figure[] = JSON.parse(result_text);

    state.changeOutput({
        kind: 'ok',
        figures: figures
    });
}

function es_to_err(model: monaco.editor.IModel, es: ErrorStructure): Error {
    const p = model.getPositionAt(es.start);
    const m: string = typeof es.messageText === 'string' ?
        es.messageText :
        es_to_err(model, es.messageText).message;
    return {
        message: m,
        line_num: p.lineNumber,
        col_num: p.column,
    }
}
