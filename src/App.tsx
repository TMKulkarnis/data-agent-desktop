import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [message, setMessage] = useState("waiting...");

  async function testEngine(){
    setMessage("Asking rust...")
    const response = await invoke("test_polars_connection")
    setMessage(response as string)
  }
  return (
    <div className="container">
      <h1>Data Agent Desktop</h1>
      <div className="card">
        <button onClick={testEngine}>Ignite Engine</button>
      </div>

      <pre style={{textAlign: "left", background: "#f4f4f4", padding: "10px", borderRadius: "8px", color: "black"}}>
        {message}
      </pre>
    </div>
  );
}

export default App;