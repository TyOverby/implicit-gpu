import * as React from 'react';
import MonacoEditor from 'react-monaco-editor';

interface EditorProps { };

interface EditorState {
    code: string,
};

export class Editor extends React.Component<EditorProps, EditorState> {
    resize_interval_id: number = -1;
    editor: monaco.editor.ICodeEditor | null = null;

    pastWidth: number = 0;
    pastHeight: number = 0;

    constructor() {
        super();
        this.state = {
            code: '// type your code...',
        };
    }

    shouldComponentUpdate() {
        return false;
    }

    editorDidMount(editor: monaco.editor.ICodeEditor, monacoModule: typeof monaco) {
        editor.focus();
        this.editor = editor;
    }

    editorWillMount(monacoModule: typeof monaco) {
        monacoModule.languages.typescript.typescriptDefaults.setCompilerOptions({
            target: monaco.languages.typescript.ScriptTarget.ES5,
            allowNonTsExtensions: true,
            diagnostics: true,
            alwaysStrict: true,
            strictNullChecks: true,
        });

        monaco.languages.typescript.typescriptDefaults.addExtraLib("declare module 'implicit' { export var x: number = 3; }");
    }

    async onChange(val: string, ev: monaco.editor.IModelContentChangedEvent) {
        console.log('onChange', val, ev);

        if (this.editor == null) { return;}

        const model = this.editor.getModel();
        const worker = await monaco.languages.typescript.getTypeScriptWorker();
        const client = await worker(model.uri);
        const compilation = await client.getEmitOutput(model.uri.toString());
        const syntaxErrors = await client.getSyntacticDiagnostics(model.uri.toString());
        const semanticErrors = await client.getSemanticDiagnostics(model.uri.toString());
        const text = compilation.outputFiles[0].text;
        console.log(text);
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
            selectOnLineNumbers: true,
            renderLineHighlight: 'none',
            minimap: {
                enabled: false,
            }
        };

        return <div className="inside-relative" ref={(e) => this.containerRef(e)}>
            <MonacoEditor
                width="100%"
                height="100%"
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
