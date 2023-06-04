use honggfuzz::fuzz;
use alumina_boot::{diagnostics::DiagnosticContext, parser::ParseCtx};

fn main() {
    let diag: DiagnosticContext = DiagnosticContext::default();
    loop {
        fuzz!(|data: &[u8]| {
            let src: String = std::str::from_utf8(data).unwrap().to_string();
            let file_id = diag.make_file_id();
            let parse_tree = ParseCtx::from_source(file_id, src);
            let _ = parse_tree.check_syntax_errors(parse_tree.root_node());
        });
    }
}
