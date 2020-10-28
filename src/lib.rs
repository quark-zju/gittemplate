//! Language to render various content in a git repo in various formats.
//!
//! # Language Specification
//!
//! ## Core syntax
//!
//! - `"x"`, `'x'`, `12`: [String](#string) and [integer](#integer) literals.
//! - `x`: Resolve a symbol named `x`.
//! - `f(x, y, z)`: Call function `f` with arguments `x`, `y`, `z`.
//!
//! That's it! The above is enough to express all features.
//!
//! ## Syntactic sugar
//!
//! Alternative syntaxes are available for shorter code, and feel familiar
//! with some common languages. They are (sorted from the highest precedence
//! to the lowest):
//!
//! - `x.foo(y)`: desugar to `foo(x, y)` [^ufcs].
//! - `x.bar`: desugar to `x.bar()`, and `bar(x)` [^optpar].
//! - `/x/`: desugar to `re("x")`.
//! - `x?`: desugar to `try(x)`.
//! - `[x, y, z]`: desugar to `vec(x, y, z)`.
//! - `!x`: desugar to `not(x)`.
//! - `x * y`: desugar to `mul(x, y)`.
//! - `x - y + z`: desugar to `add(x, neg(y), z)`.
//! - `x < y`: desugar to `lt(x, y)`.
//! - `x in y`: desugar to `contains(y, x)`.
//! - `x not in y`: desugar to `not(contains(y, x))`.
//! - `x == y`: desugar to `eq(x, y)`.
//! - `x != y`: desugar to `not(eq(x, y))`.
//! - `x && y && z`: desugar to `and(x, y, z)`.
//! - `x || y || z`: desugar to `or(x, y, z)`.
//! - `x .. y .. z`: desugar to `concat(x, y, z)`.
//! - `x => f(x)`: desugar to `lambda(x, f(x))`.
//!
//! `()` can be used to enforce operator precedence. For example,
//! `1..2+3..4` evaluates to `154`, `(1..2)+(3..4)` evaluates to `1234`.
//!
//! ## Global functions and symbols
//!
//! ### Flow control
//!
//! - `if(cond, t:Expr, f:Expr=nil)`: If `cond` is `nil` or `false`, evaluate
//!   `f` and return it. Otherwise, evaluate `t` and return it. [^booltest]
//! - `and(x, y, z)`, or `x && y && z`: `if(x, if(y, z, y), x)`.
//! - `or(x, y, z)`, or `x || y || z`: `if(x, x, if(y, y, z))`.
//! - `try(expr, fallback=nil)`, or `expr?`: Evaluate `expr`. On error return `fallback`.
//!
//! ### Functions on all types
//!
//! - `str(x)`, or `x.str`: Converts to a non-lazy string (returns
//!   [String](#string)).
//! - `typename(x)`: Obtain the type name of `x` as a string ([String](#string)).
//! - `json(x)`: Converts to JSON format ([Json](#json)).
//! - `eq(x, y)`, or `x == y`: Test if `typename(x)` equals to
//!   `typename(y)`, and `str(x)` equals to `str(y)` ([Bool](#bool)).
//! - `not(x)`, or `!x`: `if(x, false, true)` ([Bool](#bool)).
//! - `ast(expr:Expr)`: Get the internal AST of `expr`. Useful to debug
//!   operator precedence ([Expr](#expr)).
//!
//! ### Constructors
//!
//! - `vec(a, b, c, ...)`, or `[a, b, c, ...]`: Constructs a [list](#list)
//!   containing `a`, `b`, `c`, ... All values are evaluated and space is used
//!   as the separator.
//! - `concat(a, b, c, ...)`, or `a .. b .. c`: Constructs a lazy
//!   [list](#list) with no separator. `concat(a, b).str` should equal to
//!   `a.str + b.str`.
//! - `lambda(x, func(x))`, or `x => func(x)`: Construct an
//!   [anonymous function](#lambda).
//! - `repo()`: Construct a git [repo](#repo) specified by the environment
//!   (workdir and `GIT_DIR`).
//! - `repo(path)`: Construct a git [repo](#repo) at the given path.
//! - `re("s")`, or `/s/`: Construct a [regular expression](#regex).
//! - `now()`: Current [timestamp](#timestamp).
//!
//! ### Symbols
//!
//! - `nil`: Get the [`nil`](#nil) value.
//! - `true`: Get the [`true`](#bool) value.
//! - `false`: Get the [`false`](#bool) value.
//!
//! ### Additional functions and symbols
//!
//! Depending on the context, extra symbols might be available. For example,
//! within a `commit` (type: `Commit`) context, `author` is a valid symbol,
//! because `commit.author` is valid.
//!
//! ## Functions on specific types
//!
//! ### String
//!   - `s.add(t:String)`, or `s + t`: Concatenate strings
//!     ([String](#string)).
//!   - `s.repeat(n:Integer)`, or `s.mul(n)`: Repeat `n` times
//!     ([String](#string)).
//!   - `s.contains(v:String)`: Whether `v` is a sub-string of `s`
//!     ([Bool](#bool)).
//!   - `s.contains(r:Regex)`: `r.matches(s).eq(nil)` ([Bool](#bool)).
//!   - `s.matches(r:Regex)`: `r.matches(s)`
//!      ([List(String|Nil)](#list)|[Nil](#nil)).
//!   - `s.gsub(from:String, to:String)`: Substitute all `from` with `to`
//!      ([String](#string)).
//!   - `s.gsub(from:Regex, to)`: `r.gsub(s, to)` ([String](#string)).
//!   - `s.lines`: Lazy list of lines ([List(String)](#list)).
//!   - `s.upper`: Upper case ([String](#string)).
//!   - `s.lower`: Lower case ([String](#string)).
//!   - `s.label(String)`: Apply labels like `"red bold"` to `s`. Affect
//!     colors ([String](#string)).
//!   - `s.red`, `s.green`, `s.bold`, ...: Shortcut for `s.label("red")`,
//!     and so on ([String](#string)).
//!   - `s.len`: Byte length of string `s` ([Integer](#integer)).
//!
//! ### List
//!   - `l.add(o:List)`, or `l + o`: Concatenate lists with a same separator
//!     ([List](#list)).
//!   - `l.contains(v)`: Whether `v` is an item in `l` ([Bool](#bool)).
//!   - `l.map(f:Lambda)`: Transform to another lazy list by replacing each
//!     item with `f.apply(item)` ([List](#list)).
//!   - `l.filter(f:Lambda)`: New list without items when `f.apply(item)` is
//!     false ([List](#list)).
//!   - `l.skip(n:Integer)`: Skip first `n` items ([List](#list)).
//!   - `l.take(n:Integer)`: Keep at most first `n` items ([List](#list)).
//!   - `l.join(sep:String="")`: Specify item  separator ([List](#list))
//!     [^join].
//!   - `l.reverse`: New non-lazy list with reversed items ([List](#list)).
//!   - `l.sort`: New non-lazy list with sorted items ([List](#list)).
//!   - `l.count`: Number of items in the list ([Integer](#integer)).
//!   - `l.nth(i:Integer)`: The `i`-th (starting from 0) item, or
//!     [`nil`](#nil) if `i` exceeds list length.
//!   - `l.first`: `l.nth(0)`.
//!
//! ### Bool
//!   - No specialized functions. `and`, `or`, `not` are global functions.
//!
//! ### Lambda
//!   - `l.apply(v)`: Evaluate the function using parameter `v`.
//!
//! ### Integer
//!   - `i.add(j:Integer)`, or `i + j`: Add integers ([Integer](#integer)).
//!   - `i.negative`, or `i.neg`: Negative. `-i` ([Integer](#integer)).
//!   - `i.lt(o:Timestamp)`: Test if `i` is less than `o` ([Bool](#bool)).
//!
//! ### Nil
//!   - No specialized methods. `nil.or(x)` can be a pattern to specify
//!     a fallback value.
//!
//! ### Regex
//!   - `r.matches(s:String)`: Find matches (captures) in string `s`. Return
//!     `nil` if there are no matches, or a list of captured substrings.
//!      ([List(String|Nil)](#list)|[Nil](#nil)).
//!   - `r.sub(s:String, to:String)`: Replace the first matched
//!      substring in `s` with `to` ([String](#string)).
//!   - `r.sub(s:String, to:Lambda)`: Replace the first matched
//!      substring `v` in `s` with `to.apply(v)` ([String](#string)).
//!   - `r.gsub(s:String, to:String|Lambda)`: Similar to `r.sub` but
//!     replaces all matches instead of just the first ([String](#string)).
//!
//! ### Repo
//!   - `r.revs(spec:String|Expr)`:
//!     [Revset query](https://docs.rs/gitrevset/0.2.0/gitrevset/#language-specification).
//!     `spec` could be a string like `"heads(draft())"`, or an expression
//!     in valid template syntax like `heads(draft())` ([Revset](#revset)).
//!   - `r.lookup(name:String)`: Lookup a single commit by name
//!     ([Commit](#commit)).
//!   - `r.config(name:String)`: Read config as string ([String](#string)).
//!   - `r.root`: Repo path ([String](#string)).
//!
//! ### Revset
//!   - `s.commits`: Lazy list of commits (auto deref)
//!     ([List(Commit)](#list)).
//!   - `s.contains(c:Commit)`: Whether the set contains a commit
//!     ([Bool](#bool)).
//!   - `s.count`: Number of commits in set ([Integer](#integer)).
//!   - `s.reverse`: Reverse `commits` iteration order ([Revset](#revset)).
//!   - `s.graph`: Construct an [`Graph`](#graph) object to render commits.
//!
//! ### Commit
//!   - `c.author`: Author ([Signature](#signature)).
//!   - `c.committer`: Committer ([Signature](#signature)).
//!   - `c.date`, or `c.time`, or `c.timestamp`: `c.author.date`
//!     ([Timestamp](#timestamp)).
//!   - `c.message`, or `c.desc`: Commit message ([String](#string)).
//!   - `c.hash`, or `c.node`: Hex hash ([String](#string)).
//!   - `c.short`: Short (12-char) hex hash ([String](#string)).
//!   - `c.shortest`: Shortest unambiguous hex hash ([String](#string)).
//!   - `c.parents`: Parent commits ([List(Commit)](#list)).
//!
//! ### Signature
//!   - `s.date`: Timestamp of the signature ([Timestamp](#timestamp)).
//!   - `s.name`: Name of the person ([String](#string)).
//!   - `s.email`: Email of the person ([String](#string)).
//!
//! ### Timestamp
//!   - `t.epoch`: Seconds since UNIX epoch ([Integer](#integer)).
//!   - `t.offset`: Timezone offset in minutes ([Integer](#integer)).
//!   - `t.rfc2822`: Format using RFC2822 ([String](#string)).
//!   - `t.rfc3339`: Format using RFC3339 ([String](#string)).
//!   - `t.short`: Format using `%Y-%m-%d` ([String](#string)).
//!   - `t.strftime(fmt:String)`:
//!     [Customized](https://docs.rs/chrono/0.4.19/chrono/format/strftime)
//!     format ([String](#string)).
//!   - `t.relative`: Format as relative ([String](#string)).
//!   - `t.add(seconds:Integer)`: Offset by seconds ([Integer](#integer)).
//!   - `t.lt(o:Timestamp)`: Test if `t` is before `o` ([Bool](#bool)).
//!
//! ### Graph
//!   - `g.map(lambda:GraphCommit=>Object)`: Specify how to render a commit.
//!     ([Graph](#graph)).
//!   - `g.shorten(n=1)`: Specify minimal height per commit ([Graph](#graph)).
//!
//! ### GraphCommit
//!   - `graphwidth`: Width used by the graph characters ([Integer](#integer)).
//!   - `commit`: The commit object (auto deref) ([Commit](#commit)).
//!
//! ### PosixPath
//!   - `p.basename`: The file name of the path ([PosixPath](#posixpath)).
//!   - `p.dirname`: The directory name of the path ([PosixPath](#posixpath)).
//!
//! ### Json
//!   - `j.pretty(enabled:Bool=true)`: Enable or disable pretty print
//!     ([Json](#json)).
//!
//! ### Expr
//!   - No special methods. `.json` can be used to dump structured AST.
//!
//! ## Automatic type conversion
//!
//! Most types, except for revset, automatically convert to String if string
//! functions are used. This makes the code less verbose. For example, given
//! a timestamp `t`, `t.red` works just like `t.str.red` [^deref].
//!
//! The revset type automatically converts to its commits list. This makes
//! it possible to write `set.map(...)` instead of `set.commits.map(...)`.
//!
//! The GraphCommit type automatically converts to its commit.
//!
//! [^optpar]: Parentheses for calling functions without parameters are
//! optional. Similar to
//! [DLang](https://dlang.org/spec/function.html#optional-parenthesis) and
//! Ruby. Note: Ruby extends that to functions with parameters: `f(x, y)`
//! can be written as `f x, y`.
//!
//! [^ufcs]: Similar to
//! [Uniform Function Call Syntax (UFCS)](https://tour.dlang.org/tour/en/gems/uniform-function-call-syntax-ufcs)
//! in DLang.
//!
//! [^booltest]: `0`, `""`, `[]` are treated as `true`. Similar to Ruby and
//! Lua. Different from Python, which treats those as `false`.
//!
//! [^join]: In `xs.join(y)`, `xs` is a list and `y` is a string. Similar to
//! Ruby and Rust. Different from Python, which swaps the arguments:
//! `y.join(xs)`.
//!
//! [^deref]: This is similar to [`deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html)
//! in Rust, or methods on super classes in object oriented languages.

#![allow(dead_code)]
#![deny(missing_docs)]

mod ast;
mod error;
mod eval;
mod macros;
mod objects;
mod parser;

pub use ast::Expr;
pub use error::Error;
pub use eval::EvalContext;

/// `Result` type used by `gittemplate`.
pub type Result<T> = std::result::Result<T, Error>;

// Re-exports
pub use gitrevset;
