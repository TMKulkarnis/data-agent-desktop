import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import "./App.css";

function App() {
  const [msg, setMsg] = useState("Waiting for command...");
  const [currentFile, setCurrentFile] = useState("")
  const [query, setQuery] = useState("SELECT * FROM data LIMIT 5");
  const[Url, setUrl] = useState("");

  async function openFile() {
    try {
      const file = await open({
        multiple: false,
        filters: [{ name: 'CSV Files', extensions: ['csv','json', 'parquet', 'xlsx', 'xls', 'tsv'] }]
      });

      if (!file) return;

      // SAVE THE PATH: We store it in state so 'runQuery' can use it.
      setCurrentFile(file as string);
      setMsg(`File Selected: ${file}\nReady to Query.`);
      
    } catch (e) {
      setMsg(`Error: ${e}`);
    }
  }

  // Run the query
  async function runQuery() {
    if (!currentFile) {
      setMsg("Please open a CSV file first.");
      return;
    }
    setMsg("running query...");
    const result = await invoke("query_table", {path: currentFile, query: query});
    setMsg(result as string);
   }

   // Fetch file from URL and save locally
   async function fetchUrl(){
    if (!Url) return;
    setMsg("Fetching file from URL...");

    const result = await invoke("load_url", {url: Url});
    setCurrentFile(result as string);
    setMsg(`File downloaded from URL and saved to: ${result}\nReady to Query.`);
   }

  return (
<div className="container" style={{padding: "20px"}}>
      <h1>Data Agent: SQL Engine </h1>
      
      {/*The File Loader */}
      <div className="row" style={{marginBottom: "20px"}}>
        <button onClick={openFile}> Open file</button>
        <span style={{marginLeft: "10px", fontSize: "12px", color: "#888"}}>
            {currentFile ? "File Loaded " : "No file selected"}
        </span>
      </div>

      {/* The Query Box */}
      <div style={{display: "flex", flexDirection: "column", gap: "10px"}}>
        <label style={{textAlign: "left"}}>SQL Query (Table name is <b>'data'</b>):</label>
        <textarea 
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            rows={3}
            style={{
                width: "100%", 
                padding: "10px", 
                fontFamily: "monospace",
                background: "#2a2a2a",
                color: "#fff",
                border: "1px solid #444"
            }}
        />
        <button onClick={runQuery} style={{alignSelf: "flex-start"}}>
            Run Query
        </button>
      </div>
      <div style={{display: "flex", gap: "10px", marginBottom: "20px", width: "100%"}}>
          <input 
              onChange={(e) => setUrl(e.currentTarget.value)} 
              placeholder="Paste Cloud Link (e.g. https://site.com/data.csv)" 
              style={{
                  flex: 2, 
                  padding: "12px", 
                  borderRadius: "8px", 
                  border: "1px solid #555",
                  backgroundColor: "#222",
                  color: "white"
              }}
          />
          <button onClick={fetchUrl} style={{flex: 1, backgroundColor: "#6C5CE7"}}>
              Fetch URL
          </button>
      </div>

      {/* The Results Console */}
      <h3 style={{textAlign: "left", marginTop: "20px"}}>Results:</h3>
      <pre style={{ 
        textAlign: "left", 
        background: "#1e1e1e", 
        color: "#00ff00", 
        padding: "15px", 
        borderRadius: "8px", 
        overflow: "auto", 
        maxHeight: "400px", 
        whiteSpace: "pre-wrap", 
        fontFamily: "Consolas, 'Courier New', monospace"
      }}>
        {msg}
      </pre>
    </div>
  );
}

export default App;