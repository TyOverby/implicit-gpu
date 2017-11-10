declare var require;

function sleep(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

require.config({ paths: { 'vs': '../node_modules/monaco-editor/min/vs' } });
require(['../node_modules/monaco-editor/dev/vs/editor/editor.main'], function () {
    (async () => {
        const libts = await fetch("../src/implicit.ts").then(r => r.text());
        const libjs = await fetch("../out/implicit.js").then(r => r.text());

        let exports = {};
        eval(libjs);
        const implicit_module = exports;

        monaco.languages.typescript.typescriptDefaults.setCompilerOptions({
            target: monaco.languages.typescript.ScriptTarget.ES5,
            allowNonTsExtensions: true,
            diagnostics: true,
            strictNullChecks: true,
        });

        monaco.languages.typescript.typescriptDefaults.addExtraLib("declare module 'implicit' {" + libts + "}");

        const editor = monaco.editor.create(document.getElementById('container'), {
            value: `
import {circle, singleton_scene} from "implicit"
export var scene = singleton_scene(circle(10, 10, 30));
                `.trim(),
            language: 'typescript'
        });

        editor.onDidChangeModelContent(async () => {
            await sleep(0); // Let the language service get the changes.
            const model = editor.getModel();
            const worker = await monaco.languages.typescript.getTypeScriptWorker();
            const client = await worker(model.uri);
            const compilation = await client.getEmitOutput(model.uri.toString());
            const syntaxErrors = await client.getSyntacticDiagnostics(model.uri.toString());
            const semanticErrors = await client.getSemanticDiagnostics(model.uri.toString());
            const text = compilation.outputFiles[0].text;

            console.log(text);
            console.log(semanticErrors.length, syntaxErrors.length);

            if (semanticErrors.length === 0 && syntaxErrors.length === 0) {
                let exports = {};
                const require = () => implicit_module;

                try {
                    eval(text);
                } catch (e) {
                    console.error(e);
                }

                if (exports['scene']) {
                    console.log(exports['scene']);
                    let res = await fetch("/api/process", {
                        method: "POST",
                        body: JSON.stringify(exports['scene'])
                    });
                    if (res.status == 200) {
                        let svg = await res.text();
                        document.querySelector("#output").innerHTML = svg;
                        console.log(svg);
                    } else {
                        console.error(await res.text());
                    }
                }
            }
        });
    })()
});
