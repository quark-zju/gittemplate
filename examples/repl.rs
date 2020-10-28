use gittemplate::Error;
use gittemplate::EvalContext;
use gittemplate::Expr;
use gittemplate::Result;
use std::env;
use std::io;
use std::io::Write;

use termwiz::cell::AttributeChange;
use termwiz::color::AnsiColor;
use termwiz::lineedit::*;

fn eval_single(
    context: &EvalContext,
    code: &str,
    out: &mut dyn io::Write,
    print_ast: bool,
) -> Result<()> {
    let ast = Expr::parse_incomplete(code)?;
    if print_ast {
        eprintln!("# AST: {:?}", &ast);
    }
    let value = context.eval(ast)?;
    value.write_to(out)?;
    Ok(())
}

fn try_eval_single(context: &EvalContext, code: &str, out: &mut dyn io::Write) {
    let debug = std::env::var("DEBUG").is_ok();
    match eval_single(context, code, out, debug) {
        Err(e) => {
            eprint!("{}", e);
        }
        Ok(_) => {}
    }
}

fn main() {
    let mut context = EvalContext::new().with_colors_by_tty();
    if let Ok(repo) = git2::Repository::open_from_env() {
        context = context.with_global_object(repo);
    }
    let mut out = std::io::stdout();
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        // REPL
        let mut terminal = line_editor_terminal().unwrap();
        let mut editor = LineEditor::new(&mut terminal);
        let mut host = Host::new(context.clone());
        loop {
            if let Ok(Some(line)) = editor.read_line(&mut host) {
                if &line == "exit" {
                    break;
                }
                if !line.is_empty() {
                    try_eval_single(&context, &line, &mut out);
                    let _ = out.write_all(b"\n");
                    let _ = out.flush();
                }
            } else {
                break;
            }
        }
    } else {
        for code in args {
            try_eval_single(&context, &code, &mut out);
        }
    }
}

struct Host {
    history: BasicHistory,
    context: EvalContext,
}

impl Host {
    fn new(context: EvalContext) -> Self {
        Self {
            context,
            history: Default::default(),
        }
    }
}

impl LineEditorHost for Host {
    fn render_prompt(&self, prompt: &str) -> Vec<OutputElement> {
        vec![
            OutputElement::Attribute(AttributeChange::Foreground(AnsiColor::Navy.into())),
            OutputElement::Text(prompt.to_owned()),
        ]
    }

    fn history(&mut self) -> &mut dyn History {
        &mut self.history
    }

    fn complete(&self, line: &str, cursor_position: usize) -> Vec<CompletionCandidate> {
        let mut candidates = Vec::new();
        let len = line.len();
        // Complete after "." at the end.
        if line.len() == cursor_position && line.ends_with(".") {
            // Write to `null` to trigger lazy evaluation.
            let mut null = Vec::<u8>::new();
            let result = eval_single(&self.context, line, &mut null, false);
            match result {
                Err(Error::MissingingSymbol(hints)) => {
                    for hint in hints {
                        candidates.push(CompletionCandidate {
                            range: len..len,
                            text: hint,
                        });
                    }
                }
                _ => {}
            }
        }
        candidates
    }
}
