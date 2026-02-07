---
title: "Pushing Browsers to the Limit: Building a Web Serial Monitor with Rust & WASM"
date: "2026-01-18"
author: "imwoo90"
description: "A deep dive into my second major Rust project: engineering a high-performance, installation-free Serial Monitor using Dioxus, Web Workers, and OPFS."
image_url: "web_serial_monitor_cover.png"
tags: ["rust", "dioxus", "wasm", "web-serial", "opfs", "terminal", "serial"]
---

"Why do I need to install a 500MB driver just to read `Hello World` from a microcontroller?"

This question sparked a journey that pushed the boundaries of what I thought was possible in a web browser. As an embedded developer, I've spent countless hours wrestling with fragmented desktop serial monitors. My dream was simple yet audacious: **A Serial Monitor that runs instantly in Chrome, handles gigabytes of logs, and feels as snappy as a native C++ app.**

No installers. No drivers. Just a URL.

But to achieve this, I had to break the rules of web development. Here is the story of how Rust, WebAssembly, and a team of AI agents helped me build **RusTerm**.

## The Challenge: Native Performance in a Sandbox

The browser is traditionally seen as a single-threaded sandbox, ill-suited for handling high-frequency hardware interrupts or massive log streams. If you throw 115,200 characters per second at a standard DOM, it will choke.

My requirements were non-negotiable:
1.  **Zero Installation**: Absolute accessibility.
2.  **Gigabyte-Scale Logging**: Run for 24 hours without crashing.
3.  **Fluid UI**: No freezing, even during high-traffic bursts.

To make this happen, I had to architect a system that mimicked a desktop application's threading model, right inside the browser.

## 🏗️ Chapter 1: Escaping the Main Thread

The first hurdle was the UI. In a typical web app, if your parsing logic takes too long, the UI frame drops. With serial data arriving every millisecond, doing work on the main thread was a death sentence for performance.

I leveraged **Web Workers** to create a true multi-threaded environment:

-   **The Main Thread (The Face)**: This is the Dioxus UI. It is purely reactive. It cares only about rendering what the user currently sees. It uses Dioxus Signals for state management but touches *no* heavy data.
-   **The Worker Thread (The Brain)**: This is where the heavy lifting happens. It manages the `SerialPort` stream, regex parsing, and file I/O.

Rust made this separation seamless. Sharing data structures between the WASM UI and the Worker was type-safe and robust, ensuring data integrity across the boundary.

## 💾 Chapter 2: The Infinite Memory Hack

Javascript arrays are not infinite. If you try to store a few hundred megabytes of strings in an array, the browser tab will crash. I needed a way to store GBs of data persistently *inside* the browser without blowing up the heap.

Enter the **Origin Private File System (OPFS)**.

OPFS is a game-changer. Unlike the slow and limited `localStorage`, OPFS allows for high-performance, synchronous file access within Workers.

-   **The Solution**: I implemented a streaming writer that pipes serial data directly into a virtual file handle.
-   **The Result**: The application's memory usage remains flat—hovering around 50MB—whether we've logged 1,000 lines or 10,000,000 lines. We effectively bypassed the browser's memory limits by using the disk.

## ⚡ Chapter 3: Rendering the Invisible

Storing millions of lines is one thing; showing them is another. Rendering 1 million `<div>` elements would instantly freeze any computer.

To solve this, I implemented a **Virtual Scroller**.

Dioxus shines here. Its diffing algorithm is incredibly fast.
1.  We calculate the visible viewport height.
2.  We mathematically determine which lines (e.g., lines 50,000 to 50,050) are currently visible.
3.  We slice *only* those bytes from the OPFS file and render them.

The DOM only ever contains about 30 elements. As you scroll, the content is swapped out instantly. The user feels like they are scrolling through an infinite document, but the browser is barely doing any work.

## 🤝 The Architects: A Symphony of Agents

This project wasn't built in isolation. It was a collaboration between human intent and AI execution.

1.  **Stitch (The Designer)**: I started by visualizing the interface. I wanted a "Cyberpunk Terminal" aesthetic—dark, monospaced, and utilitarian. Stitch helped generate the high-fidelity CSS/HTML mockups that set the tone.
2.  **Jules (The Integrator)**: Once the design was ready, Jules acted as the bridge, integrating the raw design tokens into the Dioxus component structure, ensuring the code matched the vision.
3.  **Antigravity (The Engineer)**: Finally, Antigravity handled the heavy lifting. While I defined the architecture (Worker/OPFS split), Antigravity implemented the complex glue code—generating `wasm-bindgen` definitions and implementing lock-free data structures to pass data between threads.

It was pair programming at the speed of thought.

## ⚠️ Reality Check: The Pragmatic Pivot

As an embedded developer, I wanted to "conquer the web" using purely Rust. I wanted to prove that a system-level language could completely abstract away the quirks of web development.

But the web has its own gravity.

I encountered a hard truth: **Some things are still better in JavaScript.**

The core logic for buffering, string parsing, and OPFS file I/O eventually moved to `assets/js/log_worker.js`.
Why?
1.  **Direct API Access**: Accessing OPFS `createSyncAccessHandle` and complex stream manipulations from Rust/WASM involved creating so much boilerplate that it defeated the purpose.
2.  **Copy Overhead**: Passing massive strings back and forth between Rust's linear memory and the JavaScript engine incurred a performance penalty.

So, I compromised. Rust orchestrates the state and the UI, while a highly optimized JavaScript worker handles the raw data bandwidth. It’s a hybrid approach that plays to the strengths of both languages.

## Conclusion: The New Standard

This project proved that **Rust + WebAssembly is a paradigm shift**. We are no longer limited to "lightweight" web apps. We can bring heavy, system-level utilities to the web without sacrificing performance.

Building this monitor expanded my understanding of:
-   **Browser Internals**: The delicate dance between the Event Loop and Web Workers.
-   **Rust's Ownership**: How it keeps architecture clean, even in a distributed WASM environment.
-   **Stream Processing**: How to handle infinite data without infinite memory.

If you are tired of installing serial drivers, try the official app here: [RusTerm](https://imwoo90.github.io/RusTerm/).

The journey implies that the web is ready for embedded development. And I'm just getting started.

---
*Check out the [Project Source Code](https://github.com/imwoo90/rusterm) to see the code behind the story.*
