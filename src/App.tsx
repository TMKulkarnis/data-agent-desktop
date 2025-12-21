import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import "./App.css";

function App() {
  const [msg, setMsg] = useState("Waiting for command...");

  async function testEngine() {
    setMsg("Asking Rust...");
    const response = await invoke("test_polars_connection");
    setMsg(response as string);
  }

  async function openFile() {
    try {
      const file = await open({
        multiple: false,
        filters: [{ name: 'CSV Files', extensions: ['csv'] }]
      });

      if (!file) return;

      setMsg(`Reading file: ${file}...`);
      
      // Send the path to Rust
      const result = await invoke("load_csv", { path: file });
      setMsg(result as string);
      
    } catch (e) {
      setMsg(`Error: ${e}`);
    }
  }

  return (
    <div className="container">
      <h1>Data Agent Desktop</h1>
      
      <div className="row">
        <button onClick={testEngine}>
          Ignite Engine 🚀
        </button>

        <button onClick={openFile} style={{marginLeft: "10px"}}>
          Open CSV File
        </button>
      </div>

      <pre style={{ 
        textAlign: "left", 
        background: "#f4f4f4", 
        padding: "10px", 
        borderRadius: "8px", 
        color: "black", 
        overflow: "auto", 
        maxHeight: "400px", 
        marginTop: "20px",
        whiteSpace: "pre-wrap"
      }}>
        {msg}
      </pre>
    </div>
  );
}

export default App;