---
title: "Upgrading Ductor for Google Antigravity: PTY Warm Sessions, Sync Locks, and UX Polish"
date: "2026-07-10"
author: "imwoo90"
description: "How I fork-engineered the Ductor bot daemon to support persistent PTY terminal runtimes, fix concurrent logging race conditions, and collapse messy agent traces for a streamlined Telegram workspace."
image_url: "upgrading_ductor_cover.jpg"
tags: ["rust", "python", "telegram", "antigravity", "pty", "developer-experience"]
---

# Upgrading Ductor for Google Antigravity: PTY Warm Sessions, Sync Locks, and UX Polish

Every developer building localized AI agents faces the same frustrating friction point: **latency and state loss**.

In my local workspace, I run **Google Antigravity (AGY)**—a powerful agent framework that manages my coding tasks, handles domestic database assets, and updates my git repos. But running it via terminal hooks was painful. Every request required initializing a new shell, parsing rules, and waiting 30+ seconds for the agent to boot up. Even worse, if my Google Cloud session timed out, I’d lose all state.

To solve this, I took **Ductor** (my Telegram-based orchestrator), forked it into [ductor_for_agy](https://github.com/imwoo90/ductor_for_agy), and engineered a pseudo-terminal (PTY) session manager, a synchronization lock to prevent race conditions, and a custom Markdown parser to keep my mobile chat clean.

Here is the story of how I did it.

---

## 🎣 The Hook: The Annoyance of Cold Start Latency

AI agents are only useful if they respond quickly. When you run a command like `agy` directly in a shell script, the Python runtime must load, search the workspace, locate the active rules, and connect to its backends. By the time the agent is ready to "think," 15 seconds have passed.

Additionally, because the orchestrator was polling the workspace, a session disconnect or cloud timeout would completely disrupt running tasks. I needed the agent to be **always hot**—idle but instantly ready, holding its context in memory.

---

## 🚧 The Challenge: Terminal Emulation and Double Messages

To keep the agent "hot," I needed to run it in interactive mode (`agy --prompt-interactive`). But interactive prompts expect a standard terminal keyboard input (stdin) and output (stdout). If you redirect these in a basic background subprocess, the interactive shell immediately hangs or crashes.

Additionally, as soon as I started streaming background updates to Telegram, a second bug emerged: the **Double Response Glitch**. 

Here is what went wrong:
1. The Telegram bot received a user request and executed a command synchronously.
2. At the same time, the background log monitor thread polled the agent's `transcript.jsonl` file.
3. The log monitor saw new lines being written, grabbed them, and pushed them to Telegram.
4. The user received two identical copies of the response—one from the synchronous reply, and one from the async log watcher.

---

## 🛠️ The Solution

### Chapter 1: The PTY Keep-Warm Solution

To trick the interactive shell into thinking it was connected to a real terminal, I used Python's `pty` library to spawn a pseudo-terminal interface. 

I wrote a persistent session manager in `cli/antigravity_provider.py` that boots `agy` on a warm pseudo-terminal:

```python
import pty
import os
import subprocess

# Spawning agy on a pseudo-terminal to keep it warm
master_fd, slave_fd = pty.openpty()
process = subprocess.Popen(
    ["agy", "--prompt-interactive"],
    stdin=slave_fd,
    stdout=slave_fd,
    stderr=slave_fd,
    close_fds=True,
    preexec_fn=os.setsid
)
```

By disabling terminal ECHO and polling the `master_fd` asynchronously, the orchestrator keeps the session active 24/7. When a new user prompt arrives, it is written directly to the PTY input stream, dropping the boot latency from **30 seconds to less than 1 second**.

### Chapter 2: The Concurrency Lock

To fix the double-messaging bug, I introduced a thread synchronization lock (`_sync_in_progress`) in the bot daemon (`telegram/app.py`).

When a synchronous action is executing:
1. The bot sets the `_sync_in_progress` lock to `True`.
2. The background log monitor thread is instructed to skip reading the log updates while the lock is active.
3. Once the synchronous command completes, the final file size of `transcript.jsonl` is saved in `_processed_log_sizes`, and the lock is released.

This guarantees that the log monitor only captures updates generated during *asynchronous background tasks* (like long-running builds or multi-agent research runs), leaving direct conversations completely clean.

### Chapter 3: Collapsible Thinking and Tool Traces

AI agents output a massive volume of "thinking" logs and tool executions. On a mobile phone screen, scrolling through 200 lines of regex searches and raw tool parameters is a nightmare.

Telegram supports collapsible quote blocks and spoilers. I built a custom parser inside `LogParser` that parses the raw JSON lines of `transcript.jsonl` and wraps thoughts and tool runs in collapsible blocks:

```python
def format_thoughts(thoughts: str) -> str:
    # Telegram markdown spoiler syntax wrapper
    return f"💭 **Thinking Process**\n||{thoughts}||"
```

Now, the entire reasoning path is neatly tucked away. If a build fails, I can tap to expand the details, but my default chat view remains clean and readable.

### Chapter 4: Agent-Driven Artifact Delivery

When Antigravity generates structured markdown files (like a `/plan` or `analysis_results.md`), they are saved deep in the local `.gemini/antigravity-cli/brain/` directory.

To prevent having to SSH into my machine or open a separate editor to read them, I originally implemented an automatic directory watcher in `antigravity_provider.py` to copy files to `output_to_user/` and auto-attach them. However, to make the process more explicit and avoid unnecessary file system overhead, I refactored this into an **agent-driven approach**.

Through system rules (`AGENTS.md`), the agent itself is instructed to output the absolute path tag (e.g., `<file:/home/wimvm/.gemini/antigravity-cli/brain/session_id/artifact.md>`) at the end of its response. The Telegram messenger parser detects this tag and directly streams the file to the user's chat.

### Chapter 5: Lifecycle Recovery and UX Polish

To make the bot daemon production-ready, I added a few more crucial UX enhancements:
1. **Last-Active Chat Recovery**: The bot records the active chat and thread ID to `last_active_chat.json`. When systemd restarts the service, the bot sends a recovery notification ("*Bot Restarted — back online*") directly to the active thread, avoiding global broadcast spam.
2. **Silent Error Logging**: Rather than posting ugly `[File not found]` or `[Failed to send]` errors into the user chat when a file path is incorrect, these exceptions are silenced on Telegram and logged internally to the console.
3. **HTML Message Splitting**: Fixed a layout bug where long messages split by the Telegram message chunker would break HTML/Markdown block boundaries.

### Chapter 6: Taming the TUI — Non-Blocking Telegram Planning

One major road block in running terminal-native agents headlessly is **interactive user prompts**. During commands like `/plan`, the Antigravity CLI spins up a Bubbletea-based terminal UI (TUI) to ask the user multiple-choice questions (e.g., tech stack, design preferences). 

In a headless Telegram environment, the subprocess runs with `stdin=subprocess.DEVNULL`. When the agent tries to call the interactive `ask_question` tool, it hits an immediate EOF error. To bypass the crash, the model would simply auto-guess the tech stack and complete the plan without the user's input.

My first instinct was to route Telegram responses directly to the pseudo-terminal's standard input. But this is highly fragile: it requires keeping the backend subprocess alive and hanging in memory for minutes or hours while waiting for a user to text back on Telegram, risking resource leaks and timeouts.

Instead, I designed a **non-blocking, instruction-based TUI fallback**. By writing a strict guideline in the agent's workspace rules (`RULES-antigravity.md`), we instruct the LLM:
*   **Do NOT** call the interactive `ask_question` tool.
*   Instead, output your questions and options directly as a clean Markdown text response, and immediately **terminate the turn**.

Now, the agent cleanly exits and releases resources. Once the user replies on Telegram, the next turn starts with `--continue`, and the user's choices are fed directly into the model's new prompt context. This enables clean, multi-turn interactive dialogues without holding processes hostage.

---

## 🎯 The Reality Check: Telegram's Formatting Nightmares

Implementing this wasn't all smooth sailing. Telegram's Markdown V2 parser is extremely sensitive. A single unescaped square bracket `[` or underline `_` in a code block or thought log will cause Telegram to reject the entire HTTP request.

I had to build a robust escaping wrapper to sanitize markdown tokens while preserving formatting boundaries for code and spoilers. It took multiple regex iterations to get right, but the result is a bulletproof rendering pipeline.

---

## 🏁 Conclusion

By building `ductor_for_agy`, my workspace has transformed. I now have:
- **Zero latency**: Instant responses from a warmed-up Antigravity agent.
- **Clean interface**: Collapsible logs that keep my mobile screen clean.
- **Robust workflows**: Explicit artifact attachments and self-healing system recovery notifications.

It is a blueprint for how developers can adapt general-purpose AI daemons into custom, high-speed, localized workspaces.
