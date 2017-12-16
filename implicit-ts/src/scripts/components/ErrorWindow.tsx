import * as React from "react";
import { Error } from '../types/error';

export interface ErrorProps {
    syntax: Error[],
    semantic: Error[],
    runtime: Error[],
}

function error_to_row(ident: string): (error: Error, index: number) => JSX.Element {
    return (error: Error, index: number) => {
        return <tr key={ident + index}>
            <td> {error.line_num} </td>
            <td> {error.col_num} </td>
            <td> {error.message} </td>
        </tr>
    }
}

// 'HelloProps' describes the shape of props.
// State is never set so we use the 'undefined' type.
export class ErrorWindow extends React.Component<ErrorProps> {
    render() {
        let sections = [];
        if (this.props.syntax.length > 0) {
            sections.push(<tr key="syntax"><td colSpan={3}> <h2>Syntax Errors</h2> </td></tr>)
            sections.push(... this.props.syntax.map(error_to_row("syntax")));
        }

        if (this.props.semantic.length > 0) {
            sections.push(<tr key="semantic"><td colSpan={3}> <h2>Semantic Errors</h2> </td></tr>)
            sections.push(... this.props.semantic.map(error_to_row("semantic")));
        }

        if (this.props.runtime.length > 0) {
            sections.push(<tr key="runtime"><td colSpan={3}> <h2>Runtime Errors</h2> </td></tr>)
            sections.push(... this.props.runtime.map(error_to_row("runtime")));
        }

        return <div id="error-window">
            <h1> Errors! </h1>
            <table cellSpacing={0}>
                <thead>
                    <tr>
                        <th> Line </th>
                        <th> Col </th>
                        <th> Message </th>
                    </tr>
                </thead>
                <tbody>
                    {sections}
                </tbody>
            </table>
        </div>
    }
}
