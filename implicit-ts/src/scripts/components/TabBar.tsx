import * as React from 'react';
import * as ReactDom from 'react';

export interface TabBarProps {

}

interface TabBarState {
    selected: number
}

export class TabBar extends React.Component<TabBarProps, TabBarState> {
    constructor(props: TabBarProps) {
        super(props);
        this.state = {
            selected: 0
        };
    }

    render() {
        let tags = flatten(this.props.children);
        let even = [];
        let odd = [];
        for (let i = 0; i < tags.length; i++) {
            if (i % 2 === 0) {
                const setSelected = () => this.setState({ selected: i / 2 });
                const isSelected = this.state.selected == i / 2;
                const className = "individual-tab" + (isSelected ? " selected" : "");
                const element =
                    <div key={i} className={className} onClick={setSelected} >
                        {tags[i]}
                    </div>
                even.push(element);
            } else {
                odd.push(tags[i]);
            }
        }

        return <div className="tabs">
            <div className="tag-group">
                {even}
            </div>
            <div className="tab-selected">
                {odd[this.state.selected]}
            </div>
        </div>;
    }
}

function flatten(c: React.ReactNode): React.ReactNode[] {
    switch (typeof c) {
        case 'string':
        case 'number':
        case 'boolean':
            return [c];
    }

    if (c === null || c === undefined) {
        return [];
    }

    if (Array.isArray(c)) {
        return c.map(flatten).reduce((a, b) => { a.push(...b); return a }, []);
    }

    return [c]
}
