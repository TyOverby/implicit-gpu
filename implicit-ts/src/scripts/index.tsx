import * as React from "react";
import * as ReactDOM from "react-dom";

import { OutputWindow } from "./components/OutputWindow";
import { ErrorWindow } from "./components/ErrorWindow";
import { Editor } from "./components/Editor";
import { State, current as default_state } from './state'

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

        const defaultText = `
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
        return <div>
            <div style={({ width: "100%", height: "100%" })}>
                <Editor>
                    {defaultText}
                </Editor>
            </div>
            {side_window}
        </div >
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
