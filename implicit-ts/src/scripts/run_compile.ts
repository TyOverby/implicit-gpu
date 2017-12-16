import { getResult } from './workerManager';
import { Error, ErrorStructure } from './types/error';
import { RenderFunc, Figure } from './components/Workspace';

export default async function run_compile(
    source: string,
    text: string,
    model: monaco.editor.IModel,
    syntaxErrors: ErrorStructure[],
    semanticErrors: ErrorStructure[],
    render: RenderFunc) {

    // Zero out the errors
    render({
        compiled: text,
        errors: {
            syntax: [],
            semantic: [],
            runtime: [],
        }
    });

    if (syntaxErrors.length != 0 || semanticErrors.length != 0) {
        render({
            errors: {
                syntax: syntaxErrors.map(e => es_to_err(model, e)),
                semantic: semanticErrors.map(e => es_to_err(model, e)),
                runtime: [],
            }
        });
    }

    const result = await getResult(text);

    if (result.status === 'err') {
        render({
            errors: {
                syntax: [],
                semantic: [],
                runtime: [result.error],
            }
        });
        return;
    }

    render({
        scene: JSON.stringify(result.exports.default, null, 2)
    });

    const res = await fetch("/api/process", {
        method: "POST",
        body: JSON.stringify({
            source: source,
            scene: result.exports.default
        })
    });

    if (!res.ok) {
        render({
            errors: {
                syntax: [],
                semantic: [],
                runtime: [{
                    col_num: 0,
                    line_num: 0,
                    message: await res.text()
                }]
            }
        });
        return;
    }

    type Result = {
        figures: Figure[],
        perf: any,
    };

    const result_text = await res.text();
    const output: Result = JSON.parse(result_text);
    render({
        figures: output.figures,
        perf: output.perf
    })
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
