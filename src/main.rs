mod graph;
mod node;
mod visitor;
mod cmd_editor;
fn main() {
    let mut cmd_editor = cmd_editor::CommandLineEditor::new();
    cmd_editor.run();
}