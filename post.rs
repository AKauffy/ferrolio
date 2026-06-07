pub mod store {
    use gray_matter::{Matter, engine::YAML};
    use once_cell::sync::OnceCell;
    use pulldown_cmark::{Options, Parser, html};
    use serde::{Deserialize, Serialize};
    use walkdir::WalkDir;

    static POSTS: OnceCell<Vec<Post>> = OnceCell::new();

    /// Full post including rendered HTML body — returned by GET /posts/:slug
    #[derive(Debug, Clone, Serialize)]
    pub struct Post {
        pub slug: String,
        pub meta: PostMeta,
        pub content_html: String,
    }

    /// Metadata only — used in the list response to avoid sending full HTML for every post
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PostMeta {
        pub title: String,
        pub date: String,
        #[serde(default)]
        pub tags: Vec<String>,
        #[serde(default)]
        pub excerpt: Option<String>,
        #[serde(default)]
        pub draft: bool,
    }

    /// Subset returned by GET /posts
    #[derive(Debug, Clone, Serialize)]
    pub struct PostSummary {
        pub slug: String,
        #[serde(flatten)]
        pub meta: PostMeta,
    }

    fn posts_dir() -> std::path::PathBuf {
        std::env::var("POSTS_DIR")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| std::path::PathBuf::from("posts"))
    }

    pub fn load_posts() -> &'static Vec<Post> {
        POSTS.get_or_init(|| {
            let matter = Matter::<YAML>::new();
            let mut posts: Vec<Post> = WalkDir::new(posts_dir())
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path().extension().and_then(|s| s.to_str()) == Some("md")
                })
                .filter_map(|entry| {
                    let path = entry.path();
                    let raw = std::fs::read_to_string(path).ok()?;

                    let parsed = matter.parse(&raw);
                    let meta: PostMeta = parsed.data?.deserialize().ok()?;

                    // Skip drafts in production
                    if meta.draft {
                        return None;
                    }

                    let slug = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("")
                        .to_string();

                    let content_html = markdown_to_html(&parsed.content);

                    Some(Post { slug, meta, content_html })
                })
                .collect();

            // Sort newest first
            posts.sort_by(|a, b| b.meta.date.cmp(&a.meta.date));
            posts
        })
    }

    pub fn all_summaries() -> Vec<PostSummary> {
        load_posts()
            .iter()
            .map(|p| PostSummary {
                slug: p.slug.clone(),
                meta: p.meta.clone(),
            })
            .collect()
    }

    pub fn find_by_slug(slug: &str) -> Option<&'static Post> {
        load_posts().iter().find(|p| p.slug == slug)
    }

    fn markdown_to_html(md: &str) -> String {
        let mut opts = Options::empty();
        opts.insert(Options::ENABLE_STRIKETHROUGH);
        opts.insert(Options::ENABLE_TABLES);
        opts.insert(Options::ENABLE_FOOTNOTES);

        let parser = Parser::new_ext(md, opts);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }
}
