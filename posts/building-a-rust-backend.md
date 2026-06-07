---
title: "Building a Rust Backend for My Portfolio"
date: "2026-06-01"
tags: ["rust", "axum", "backend"]
excerpt: "Why I chose Rust and Axum for my portfolio's blog API."
draft: false
---

## Why Rust?

I wanted something lightweight and fast that compiles to a single binary — perfect for a homelab Docker deployment.

## The Stack

- **Axum** for routing
- **pulldown-cmark** for Markdown rendering
- **gray_matter** for frontmatter parsing

## Writing Posts

Just drop a `.md` file in the `posts/` directory and redeploy. No database, no CMS, no fuss.
