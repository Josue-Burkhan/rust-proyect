const { invoke } = window.__TAURI__.tauri;

const analyzeBtn = document.querySelector("#analyze-btn");
const logOutput = document.querySelector("#log-output");

analyzeBtn.addEventListener("click", async () => {
    logOutput.textContent = "Analizando...";
    try {
        const logs = await invoke("analyze_log_file", { path: "test.log" });
        logOutput.textContent = logs.join("\n");
    } catch (error) {
        logOutput.textContent = `Error: ${error}`;
    }
});