import * as React from 'react';
import { getResult } from '../workerManager';
import MonacoEditor from 'react-monaco-editor';
import * as state from '../state';
import Error from '../types/error';

interface EditorProps { };

export interface ErrorStructure {
    start: number,
    length: number,
    messageText: string | ErrorStructure,
}

interface EditorState {
    code: string,
};

const start_code = `
import {circle, Implicit, or, singleton_scene} from 'implicit';

const circles: Implicit[] = [];

for (let i = 0; i < 10; i ++) {
    for (let k = 0; k < 10; k++) {
        const r = Math.sqrt(i + k);
        circles.push(circle(i * 10, k * 10, r));
    }
}

export default singleton_scene(or(... circles));
`.trim();

export class Editor extends React.Component<EditorProps, EditorState> {
    resize_interval_id: number = -1;
    editor: monaco.editor.ICodeEditor | null = null;

    pastWidth: number = 0;
    pastHeight: number = 0;

    constructor() {
        super();
        this.state = {
            code: start_code,
        };
    }

    shouldComponentUpdate() {
        return false;
    }

    editorDidMount(editor: monaco.editor.ICodeEditor, monacoModule: typeof monaco) {
        editor.focus();
        this.editor = editor;
        this.onChange("", null as any);
    }

    async editorWillMount(monacoModule: typeof monaco) {
        const implicit_source = await (await fetch('./implicit.ts')).text();
        monaco.languages.typescript.typescriptDefaults.addExtraLib("declare module 'implicit' {" + implicit_source + "}");

        monacoModule.languages.typescript.typescriptDefaults.setCompilerOptions({
            target: monaco.languages.typescript.ScriptTarget.ES5,
            allowNonTsExtensions: true,
            diagnostics: true,
            alwaysStrict: true,
            strictNullChecks: true,
        });
    }

    async onChange(val: string, ev: monaco.editor.IModelContentChangedEvent) {
        if (this.editor == null) { return; }

        const model = this.editor.getModel();
        const worker = await monaco.languages.typescript.getTypeScriptWorker();
        const client = await worker(model.uri);
        const compilation = await client.getEmitOutput(model.uri.toString());
        const syntaxErrors: ErrorStructure[] = await client.getSyntacticDiagnostics(model.uri.toString());
        const semanticErrors: ErrorStructure[] = await client.getSemanticDiagnostics(model.uri.toString());
        const text = compilation.outputFiles[0].text;

        if (syntaxErrors.length != 0 || semanticErrors.length != 0) {
            state.changeOutput({
                kind: 'err',
                errors: {
                    syntax: syntaxErrors.map(e => es_to_err(model, e)),
                    semantic: semanticErrors.map(e => es_to_err(model, e)),
                    runtime: [],
                },
            });
        } else {
            const result = await getResult(text);
            if (result.status === 'ok') {
                const res = await fetch("/api/process", {
                    method: "POST",
                    body: JSON.stringify(result.exports.default)
                });

                const result_text = await res.text();
                if (res.ok) {
                    state.changeOutput({
                        kind: 'ok',
                        figures_svg: [result_text]
                    });
                } else {
                    state.changeOutput({
                        kind: 'err',
                        errors: {
                            syntax: [],
                            semantic: [],
                            runtime: [{
                                col_num: 0,
                                line_num: 0,
                                message: result_text
                            }],
                        }
                    })
                }
            } else if (result.status === 'err') {
                state.changeOutput({
                    kind: 'err',
                    errors: {
                        syntax: [],
                        semantic: [],
                        runtime: [result.error],
                    }
                })
            }
        }
    }

    containerRef(e: HTMLDivElement | null) {
        if (e === null) {
            if (this.resize_interval_id != -1) {
                clearInterval(this.resize_interval_id)
            }
            return;
        }

        const resizeHandler = () => {
            if (e.clientHeight !== this.pastHeight || e.clientWidth !== this.pastWidth) {
                this.pastHeight = e.clientHeight;
                this.pastWidth = e.clientWidth;
                if (this.editor !== null) {
                    this.editor.layout({ width: this.pastWidth, height: this.pastHeight });
                }
            }
        }

        window.addEventListener('resize', resizeHandler);
        this.resize_interval_id = setInterval(resizeHandler, 500);
    }

    render() {
        const code = this.state.code;
        const options: monaco.editor.IEditorOptions = {
            scrollBeyondLastLine: false,
            selectOnLineNumbers: true,
            renderLineHighlight: 'none',
            minimap: {
                enabled: false,
            }
        };

        return <div className="inside-relative" ref={(e) => this.containerRef(e)}>
            <MonacoEditor
                width="50%"
                height="50%"
                language="typescript"
                theme="vs-dark"
                value={code}
                options={options}
                onChange={this.onChange.bind(this)}
                editorDidMount={this.editorDidMount.bind(this)}
                editorWillMount={this.editorWillMount.bind(this)}
            />
        </div>;
    }
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
