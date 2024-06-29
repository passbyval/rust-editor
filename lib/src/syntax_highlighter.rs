use cached::proc_macro::cached;
use eframe::egui::text::LayoutJob;
use eframe::egui::{Color32, FontId, TextFormat};
use tree_sitter_highlight;
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};
use tree_sitter_javascript;
use tree_sitter_typescript;

#[cached]
pub fn highlight(code: String) -> egui::text::LayoutJob {
    let mut highlighter = Highlighter::new();
    let mut job: LayoutJob = egui::text::LayoutJob::default();
    let bytes = code.as_bytes();

    let ast_token_types = [
        ("attribute", "#9cdcfe"),
        ("constant", "#569cd6"),
        ("function.builtin", "#C8C8C8"),
        ("function", "#C8C8C8"),
        ("keyword", "#569CD6"),
        ("operator", "#b4b4b4"),
        ("property", "#DADADA"),
        ("punctuation", "#b4b4b4"),
        ("punctuation.bracket", "#b4b4b4"),
        ("punctuation.delimiter", "#b4b4b4"),
        ("string", "#ce9178"),
        ("string.special", "#d16969"),
        ("tag", "#569cd6"),
        ("type", "#4EC9B0"),
        ("type.builtin", "#4EC9B0"),
        ("variable", "#C8C8C8"),
        ("variable.builtin", "#C8C8C8"),
        ("variable.parameter", "#7F7F7F"),
        ("variable.builtin", "#C8C8C8"),
        ("comment", "#6A9955"),
    ];

    let mut html_config = HighlightConfiguration::new(
        tree_sitter_html::language(),
        tree_sitter_html::HIGHLIGHTS_QUERY,
        tree_sitter_html::INJECTIONS_QUERY,
        "",
    )
    .unwrap();

    let mut javascript_config = HighlightConfiguration::new(
        tree_sitter_javascript::language(),
        tree_sitter_javascript::HIGHLIGHT_QUERY,
        tree_sitter_javascript::INJECTION_QUERY,
        tree_sitter_javascript::LOCALS_QUERY,
    )
    .unwrap();

    let mut typescript_config = HighlightConfiguration::new(
        tree_sitter_typescript::language_typescript(),
        tree_sitter_typescript::HIGHLIGHT_QUERY,
        "",
        tree_sitter_javascript::LOCALS_QUERY,
    )
    .unwrap();

    let &names = &ast_token_types.map(|p| p.0);

    print!("{:?}", &names);

    javascript_config.configure(&names);
    typescript_config.configure(&names);
    html_config.configure(&names);

    let events = highlighter
        .highlight(&javascript_config, &bytes, None, |_| None)
        .unwrap();

    let mut color: &str = "#6A9955";

    for event in events {
        match event.unwrap() {
            HighlightEvent::Source { start, end } => {
                let text = String::from_utf8(bytes.to_vec()).expect("Found invalid UTF-8");

                job.append(
                    &text[start..end],
                    0.0,
                    TextFormat {
                        color: Color32::from_hex(&color).unwrap(),
                        font_id: FontId {
                            family: egui::FontFamily::Monospace,
                            ..FontId::default()
                        },
                        ..TextFormat::default()
                    },
                )
            }
            HighlightEvent::HighlightStart(s) => color = &ast_token_types[s.0].1,
            HighlightEvent::HighlightEnd => color = "#6A9955",
        }
    }

    job
}
