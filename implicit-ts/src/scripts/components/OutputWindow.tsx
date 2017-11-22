import * as React from "react";
import * as ReactDOM from "react-dom";
import { Figure } from '../state';

export interface OutputProps { figures_svg: Figure[] }
export interface OutputState { }

// 'HelloProps' describes the shape of props.
// State is never set so we use the 'undefined' type.
export class OutputWindow extends React.Component<OutputProps, OutputState> {
    me: Element | null = null;

    constructor() {
        super()
        this.state = { width: 0, height: 0 };
    }

    componentDidMount() {
        this.me = ReactDOM.findDOMNode(this);
    }

    render() {
        const text_to_svg = (figure: Figure, idx: number): JSX.Element => {
            const viewbox = `${0} ${0} ${figure.width - figure.left} ${figure.height - figure.top}`;
            const source = figure.svg.replace(
                "<svg ",
                `<svg viewbox=\"${viewbox}\"`);
            return <div key={idx} style={({ position: 'absolute' })} dangerouslySetInnerHTML={({ __html: source })} />
        }

        const svgs = this.props.figures_svg.map(text_to_svg);
        return <div>
            {svgs}
        </div >
    }
}
