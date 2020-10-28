# gittemplate

[![Documentation](https://docs.rs/gittemplate/badge.svg)](https://docs.rs/gittemplate)
![Build Status](https://github.com/quark-zju/gittemplate/workflows/build/badge.svg)

A scripting language for rendering data to text in a Git repository.

The goal is to integrate common read operations (`log`, `status`, `diff`) in
a unified, concise programming language, to complement areas where git lacks
of flexibility.

Refer to the crate documentation for language specification, and the REPL
example for how to use it in a Rust program.

Example:

    > revs('heads(all())+roots(all())').graph.map(c=>[c.shortest.yellow.bold, c.author.name.cyan, c.date.relative].join('  ') .. '\n' .. c.message.lines.first)
    o  5ef  Jun Wu  8 minutes ago
    ╷  commit.shortest
    ╷
    ╷ o  c1c2  Jun Wu  14 hours ago
    ╭─╯  WIP on master: e234d0e graph
    ╷
    o  68  Jun Wu  a week ago
       grammar and parser


## This is WIP

The fundamental part of the language like the type system, the interpreter
is relatively complete. The functions and types in stdlib are subject to
change. Namely, the following areas are being considered:
- Types to make `diff`, `status` possible are TBD.
- Merge `gitrevset` language (which _only_ has a unique `Set` type) in.
  This unlocks more flexibility like `set.filter(c => c.message.len < 10)`.
- Error handling can probably be cleaned up a bit.
- Should it be extended to read-write operations like `rebase`?
- What is the day-to-day interface: REPL? Git aliases?
- Should it integrate with a pager?

If you're interested and have some ideas, feel free to leave them via issues.
