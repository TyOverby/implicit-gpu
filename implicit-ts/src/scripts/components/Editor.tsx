import * as React from 'react';
import * as ReactDOM from 'react-dom';
import MonacoEditor from 'react-monaco-editor';
import * as state from '../state';
import run_compile from '../run_compile';

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

    constructor() {
        super();
        this.state = {
            code: start_code,
        };
    }

    shouldComponentUpdate() {
        return false;
    }

    componentDidMount() {
        const parent = ReactDOM.findDOMNode(this).parentElement;
        const listener = () => {
            if (parent === null) {
                throw new Error("not loaded");
            }
            if (this.editor === null) {
                setTimeout(listener, 50);
                return;
            }
            this.editor.layout({ width: parent.clientWidth, height: parent.clientHeight });
        };

        window.addEventListener('resize', listener);
        listener();
    }

    editorDidMount(editor: monaco.editor.ICodeEditor, monacoModule: typeof monaco) {
        editor.focus();
        this.editor = editor;
        this.onChange(start_code);
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

    async onChange(val: string) {
        if (this.editor == null) { return; }

        const model = this.editor.getModel();
        const worker = await monaco.languages.typescript.getTypeScriptWorker();
        const client = await worker(model.uri);
        const compilation = await client.getEmitOutput(model.uri.toString());
        const syntaxErrors: ErrorStructure[] = await client.getSyntacticDiagnostics(model.uri.toString());
        const semanticErrors: ErrorStructure[] = await client.getSemanticDiagnostics(model.uri.toString());
        const text = compilation.outputFiles[0].text;

        await run_compile(val, text, model, syntaxErrors, semanticErrors);
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

        return <MonacoEditor
            language="typescript"
            theme="vs-dark"
            value={code}
            options={options}
            onChange={this.onChange.bind(this)}
            editorDidMount={this.editorDidMount.bind(this)}
            editorWillMount={this.editorWillMount.bind(this)}
        />;
    }
}
