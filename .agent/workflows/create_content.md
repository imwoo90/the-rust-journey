---
description: Create a new blog post or project entry with the "Rust Journey" style (Story-driven Post / Technical Project)
---

This workflow standardizes the creation of new content for "The Rust Journey" blog.

# 1. Analyze and Setup
1.  **Identify Type**:
    *   **Post**: A narrative, story-driven blog entry.
    *   **Project**: A technical, factual project showcase.
2.  **Determine Slug**: Convert the title to kebab-case (e.g., "Web Serial Monitor" -> `web-serial-monitor`).
3.  **Create Directory**:
    *   Post: `public/content/posts/<slug>`
    *   Project: `public/content/projects/<slug>`

# 2. Creates Directory
// turbo
Create the directory using `mkdir -p <full_path>`.

# 3. Content Strategy

## If Type is "Post" (Story Mode)
Write the `index.md` with a **"Hero's Journey"** structure:
*   **The Hook**: Start with a personal frustration or ambitious goal.
*   **The Challenge**: Explain why this was hard (e.g., "Browsers are single-threaded").
*   **The Solution (Chapters)**: Break down technical wins (e.g., "Chapter 1: Multi-threading").
*   **The Reality Check**: A section explaining non-obvious struggles or compromises (e.g., "Why I used JS instead of Rust").
*   **Conclusion**: What was learned.

## If Type is "Project" (Tech Spec Mode)
Write the `index.md` with a **"Technical Documentation"** structure:
*   **Introduction**: Short and professional.
*   **Key Features**: Bullet points with emojis (🚀, 🛠️).
*   **Technical Architecture**: Explain the stack and design choices clearly.
*   **How to Run**: Simple, copy-pasteable commands.

# 4. Frontmatter Standard
Ensure every file starts with:
```yaml
---
title: "..."
date: "YYYY-MM-DD"
author: "imwoo90"
description: "..."
image_url: "<slug>_cover.png" # or _project.png
tags: ["rust", "dioxus", ...]
---
```

# 5. Visuals
1.  Attempt to generate a "Cyberpunk/Technical" style cover image.
2.  If generation fails, instruct the user to place the image at `<full_path>/<slug>_cover.png`.
