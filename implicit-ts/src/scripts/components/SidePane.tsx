import * as React from "react";
import * as ReactDOM from "react-dom";

import { OutputWindow } from "./OutputWindow";
import { TabBar } from "./TabBar";
import { ErrorWindow } from "./ErrorWindow";
import { Editor } from "./Editor";
import { WorkspaceState } from './Workspace'
import { Fireplace } from '../../../../../../web/fireplace/src/scripts/components/Fireplace';

export class SidePane extends React.Component<WorkspaceState> {
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

                <span> Compiled </span>
                <pre>{this.props.compiled}</pre>

                <span> Scene </span>
                <pre>{this.props.scene}</pre>
            </TabBar>
        </div>
    }
}
