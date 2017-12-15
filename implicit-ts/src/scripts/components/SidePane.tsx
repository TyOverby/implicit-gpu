import * as React from "react";
import * as ReactDOM from "react-dom";

import { OutputWindow } from "./OutputWindow";
import { TabBar } from "./TabBar";
import { ErrorWindow } from "./ErrorWindow";
import { Editor } from "./Editor";
import { State } from '../state'
import { Fireplace } from '../../../../../../web/flame-vis/src/scripts/components/Fireplace';

export class SidePane extends React.Component<State> {
    render() {
        let perf = this.props.perf.length > 0 ? <Fireplace threads={this.props.perf} /> : <span> no perf info yet</span>
        return <div className="side-bar">
            <TabBar>
                <span>Output</span>
                <OutputWindow figures_svg={this.props.figures} />

                <span>Errors</span>
                <ErrorWindow {...this.props.errors} />

                <span> Performance</span>
                {perf}
            </TabBar>
        </div>
    }
}
