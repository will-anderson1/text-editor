import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import {CodeiumEditor} from "../CustomCodeium/components";
function TextEditor() {
  return (
    <div>
      <CodeiumEditor language="python" theme="vs-dark"/>
    </div>
  );
}

export default TextEditor;
