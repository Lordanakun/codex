use super::*;
use pretty_assertions::assert_eq;

#[test]
fn render_plugin_instructions_returns_none_for_empty_plugins() {
    assert_eq!(render_plugin_instructions(&[]), None);
}
