import React, { useEffect, useState } from "react";
import "./App.css";
import { seal, code } from "./inputs";

const journalBase64 = "AAAAAINJFKY";
const idBase64 = "84keoVl3RAHWa4hlfV/q0f3opURNbbSp/7jU7qJ5Ft8";

function App() {
  const [text, setText] = useState("Validating proof...");

  useEffect(() => {
    const journal = Buffer.from(journalBase64, "base64");
    const sealBytes = new Uint32Array(Buffer.from(seal, "base64").buffer);
    const id = Buffer.from(idBase64, "base64");
    import("risc0-js").then((module) => {
      try {
        new module.Receipt(journal, sealBytes).validate(id);
        setText("Proof is valid");

        // const codeBytes = Buffer.from(code, "base64");
        // const input = new Uint32Array(
        //   Buffer.from("AACt3gAAAAAAAO++AAAAAA", "base64")
        // );
        // const genRec = module.generate_proof(codeBytes, id, input);
        // module.validate_proof(genRec, id);
        // setText("Proof is valid");
      } catch (e) {
        setText("Error: " + e);
        console.error(e);
      }
    });
  }, []);
  return <div className="App">{text}</div>;
}

export default App;
