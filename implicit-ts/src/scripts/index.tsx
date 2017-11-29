import * as React from "react";
import * as ReactDOM from "react-dom";

import manualSize from "./components/ManualSize";
import { OutputWindow } from "./components/OutputWindow";
import { ErrorWindow } from "./components/ErrorWindow";
import { Editor } from "./components/Editor";
import { State, current as default_state } from './state'

const editor = (() => {
    let global_editor_instance: Editor | null = null;
    const global_editor = <Editor ref={(a) => global_editor_instance = a} />;
    const editor_proxy = manualSize(
        () => global_editor,
        (x, y, w, h) => {
            if (global_editor_instance != null) {
                global_editor_instance.update_position(x, y, w, h);
            }
        });
    return editor_proxy;
})();

export class Index extends React.Component<State> {
    render() {
        let side_window: JSX.Element;
        if (this.props.output.kind === 'ok') {
            side_window = <OutputWindow figures_svg={this.props.output.figures} />;
        } else if (this.props.output.kind === 'err' && this.props.prev_ok.length > 0) {
            side_window = <div>
                <OutputWindow figures_svg={this.props.prev_ok} />
                <ErrorWindow {... this.props.output.errors} />
            </div>;
        } else if (this.props.output.kind === 'err') {
            side_window = <ErrorWindow {... this.props.output.errors} />
        } else {
            throw new Error("unexpected output kind: ");
        }

        return <div>
            {editor}
            {side_window}
        </div>
    }
}

export function render(state: State) {
    ReactDOM.render(
        <Index {...state} />,
        document.querySelector("#container"));
}

let lastWidth = 0;
let lastHeight = 0;
function resize_handler() {
    if (lastWidth != window.innerWidth || lastHeight != window.innerHeight) {
        lastWidth = window.innerWidth;
        lastHeight = window.innerHeight;
        render(default_state);
    }

    window.requestAnimationFrame(resize_handler);
}

window.requestAnimationFrame(resize_handler);