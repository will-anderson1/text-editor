import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import TextEditor from "./TextEditor/TextEditor"
function MainContent() {
  const [code, setCode] = useState(
    `function add(a, b) {\n  return a + b;\n}`
  );
  return (
    <main>
        <TextEditor/>
    </main>
  );
}

export default MainContent;
