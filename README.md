🦀 Rust-Rust: A Native Rust Learning App, Built in Rust
An interactive, fully Rust-based learning platform for mastering Rust — with lessons, quizzes, live code execution (no Docker), and AI-powered explanations using LiteLLM or any OpenAI-compatible LLM.

✅ Core Features
📘 1. Lessons
Markdown-based modules, ranging from beginner to advanced.

Syntax-highlighted embedded code snippets.

Expandable with auto-generated examples via LLM.

❓ 2. Interactive Questions
MCQs, short answer, and code completion.

Smart validation and feedback via LLM.

Show explanations, corrections, and hints.

⚙️ 3. Code Execution
Runs user Rust code:

Prefer WASM-based execution (via wasmer, wasmtime, or wasm-bindgen).

Fallback to native in-process rustc execution with:

Sandboxed Command runner.

Temp file compile + capture output/errors.

No Docker involved.

Optionally use Firecracker VM later for secure cloud exec (if scaling).

🤖 4. LLM Integration
Uses LiteLLM as the default backend.

Configurable model source (OpenAI, Ollama, LocalAI, Together.ai).

Used for:

Generating questions.

Correcting code.

Explaining answers.

Providing concept summaries.

🛠️ Stack
🧩 Backend
Component	Tech Used
Web server	axum or actix-web
Code execution	wasmer, wasmtime, or Command::new("rustc") with temp files
LLM interface	ureq or reqwest calling LiteLLM API
Markdown engine	pulldown-cmark
Auth/local save	Localstore/SQLite or flat files for MVP

💻 Frontend Options
Choose based on your target:

Web app: Leptos or Yew (WASM)

Desktop: Tauri (Rust + WebView)

CLI-first MVP: tui-rs for terminal UI

🔐 Code Execution Design (No Docker)
Option A: WASM Execution
Use wasmer or wasmtime to run compiled Rust → WASM.

Compile exercises to .wasm ahead-of-time.

User code inserted into defined placeholders.

Execute in-memory securely.

Option B: Native rustc
rust
Copy
Edit
Command::new("rustc")
    .arg("temp_user_code.rs")
    .arg("-o")
    .arg("temp_user_binary")
    .output()
Runs only locally with your rustc installed.

No external sandbox, so suitable for self-hosted or trusted users.

⚙️ LiteLLM Integration
Config
toml
Copy
Edit
[llm]
provider = "openai" # or "ollama", "togetherai", etc.
api_key = "your-key-here"
base_url = "http://localhost:4000"  # default LiteLLM proxy URL
Code Snippet
rust
Copy
Edit
let prompt = "Explain what Rust lifetimes are in simple terms.";
let response = send_prompt_to_llm(prompt).await?;
println!("{}", response);
Use reqwest to call POST /v1/chat/completions.

📁 Suggested Project Structure
bash
Copy
Edit
rust-rust/
├── backend/
│   ├── main.rs                # API + execution logic
│   ├── exec/                  # Rust code runner (wasm/native)
│   ├── ai/                    # LLM interface (LiteLLM)
│   └── content/               # Lessons, questions
├── frontend/                  # Yew or Tauri frontend
├── config.toml               # LLM & execution settings
├── Cargo.toml
🚀 Development Plan (Phased)
Phase	Milestone
✅ 1	CLI app MVP: Lessons + MCQs + Code exec
✅ 2	Add LLM-powered answer checking
🔄 3	Web frontend (Yew/Leptos or Tauri)
🔄 4	Progress tracking and user sessions
🔄 5	Packaging and release as offline-native app