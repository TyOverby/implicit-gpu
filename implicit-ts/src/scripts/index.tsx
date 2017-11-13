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
            side_window = <OutputWindow figures_svg={this.props.output.figures_svg} />;
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
            <Editor />
            {side_window}
        </div>
    }
}

export function render(state: State) {
    ReactDOM.render(
        <Index {...state} />,
        document.querySelector("#container"));
}

render(default_state);
