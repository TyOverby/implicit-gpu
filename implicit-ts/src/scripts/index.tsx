import * as React from "react";
import * as ReactDOM from "react-dom";

import { OutputWindow } from "./components/OutputWindow";
import { ErrorWindow } from "./components/ErrorWindow";
import { SidePane } from "./components/SidePane";
import { Editor } from "./components/Editor";
import { State, current as default_state } from './state'

export class Index extends React.Component<State> {
    render() {
        const defaultText = `
import * as impl from 'implicit';

let c = <Circle x={0} y={20} r={10} />
export default impl.singleton_scene(c as any);
`.trim();
        return <div>
            <div style={({ width: "100%", height: "100%" })}>
                <Editor>
                    {defaultText}
                </Editor>
            </div>
            <SidePane {...this.props} />
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
