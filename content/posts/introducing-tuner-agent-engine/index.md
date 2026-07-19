---
title: "Why I Rebuilt My AI Agent Supervisor in Rust: The Journey from Python to Tuner"
date: "2026-07-19"
author: "imwoo90"
description: "An architectural deep-dive into why I deprecated the legacy Python ductor_for_agy framework and rebuilt my autonomous AI agent orchestrator (Tuner) in Rust—achieving zero latency, strict compile-time guidelines, and robust concurrency."
image_url: "tuner_cover.jpg"
tags: ["rust", "python", "antigravity", "architecture", "developer-experience", "telegram"]
---

# Why I Rebuilt My AI Agent Supervisor in Rust: The Journey from Python to Tuner

Every developer building a localized AI assistant knows the feeling: **constant iteration brings complexity**. 

My workspace runs on **Google Antigravity (AGY)**—a powerful agent framework that edits code, runs builds, and assists in my daily workflows. To control and interact with AGY headless over Telegram, I originally developed **ductor_for_agy**, a Python-based daemon that handled lazy-loaded pseudo-terminals (PTYs), concurrency locks, and Telegram markdown formatting.

But as the requirements grew, Python's dynamic runtime began to crack under the weight of Telegram long polling, webhook endpoints, and strict codebase guidelines. 

This is the story of why I abandoned Python and rebuilt my entire agent supervisor from scratch in Rust, creating **Tuner**.

---

## 🛑 The Friction: Why the Python Daemon Scaled Poorly

Python is a fantastic language for prototyping. However, as the orchestrator evolved into a production-grade supervisor, four major pain points emerged:

### 1. The Concurrency Nightmare
To make the AI agent truly useful, the supervisor had to handle several asynchronous loops concurrently:
*   Telegram Long Poll loop for instant messaging responses.
*   Axum-like webhook endpoints listening for CI/CD and system alerts.
*   Background PTY process watchers streaming agent output logs.
*   Cron jobs monitoring daily routines and system status.

Doing this in Python using `asyncio` became highly fragile. Race conditions, occasional deadlocks, and virtual PTY subprocesses leaking into zombie states after abnormal terminations became constant battles.

### 2. Deployment & Virtual Environment Overhead
Running the supervisor on systemd required managing virtual environments, path resolutions, and dependency synchronization. If a system update changed Python paths or library bindings, the systemd user service would silently crash. I wanted a **single, self-contained binary** that could self-install and run with zero external runtime dependencies.

### 3. Type Safety and Refactoring Risk
As we introduced session structures tracking cumulative API token usage (in USD), per-topic LLM configs, localization maps, and DAG (Directed Acyclic Graph) task registries, code refactoring in Python became dangerous. A minor variable rename could slip past runtime testing and cause a crash hours later mid-conversation.

### 4. Compile-Time Enforcement of Design Rules
Our team follows a strict codebase design philosophy: **the source code itself must serve as the documentation**. We set physical limits on complexity—such as restricting single functions to a maximum of 2,000 characters to keep code modular and readable. In Python, checking these constraints required external linters or pre-commit hooks, which could be bypassed or ignored.

---

## 🦀 Enter Rust: Building Tuner

To solve these problems once and for all, I decided to rebuild the orchestrator in Rust. The project, named **Tuner**, was designed as a modular, type-safe, and high-performance agent runtime.

Here is how Rust's ecosystem solved each of our Python bottlenecks:

### 1. Compile-Time Guardrails (`build.rs`)
In Rust, we took our function-size rules and integrated them directly into the compiler via a custom `build.rs` script. 

During the Cargo compilation phase, `build.rs` parses the abstract syntax tree or scans all source files in `src/` to verify function and module boundaries. If it finds a function exceeding 2,000 characters, it triggers a compile error:

```rust
// A snippet of build.rs checking function size limits
fn check_file_limits(path: &Path) -> Result<(), String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    // Scan for function lines and block lengths...
    if function_len > 2000 {
        return Err(format!(
            "Function at {:?} line {} exceeds the 2,000 character limit!",
            path, line_num
        ));
    }
    Ok(())
}
```

This turns coding standards into a compiler-level guarantee. You cannot compile or run the code unless it adheres to our readability guidelines.

### 2. Safe Async Concurrency with Tokio
Rust's `tokio` runtime provided a rock-solid foundation for multi-threaded concurrency. We spawned isolated, lightweight green threads for the Telegram bot client, the cron scheduler, and the Axum API server. 

Because of Rust's strict ownership model and thread-safety checks (`Send` + `Sync`), concurrent write access to the session storage was safely mediated via thread-safe Mutexes and channels, completely eliminating the race conditions we faced in Python.

### 3. PTY Process Supervision & Automatic Cleanup
When the agent executes a command, Tuner spawns a virtual PTY session. In Rust, we wrapped this in an async background task wrapper that streams outputs in real-time, respects timeouts, and captures SIGINT/SIGKILL signals. 

If the user triggers `/abort` or `/stop` via Telegram, the Rust supervisor cleanly terminates the PTY subprocess and its entire PGID (Process Group ID), ensuring no zombie processes are left hanging in memory.

### 4. Zero-Friction Deployment
Deploying Tuner is as simple as running:
```bash
cargo build --release
./target/release/tuner --install-systemd
```
Tuner compiles down to a single binary. The `--install-systemd` flag automatically creates, registers, and starts a systemd user service (`tuner.service`) on the local desktop, pointing directly to the compiled binary. No virtualenvs, no path conflicts, just one binary doing its job.

---

## 📂 The Modular Architecture of Tuner

By starting from scratch, I was able to structure the application into clean, isolated modules:

*   `src/cli/antigravity`: Wraps the `agy` CLI, handles trust verification, parses agent JSON outputs, and discovers available models.
*   `src/background`: Manages virtual PTY runtimes, streams async output buffers, and handles graceful process termination.
*   `src/session`: Serializes conversation states, tracks topic-based memory, and resets limits.
*   `src/webhook`: An Axum server with HMAC-SHA256 signature verification for receiving external payloads securely.
*   `src/tasks`: A DAG (Directed Acyclic Graph) task runner that analyzes dependencies and runs workspace tasks in order.
*   `src/i18n`: Loads TOML-based translation dictionaries dynamically, supporting both English and Korean.

---

## 📈 The Verdict

Rebuilding the system in Rust took effort, but the results speak for themselves:
*   **Zero Latency**: Sub-second responsiveness from a warm PTY session.
*   **Absolute Stability**: Over 460 unit and integration tests verify session states, formatting, and webhook signatures.
*   **Guaranteed Quality**: The compiler itself prevents code bloat, keeping functions concise and maintainable.
*   **Low Footprint**: Memory usage dropped significantly compared to the Python virtualenv runtime.

Migrating from Python to Rust wasn't just about speed. It was about creating an agent supervisor that is as robust and bulletproof as the compiled code it helps write.
