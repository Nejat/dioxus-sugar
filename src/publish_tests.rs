#[test]
fn readme_usage_version() {
    version_sync::assert_markdown_deps_updated!("README.md");
}

#[test]
fn readme_docs_link_version() {
    version_sync::assert_contains_regex!("README.md", "/dioxus-sugar/{version}/dioxus_sugar/");
}

#[test]
fn readme_examples_link_version() {
    version_sync::assert_contains_regex!("README.md", "/v{version}/examples");
}

#[test]
fn html_root_url() {
    version_sync::assert_html_root_url_updated!("src/lib.rs");
}
