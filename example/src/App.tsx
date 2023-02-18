import React, { useEffect } from "react";
import "./App.css";

function App() {
  useEffect(() => {
    import("risc0-js").then((module) => {
      module.greet("WebAssembly");
    });
  }, []);
  return <div className="App">Ignore</div>;
}

export default App;
