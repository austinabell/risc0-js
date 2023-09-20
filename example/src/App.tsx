import { useEffect, useState } from "react";
import "./App.css";
import init, { Receipt } from "risc0-js";

function App(): JSX.Element {
  const [text, setText] = useState("Validating proof...");

  async function fetchBinaryFiles(): Promise<Uint8Array[]> {
    const urls = [
      process.env.PUBLIC_URL + "/receipt.bin",
      process.env.PUBLIC_URL + "/method_id.bin",
    ];

    return Promise.all(
      urls.map((url) =>
        fetch(url).then((response) => {
          if (!response.ok) {
            throw new Error("HTTP error " + response.status);
          }
          return response.arrayBuffer();
        })
      )
    ).then((buffers) => {
      return buffers.map((buffer) => new Uint8Array(buffer));
    });
  }

  useEffect(() => {
    init().then(() => {
      try {
        fetchBinaryFiles().then(([receipt, method_id]) => {
          Receipt.bincode_deserialize(receipt).validate(method_id);
          setText("Proof is valid");
        });
      } catch (e) {
        setText("Error: " + e);
        console.error(e);
      }
    });
  }, []);
  return <div className="App">{text}</div>;
}

export default App;
