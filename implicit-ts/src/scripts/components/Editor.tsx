import * as React from 'react';
import * as ReactDOM from 'react-dom';
import { ErrorStructure } from '../types/error';
import "monaco-editor";
import { ErrorWindow } from './ErrorWindow';
import run_compile from '../run_compile';

export class Editor extends React.Component {
    me: HTMLElement;
    editor: monaco.editor.IEditor;

    async componentDidMount() {
        this.me = ReactDOM.findDOMNode(this) as HTMLElement;
        await (window as any).monaco_loaded;
        await this.monacoDidLoad();
    }

    async monacoDidLoad() {
        const [
            implicit_source,
            react_like_source,
            jsx_like_source,
            components_source,
        ] = await Promise.all([
            fetch('./lib/implicit.ts').then(a => a.text()),
            fetch('./lib/react_like.ts').then(a => a.text()),
            fetch('./lib/jsx_like.ts').then(a => a.text()),
            fetch('./lib/components.d.ts').then(a => a.text()),
        ]);

        monaco.languages.typescript.typescriptDefaults.addExtraLib("declare module 'implicit' {" + implicit_source + "}", "implicit.ts");
        monaco.languages.typescript.typescriptDefaults.addExtraLib(react_like_source, "react_like.ts");
        monaco.languages.typescript.typescriptDefaults.addExtraLib(jsx_like_source, "jsx_like.ts");
        monaco.languages.typescript.typescriptDefaults.addExtraLib(components_source, "components.d.ts");
        monaco.languages.typescript.typescriptDefaults.setCompilerOptions({
            target: monaco.languages.typescript.ScriptTarget.ES5,
            lib: ["ES6"],
            jsx: 2,
            jsxFactory: 'Impl.createElement',
            reactNamespace: 'Impl',
            allowNonTsExtensions: true,
            sourceMap: true,
            diagnostics: true,
            alwaysStrict: true,
            strictNullChecks: true,
        });

        let text: string;
        if (typeof this.props.children === 'string') {
            text = this.props.children;
        } else if (Array.isArray(this.props.children)) {
            text = (this.props.children as string[]).reduce((a, b) => a + b, "");
        } else {
            text = "";
        }

        this.editor = monaco.editor.create(this.me, {
            folding: true,
            showFoldingControls: 'always',
            theme: "vs-dark",
            minimap: {
                enabled: false
            },
            selectOnLineNumbers: true,
            automaticLayout: true,
            model: monaco.editor.createModel(text, "typescript", monaco.Uri.file("./test.tsx")),
        });

        const model = this.editor.getModel() as monaco.editor.IModel;
        this.modelUpdated(model);
        model.onDidChangeContent(e => this.modelUpdated(model));
    }

    async modelUpdated(model: monaco.editor.IModel) {
        const string = model.getValue();
        const worker = await monaco.languages.typescript.getTypeScriptWorker();
        const client = await worker(model.uri);
        const compilation = await client.getEmitOutput(model.uri.toString());
        const syntaxErrors: ErrorStructure[] = await client.getSyntacticDiagnostics(model.uri.toString());
        const semanticErrors: ErrorStructure[] = await client.getSemanticDiagnostics(model.uri.toString());
        const output = compilation.outputFiles[1].text;
        const mapping = compilation.outputFiles[0].text;
        await this.textUpdated(string, output, mapping, model, syntaxErrors, semanticErrors);
    }

    async textUpdated(
        source: string,
        output: string,
        mapping: string,
        model: monaco.editor.IModel,
        syntaxErrors: ErrorStructure[],
        semanticErrors: ErrorStructure[],
    ) {
        console.log(output);
        await run_compile(source, output, model, syntaxErrors, semanticErrors);
    }

    render() {
        const style: React.CSSProperties = {
            width: "100%",
            height: "100%",
            overflow: "hidden",
        };

        return <div className="react-editor-container" style={style} />
    }
}
