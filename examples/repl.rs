use gittemplate::EvalContext;
use gittemplate::Expr;
use gittemplate::Result;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env;
use std::io;

fn eval_single(context: &EvalContext, code: &str, out: &mut dyn io::Write) -> bool {
    let debug = std::env::var("DEBUG").is_ok();
    let result: Result<()> = (|| {
        let ast = Expr::parse_incomplete(code)?;
        if debug {
            eprintln!("# AST: {:?}", &ast);
        }
        let value = context.eval(ast)?;
        value.write_to(out)?;
        Ok(())
    })();
    match result {
        Ok(()) => true,
        Err(e) => {
            eprintln!("{}", e);
            false
        }
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
        let history_file = "repl-history.txt";
        let mut rl = Editor::<()>::new();
        let _ = rl.load_history(history_file);
        loop {
            let code = rl.readline("> ");
            match code {
                Ok(line) => {
                    if !line.is_empty() {
                        rl.add_history_entry(line.as_str());
                        eval_single(&context, &line, &mut out);
                    }
                }
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
                Err(err) => {
                    eprintln!("{:?}", err);
                    break;
                }
            }
        }
        let _ = rl.save_history(history_file);
    } else {
        for code in args {
            eval_single(&context, &code, &mut out);
        }
    }
}
