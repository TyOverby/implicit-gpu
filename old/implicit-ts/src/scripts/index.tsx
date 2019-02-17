import { Workspace, WorkspaceProps } from './components/Workspace';
import * as ReactDOM from 'react-dom';
import * as React from 'react';

const defaultText = `
import * as impl from 'implicit';

let normal = <Circle x={0} y={20} r={10} />
let should_error = <Circle x={0} y={20}/>
let explicit: impl.Implicit = <Circle x={0} y={20} r={10} />

export default impl.singleton_scene(explicit);
`.trim();

ReactDOM.render(
    <Workspace source={defaultText} />,
    document.querySelector("#container"));
