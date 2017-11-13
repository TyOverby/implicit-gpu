import * as React from "react";

export interface OutputProps { figures_svg: string[] }

// 'HelloProps' describes the shape of props.
// State is never set so we use the 'undefined' type.
export class OutputWindow extends React.Component<OutputProps> {
    render() {
        function text_to_svg(src: string, idx: number): JSX.Element {
            return <div key={idx} dangerouslySetInnerHTML={({ __html: src })} />
        }
        const svgs = this.props.figures_svg.map(text_to_svg);
        return <div>
            {svgs}
        </div>
    }
}
