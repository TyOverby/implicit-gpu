import { Workspace, WorkspaceProps } from './components/Workspace';
import * as ReactDOM from 'react-dom';
import * as React from 'react';

const defaultText = `
import * as impl from 'implicit';

let c = <Circle x={0} y={20} r={10} />
export default impl.singleton_scene(c as any);
`.trim();

ReactDOM.render(
    <Workspace source={defaultText} />,
    document.querySelector("#container"));
