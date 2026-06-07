# portfolio-backend

Axum-based blog API for `portfolio.kauffy.dev`. Parses Markdown files with YAML frontmatter at startup and serves them as JSON.

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/posts` | All post summaries (no body HTML) |
| GET | `/posts/:slug` | Full post with rendered HTML |

### Response shapes

**GET /posts**
```json
{
  "posts": [
    {
      "slug": "my-first-post",
      "title": "My First Post",
      "date": "2026-06-01",
      "tags": ["rust", "axum"],
      "excerpt": "A short description.",
      "draft": false
    }
  ]
}
```

**GET /posts/:slug**
```json
{
  "slug": "my-first-post",
  "meta": { "title": "...", "date": "...", "tags": [], "excerpt": "...", "draft": false },
  "content_html": "<h2>Why Rust?</h2><p>...</p>"
}
```

## Writing a post

Create a `.md` file in `posts/` with YAML frontmatter:

```markdown
---
title: "My Post Title"
date: "2026-06-07"
tags: ["tag1", "tag2"]
excerpt: "One sentence description shown in listings."
draft: false
---

Your Markdown content here.
```

- **slug** is derived from the filename (`my-post-title.md` → slug `my-post-title`)
- Posts with `draft: true` are excluded at startup
- Posts are sorted newest-first by `date`

## Running locally

```bash
cargo run
# Server starts on http://localhost:8080
```

## Environment variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `8080` | Server port |
| `POSTS_DIR` | `posts` | Path to Markdown files |
| `ALLOWED_ORIGIN` | `http://localhost:3000` | CORS allowed origin |
| `RUST_LOG` | `portfolio_backend=debug` | Log level |

## Docker

```bash
docker build -t portfolio-backend .
docker run -p 8080:8080 portfolio-backend
```

For Nginx Proxy Manager, proxy to `http://<container>:8080` and set `ALLOWED_ORIGIN=https://portfolio.kauffy.dev`.
