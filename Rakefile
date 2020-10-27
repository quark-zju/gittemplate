task default: %w[build]

GRAMMAR_SRC = 'src/parser/grammar.lalrpop'
GRAMMAR_DST = GRAMMAR_SRC.sub('.lalrpop', '.rs')

MACROS_SRC = 'src/macros_gen.py'
MACROS_DST = 'src/macros.rs'

rule GRAMMAR_DST => [GRAMMAR_SRC] do
  begin
    sh "lalrpop #{GRAMMAR_SRC}"
  rescue
    puts "is lalrpop installed?"
    puts "try: cargo install lalrpop --features lexer"
    raise
  end
end

rule MACROS_DST => [MACROS_SRC] do
  begin
    sh "python3 #{MACROS_SRC}"
  rescue
    puts "is python3 installed?"
    raise
  end
end

task :build => [GRAMMAR_DST, MACROS_DST] do
  sh 'cargo build'
end

task :fmt do
  sh 'cargo fmt'
  sh "black #{Dir['**/*.py'] * ' '}"
end

task :doc do
  sh 'cargo doc --no-deps --open'
end

task :repl do
  sh 'cargo run --example repl'
end