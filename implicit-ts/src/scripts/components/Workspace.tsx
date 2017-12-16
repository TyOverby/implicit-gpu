import * as React from "react";
import * as ReactDOM from "react-dom";

import { OutputWindow } from "./OutputWindow";
import { ErrorWindow } from "./ErrorWindow";
import { SidePane } from "./SidePane";
import { Editor } from "./Editor";
import { Error } from "../types/error";

export type Figure = {
    svg: string,
    left: number,
    top: number,
    width: number,
    height: number,
};

export type Errors = {
    syntax: Error[],
    semantic: Error[],
    runtime: Error[],
};

export type Perf = any; // TODO: fix

export type WorkspaceProps = {
    source: string,
};

export type WorkspaceState = {
    figures: Figure[],
    errors: Errors,

    // Debugging Help
    perf: Perf,
    compiled: string,
    scene: string,
};

export type RenderFunc = (state: Partial<WorkspaceState>) => void;

export class Workspace extends React.Component<WorkspaceProps, WorkspaceState> {
    constructor(props: WorkspaceProps) {
        super(props);

        this.state = {
            figures: [],
            errors: {
                syntax: [],
                semantic: [],
                runtime: [],
            },

            compiled: "",
            perf: [],
            scene: "",
        };
    }
    render() {
        const combined = { ...this.props, ...this.state };
        const render = ((s: any) => this.setState(s)) as RenderFunc;

        return <div>
            <div style={({ width: "100%", height: "100%" })}>
                <Editor render={render}>
                    {this.props.source}
                </Editor>
            </div>
            <SidePane {...combined} />
        </div >
    }
}
