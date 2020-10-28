use gittemplate::Error;
use gittemplate::EvalContext;
use gittemplate::Expr;
use gittemplate::Result;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

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
        let history_path = Path::new("repl-history.txt");
        let mut host = Host::new(context.clone(), history_path);
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
                host.add_history(&line);
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
    pending_history: Vec<String>,
    history_path: PathBuf,
}

impl Host {
    fn new(context: EvalContext, history_path: &Path) -> Self {
        let mut host = Self {
            context,
            history: Default::default(),
            pending_history: Default::default(),
            history_path: history_path.to_path_buf(),
        };
        host.import_history();
        host
    }

    fn import_history(&mut self) {
        if let Ok(s) = fs::read_to_string(&self.history_path) {
            for line in s.lines() {
                self.history.add(line);
            }
        }
    }

    fn add_history(&mut self, line: &str) {
        self.history.add(line);
        self.pending_history.push(line.to_string());
    }

    fn flush_history(&mut self) {
        if let Ok(mut f) = fs::OpenOptions::new().append(true).open(&self.history_path) {
            for line in &self.pending_history {
                let _ = f.write_all(line.as_bytes());
                let _ = f.write_all(b"\n");
            }
            self.pending_history.clear();
        }
    }
}

impl Drop for Host {
    fn drop(&mut self) {
        self.flush_history()
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
