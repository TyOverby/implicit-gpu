import * as React from "react";
import * as ReactDOM from "react-dom";

import { Hello } from "./components/Hello";
import { Editor } from "./components/Editor";


ReactDOM.render(
    <div>
        <Editor />
        <Hello compiler="TypeScript" framework="React" />
    </div>,
    document.querySelector("#container"));
