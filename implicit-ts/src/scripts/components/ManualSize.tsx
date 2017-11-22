import * as React from 'react';
import * as ReactDOM from 'react-dom';

type Constructor = () => JSX.Element;
type Updater = (x: number, y: number, w: number, h: number) => void;

export default function manualSize(c: Constructor, update: Updater, id?: string): JSX.Element {
    return <ManualSizing cons={c} up={update} id={id} />
}

type ManualProps = {
    id?: string;
    cons: Constructor;
    up: Updater;
};

type ManualState = {
    x: number;
    y: number;
    width: number;
    height: number;
};

class ManualSizing extends React.Component<ManualProps, ManualState> {


    dom: Element;
    last_animation_tick: number;

    constructor() {
        super();
        this.state = { x: 0, y: 0, width: 0, height: 0 };
        this.last_animation_tick = 0;
    }

    componentDidMount() {
        this.dom = ReactDOM.findDOMNode(this);
        this.attempt_resize();
    }
    componentWillUnmount() {
        if (this.last_animation_tick !== 0) {
            window.cancelAnimationFrame(this.last_animation_tick);
        }
    }

    attempt_resize() {
        const new_x = this.dom.clientLeft;
        const new_y = this.dom.clientTop;
        const new_w = this.dom.clientWidth;
        const new_h = this.dom.clientHeight;

        if (this.state.x != new_x ||
            this.state.y != new_y ||
            this.state.width != new_w ||
            this.state.height != new_h) {

            this.props.up(new_x, new_y, new_w, new_h);

            this.setState({
                x: new_x,
                y: new_y,
                width: new_w,
                height: new_h,
            })

        }

        this.last_animation_tick = window.requestAnimationFrame(() => this.attempt_resize());
    }

    render() {
        return <div id={this.props.id} className="manual-size">
            {this.props.cons()}
        </div>
    }
}
