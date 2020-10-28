// auto-generated: "lalrpop 0.19.1"
// sha256: 4e234fdff01cf0cffe8bbca4bdd88511fdb61e2b4ca319d404085387cc5134
use crate::ast;
use crate::ast::Expr;
use crate::ast::unquote;
use crate::objects::protocol::IntoObject;
use super::desugar_concat;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__TopExpr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::ast;
    use crate::ast::Expr;
    use crate::ast::unquote;
    use crate::objects::protocol::IntoObject;
    use super::super::desugar_concat;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1((&'input str, Expr)),
        Variant2(::std::vec::Vec<(&'input str, Expr)>),
        Variant3((Expr, &'input str)),
        Variant4(::std::vec::Vec<(Expr, &'input str)>),
        Variant5(Expr),
        Variant6(::std::option::Option<Expr>),
        Variant7(Vec<Expr>),
        Variant8(ast::Symbol),
    }
    const __ACTION: &[i8] = &[
        // State 0
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 41, 42, 43, 44,
        // State 1
        0, -42, -42, 0, -42, 11, -42, 0, -42, -42, -42, 0, 0, 0, -42, -42, -42, -42, 12, 0, 0, 0, 0,
        // State 2
        0, 0, 17, 0, -52, 0, -52, 0, -52, 0, 0, 0, 0, 0, -52, 0, 0, -52, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, -54, 0, -54, 0, -54, 0, 0, 0, 0, 0, -54, 0, 0, 18, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, -56, 0, -56, 0, 19, 0, 0, 0, 0, 0, -56, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, -69, -69, 21, -69, -69, -69, -69, -69, -69, -69, -69, -69, 0, -69, -69, -69, -69, -69, 0, 0, 0, 0,
        // State 6
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 41, 42, 43, 44,
        // State 7
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 41, 42, 43, 44,
        // State 8
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 56, 0, 0, 0, 0, 41, 42, 43, 44,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0,
        // State 10
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 41, 42, 43, 44,
        // State 11
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 41, 42, 43, 44,
        // State 12
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 41, 42, 43, 44,
        // State 13
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 41, 42, 43, 44,
        // State 14
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 41, 42, 43, 44,
        // State 15
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 41, 42, 43, 44,
        // State 16
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 41, 42, 43, 44,
        // State 17
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 41, 42, 43, 44,
        // State 18
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 41, 42, 43, 44,
        // State 19
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 41, 42, 43, 44,
        // State 20
        7, 0, 0, 8, 68, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 41, 42, 43, 44,
        // State 21
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 71, 0, 0, 0, 0, 41, 42, 43, 44,
        // State 22
        0, -31, -31, 21, -31, -31, -31, -31, -31, -31, -31, 0, -31, 0, -31, -31, -31, -31, -31, 0, 0, 0, 0,
        // State 23
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 41, 42, 43, 44,
        // State 24
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 41, 42, 43, 44,
        // State 25
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 41, 42, 43, 44,
        // State 26
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 41, 42, 43, 44,
        // State 27
        7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 41, 42, 43, 44,
        // State 28
        7, 0, 0, 8, 81, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 41, 42, 43, 44,
        // State 29
        0, -38, -38, 0, -38, -38, -38, 10, -38, -38, -38, 0, 45, 0, -38, -38, -38, -38, -38, 0, 0, 0, 0,
        // State 30
        0, -40, -40, 0, -40, -40, -40, 0, -40, -40, -40, 0, 0, 0, -40, -40, -40, -40, -40, 0, 0, 0, 0,
        // State 31
        0, -44, -44, 0, -44, 0, -44, 0, -44, 13, -44, 0, 0, 0, -44, -44, -44, -44, 0, 0, 0, 0, 0,
        // State 32
        0, -46, -46, 0, -46, 0, -46, 0, -46, 0, -46, 0, 0, 0, -46, 14, 47, -46, 0, 0, 0, 0, 0,
        // State 33
        0, 15, -49, 0, -49, 0, -49, 0, -49, 0, 16, 0, 0, 0, -49, 0, 0, -49, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, -58, 0, -58, 0, 0, 0, 0, 0, 0, 0, -58, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, -28, -28, 0, -28, -28, -28, -28, -28, -28, -28, 0, -28, 0, -28, -28, -28, -28, -28, 0, 0, 0, 0,
        // State 37
        0, -27, -27, 0, -27, -27, -27, -27, -27, -27, -27, 0, -27, 0, -27, -27, -27, -27, -27, 0, 0, 0, 0,
        // State 38
        0, -26, -26, 0, -26, -26, -26, -26, -26, -26, -26, 20, -26, 0, -26, -26, -26, -26, -26, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, -68, -68, 0, -68, -68, -68, -68, -68, -68, -68, 0, -68, 0, -68, -68, -68, -68, -68, 0, 0, 0, 0,
        // State 41
        0, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, -70, 0, -70, -70, -70, -70, -70, 0, 0, 0, 0,
        // State 42
        0, -66, -66, 0, -66, -66, -66, -66, -66, -66, -66, 0, -66, 0, -66, -66, -66, -66, -66, 0, 0, 0, 0,
        // State 43
        0, -67, -67, 0, -67, -67, -67, -67, -67, -67, -67, 0, -67, 0, -67, -67, -67, -67, -67, 0, 0, 0, 0,
        // State 44
        0, -32, -32, 0, -32, -32, -32, -32, -32, -32, -32, 0, -32, 0, -32, -32, -32, -32, -32, 0, 0, 0, 0,
        // State 45
        0, -43, -43, 0, -43, 0, -43, 0, -43, -43, -43, 0, 0, 0, -43, -43, -43, -43, 24, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 26, 0, -53, 0, -53, 0, -53, 0, 0, 0, 0, 0, -53, 0, 0, -53, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, -55, 0, -55, 0, -55, 0, 0, 0, 0, 0, -55, 0, 0, 27, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, -57, 0, -57, 0, 28, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, -29, -29, 0, -29, -29, -29, -29, -29, -29, -29, 0, -29, 0, -29, -29, -29, -29, -29, 0, 0, 0, 0,
        // State 51
        0, -39, -39, 0, -39, -39, -39, 0, -39, -39, -39, 0, 0, 0, -39, -39, -39, -39, -39, 0, 0, 0, 0,
        // State 52
        0, -26, -26, 0, -26, -26, -26, -26, -26, -26, -26, 0, -26, 0, -26, -26, -26, -26, -26, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 72, 0, 0, 0, 0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, -34, -34, 0, -34, -34, -34, -34, -34, -34, -34, 0, -34, 0, -34, -34, -34, -34, -34, 0, 0, 0, 0,
        // State 56
        0, -41, -41, 0, -41, -41, -41, 0, -41, -41, -41, 0, 0, 0, -41, -41, -41, -41, -41, 0, 0, 0, 0,
        // State 57
        0, -24, -24, 0, -24, 11, -24, 0, -24, -24, -24, 0, 0, 0, -24, -24, -24, -24, -24, 0, 0, 0, 0,
        // State 58
        0, -45, -45, 0, -45, 0, -45, 0, -45, 0, -45, 0, 0, 0, -45, -45, -45, -45, 0, 0, 0, 0, 0,
        // State 59
        0, -47, -47, 0, -47, 0, -47, 0, -47, 0, -47, 0, 0, 0, -47, 0, 0, -47, 0, 0, 0, 0, 0,
        // State 60
        0, 0, -51, 0, -51, 0, -51, 0, -51, 0, 0, 0, 0, 0, -51, 0, 0, -51, 0, 0, 0, 0, 0,
        // State 61
        0, 0, -50, 0, -50, 0, -50, 0, -50, 0, 0, 0, 0, 0, -50, 0, 0, -50, 0, 0, 0, 0, 0,
        // State 62
        0, 0, -4, 0, -4, 0, -4, 0, -4, 0, 0, 0, 0, 0, -4, 0, 0, -4, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, -14, 0, -14, 0, -14, 0, 0, 0, 0, 0, -14, 0, 0, -14, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, -9, 0, -9, 0, -9, 0, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, -59, 0, -59, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 82, 0, 72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, -63, -63, 0, -63, -63, -63, -63, -63, -63, -63, 0, -63, 0, -63, -63, -63, -63, -63, 0, 0, 0, 0,
        // State 68
        0, -37, -37, 0, -37, -37, -37, -37, -37, -37, -37, 0, -37, 0, -37, -37, -37, -37, -37, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, 83, 0, 0, 0, 0, 0, 0, 0, 84, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, -36, -36, 0, -36, -36, -36, -36, -36, -36, -36, 0, -36, 0, -36, -36, -36, -36, -36, 0, 0, 0, 0,
        // State 71
        -19, 0, 0, -19, -19, 0, 0, 0, 0, 0, 0, 0, 0, -19, -19, 0, 0, 0, 0, -19, -19, -19, -19,
        // State 72
        0, -33, -33, 0, -33, -33, -33, -33, -33, -33, -33, 0, -33, 0, -33, -33, -33, -33, -33, 0, 0, 0, 0,
        // State 73
        0, -30, -30, 0, -30, -30, -30, -30, -30, -30, -30, 0, -30, 0, -30, -30, -30, -30, -30, 0, 0, 0, 0,
        // State 74
        0, -25, -25, 0, -25, 11, -25, 0, -25, -25, -25, 0, 0, 0, -25, -25, -25, -25, -25, 0, 0, 0, 0,
        // State 75
        0, -48, -48, 0, -48, 0, -48, 0, -48, 0, -48, 0, 0, 0, -48, 0, 0, -48, 0, 0, 0, 0, 0,
        // State 76
        0, 0, -5, 0, -5, 0, -5, 0, -5, 0, 0, 0, 0, 0, -5, 0, 0, -5, 0, 0, 0, 0, 0,
        // State 77
        0, 0, 0, 0, -15, 0, -15, 0, -15, 0, 0, 0, 0, 0, -15, 0, 0, -15, 0, 0, 0, 0, 0,
        // State 78
        0, 0, 0, 0, -10, 0, -10, 0, -10, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 79
        0, 0, 0, 0, 85, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 80
        0, -65, -65, 0, -65, -65, -65, -65, -65, -65, -65, 0, -65, 0, -65, -65, -65, -65, -65, 0, 0, 0, 0,
        // State 81
        0, -62, -62, 0, -62, -62, -62, -62, -62, -62, -62, 0, -62, 0, -62, -62, -62, -62, -62, 0, 0, 0, 0,
        // State 82
        -20, 0, 0, -20, -20, 0, 0, 0, 0, 0, 0, 0, 0, -20, -20, 0, 0, 0, 0, -20, -20, -20, -20,
        // State 83
        0, -35, -35, 0, -35, -35, -35, -35, -35, -35, -35, 0, -35, 0, -35, -35, -35, -35, -35, 0, 0, 0, 0,
        // State 84
        0, -64, -64, 0, -64, -64, -64, -64, -64, -64, -64, 0, -64, 0, -64, -64, -64, -64, -64, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 23 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -42,
        // State 2
        -52,
        // State 3
        -54,
        // State 4
        -56,
        // State 5
        -69,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        -31,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        -38,
        // State 30
        -40,
        // State 31
        -44,
        // State 32
        -46,
        // State 33
        -49,
        // State 34
        -58,
        // State 35
        -71,
        // State 36
        -28,
        // State 37
        -27,
        // State 38
        -26,
        // State 39
        -72,
        // State 40
        -68,
        // State 41
        -70,
        // State 42
        -66,
        // State 43
        -67,
        // State 44
        -32,
        // State 45
        -43,
        // State 46
        0,
        // State 47
        -53,
        // State 48
        -55,
        // State 49
        -57,
        // State 50
        -29,
        // State 51
        -39,
        // State 52
        -26,
        // State 53
        0,
        // State 54
        0,
        // State 55
        -34,
        // State 56
        -41,
        // State 57
        -24,
        // State 58
        -45,
        // State 59
        -47,
        // State 60
        -51,
        // State 61
        -50,
        // State 62
        -4,
        // State 63
        -14,
        // State 64
        -9,
        // State 65
        -59,
        // State 66
        0,
        // State 67
        -63,
        // State 68
        -37,
        // State 69
        0,
        // State 70
        -36,
        // State 71
        0,
        // State 72
        -33,
        // State 73
        -30,
        // State 74
        -25,
        // State 75
        -48,
        // State 76
        -5,
        // State 77
        -15,
        // State 78
        -10,
        // State 79
        0,
        // State 80
        -65,
        // State 81
        -62,
        // State 82
        0,
        // State 83
        -35,
        // State 84
        -64,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 47,
            5 => 49,
            8 => 48,
            11 => match state {
                20 => 28,
                _ => 21,
            },
            14 => 45,
            15 => 29,
            16 => match state {
                6 => 51,
                10 => 56,
                _ => 30,
            },
            17 => match state {
                11 => 57,
                23 => 74,
                _ => 1,
            },
            18 => match state {
                12 => 58,
                _ => 31,
            },
            19 => match state {
                13 => 59,
                24 => 75,
                _ => 32,
            },
            20 => match state {
                14 => 60,
                15 => 61,
                _ => 33,
            },
            21 => match state {
                16 => 62,
                25 => 76,
                _ => 2,
            },
            22 => match state {
                17 => 63,
                26 => 77,
                _ => 3,
            },
            23 => match state {
                18 => 64,
                27 => 78,
                _ => 4,
            },
            24 => match state {
                19 => 65,
                _ => 34,
            },
            25 => match state {
                7 => 53,
                8 => 54,
                20 => 66,
                21 => 69,
                28 => 79,
                _ => 35,
            },
            27 => match state {
                22 => 73,
                _ => 50,
            },
            28 => 36,
            29 => 37,
            30 => match state {
                6 | 10..=19 | 23..=27 => 52,
                _ => 38,
            },
            31 => match state {
                9 => 22,
                _ => 5,
            },
            32 => 39,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> Vec<::std::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""!=""###,
            r###""&&""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###"",""###,
            r###"".""###,
            r###""..""###,
            r###""<""###,
            r###""==""###,
            r###""=>""###,
            r###""?""###,
            r###""[""###,
            r###""]""###,
            r###""in""###,
            r###""not""###,
            r###""||""###,
            r###"r#"(\\+|-)"#"###,
            r###"r#"/([^/]|\\x5c.)*/"#"###,
            r###"r#"[a-zA-Z0-9_$@]+"#"###,
            r###"r#"\\x22([^\\x22\\x5c]|\\x5c.)*\\x22"#"###,
            r###"r#"\\x27([^\\x27\\x5c]|\\x5c.)*\\x27"#"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct __StateMachine<'input>
    where 
    {
        ignore_errors: bool,
        input: &'input str,
        __phantom: ::std::marker::PhantomData<(&'input ())>,
    }
    impl<'input> __state_machine::ParserDefinition for __StateMachine<'input>
    where 
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Expr;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, ::std::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 23 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, ::std::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> Vec<String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut Vec<i8>,
            symbols: &mut Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.ignore_errors,
                self.input,
                action,
                start_location,
                states,
                symbols,
                ::std::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
        'input,
    >(
        __token: &Token<'input>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(5, _) if true => Some(0),
            Token(6, _) if true => Some(1),
            Token(7, _) if true => Some(2),
            Token(8, _) if true => Some(3),
            Token(9, _) if true => Some(4),
            Token(10, _) if true => Some(5),
            Token(11, _) if true => Some(6),
            Token(12, _) if true => Some(7),
            Token(13, _) if true => Some(8),
            Token(14, _) if true => Some(9),
            Token(15, _) if true => Some(10),
            Token(16, _) if true => Some(11),
            Token(17, _) if true => Some(12),
            Token(18, _) if true => Some(13),
            Token(19, _) if true => Some(14),
            Token(20, _) if true => Some(15),
            Token(21, _) if true => Some(16),
            Token(22, _) if true => Some(17),
            Token(0, _) if true => Some(18),
            Token(1, _) if true => Some(19),
            Token(2, _) if true => Some(20),
            Token(3, _) if true => Some(21),
            Token(4, _) if true => Some(22),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 => match __token {
                Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(21, __tok0) | Token(22, __tok0) | Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct TopExprParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl TopExprParser {
        pub fn new() -> TopExprParser {
            let __builder = super::__intern_token::new_builder();
            TopExprParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
        >(
            &self,
            ignore_errors: bool,
            input: &'input str,
        ) -> Result<Expr, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    ignore_errors,
                    input,
                    __phantom: ::std::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> Option<Result<Expr,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            26 => {
                __reduce26(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            27 => {
                __reduce27(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            28 => {
                __reduce28(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            29 => {
                __reduce29(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            30 => {
                __reduce30(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            31 => {
                __reduce31(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            32 => {
                __reduce32(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            33 => {
                __reduce33(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            34 => {
                __reduce34(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            35 => {
                __reduce35(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            36 => {
                __reduce36(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            37 => {
                __reduce37(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            38 => {
                __reduce38(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            39 => {
                __reduce39(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            40 => {
                __reduce40(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            41 => {
                __reduce41(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            42 => {
                __reduce42(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            43 => {
                __reduce43(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            44 => {
                __reduce44(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            45 => {
                __reduce45(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            46 => {
                __reduce46(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            47 => {
                __reduce47(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            48 => {
                __reduce48(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            49 => {
                __reduce49(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            50 => {
                __reduce50(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            51 => {
                __reduce51(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            52 => {
                __reduce52(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            53 => {
                __reduce53(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            54 => {
                __reduce54(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            55 => {
                __reduce55(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            56 => {
                __reduce56(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            57 => {
                __reduce57(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            58 => {
                __reduce58(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            59 => {
                __reduce59(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            60 => {
                __reduce60(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            61 => {
                __reduce61(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            62 => {
                __reduce62(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            63 => {
                __reduce63(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            64 => {
                __reduce64(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            65 => {
                __reduce65(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            66 => {
                __reduce66(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            67 => {
                __reduce67(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            68 => {
                __reduce68(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            69 => {
                __reduce69(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            70 => {
                __reduce70(ignore_errors, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            71 => {
                // __TopExpr = TopExpr => ActionFn(0);
                let __sym0 = __pop_Variant5(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(ignore_errors, input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (Expr, &'input str), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (&'input str, Expr), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Expr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ast::Symbol, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Expr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(Expr, &'input str)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(&'input str, Expr)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("&&" Expr105) = "&&", Expr105 => ActionFn(45);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action45::<>(ignore_errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("&&" Expr105)* =  => ActionFn(43);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action43::<>(ignore_errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("&&" Expr105)* = ("&&" Expr105)+ => ActionFn(44);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(ignore_errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("&&" Expr105)+ = "&&", Expr105 => ActionFn(62);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action62::<>(ignore_errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("&&" Expr105)+ = ("&&" Expr105)+, "&&", Expr105 => ActionFn(63);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action63::<>(ignore_errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (".." Expr115) = "..", Expr115 => ActionFn(51);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action51::<>(ignore_errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (".." Expr115)* =  => ActionFn(49);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action49::<>(ignore_errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (".." Expr115)* = (".." Expr115)+ => ActionFn(50);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(ignore_errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (".." Expr115)+ = "..", Expr115 => ActionFn(66);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action66::<>(ignore_errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (".." Expr115)+ = (".." Expr115)+, "..", Expr115 => ActionFn(67);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action67::<>(ignore_errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("||" Expr110) = "||", Expr110 => ActionFn(48);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action48::<>(ignore_errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("||" Expr110)* =  => ActionFn(46);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action46::<>(ignore_errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("||" Expr110)* = ("||" Expr110)+ => ActionFn(47);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(ignore_errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("||" Expr110)+ = "||", Expr110 => ActionFn(70);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action70::<>(ignore_errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("||" Expr110)+ = ("||" Expr110)+, "||", Expr110 => ActionFn(71);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action71::<>(ignore_errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 8)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (Expr200 ",") = Expr200, "," => ActionFn(39);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action39::<>(ignore_errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (Expr200 ",")* =  => ActionFn(37);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action37::<>(ignore_errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (Expr200 ",")* = (Expr200 ",")+ => ActionFn(38);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(ignore_errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (Expr200 ",")+ = Expr200, "," => ActionFn(74);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action74::<>(ignore_errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 11)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (Expr200 ",")+ = (Expr200 ",")+, Expr200, "," => ActionFn(75);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action75::<>(ignore_errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (r#"(\\+|-)"# Expr040) = r#"(\\+|-)"#, Expr040 => ActionFn(42);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action42::<>(ignore_errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 12)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (r#"(\\+|-)"# Expr040)* =  => ActionFn(40);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action40::<>(ignore_errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 13)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (r#"(\\+|-)"# Expr040)* = (r#"(\\+|-)"# Expr040)+ => ActionFn(41);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(ignore_errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (r#"(\\+|-)"# Expr040)+ = r#"(\\+|-)"#, Expr040 => ActionFn(80);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action80::<>(ignore_errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 14)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (r#"(\\+|-)"# Expr040)+ = (r#"(\\+|-)"# Expr040)+, r#"(\\+|-)"#, Expr040 => ActionFn(81);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action81::<>(ignore_errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 14)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr000 = Symbol => ActionFn(20);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(ignore_errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr000 = Regex => ActionFn(21);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(ignore_errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr000 = Literal => ActionFn(22);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(ignore_errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr000 = SymbolName, GroupedArgs => ActionFn(23);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action23::<>(ignore_errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 15)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr000 = Expr000, ".", SymbolName, GroupedArgs => ActionFn(24);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action24::<>(ignore_errors, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (4, 15)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr000 = Expr000, ".", SymbolName => ActionFn(25);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action25::<>(ignore_errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr000 = Expr000, "?" => ActionFn(26);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action26::<>(ignore_errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 15)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr000 = "[", Expr200, "]" => ActionFn(84);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action84::<>(ignore_errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr000 = "[", "]" => ActionFn(85);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action85::<>(ignore_errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 15)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr000 = "[", (Expr200 ",")+, Expr200, "]" => ActionFn(86);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action86::<>(ignore_errors, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (4, 15)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr000 = "[", (Expr200 ",")+, "]" => ActionFn(87);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action87::<>(ignore_errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr000 = "(", Expr200, ")" => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action28::<>(ignore_errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 15)
    }
    pub(crate) fn __reduce37<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr010 = Expr000 => ActionFn(18);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(ignore_errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce38<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr010 = "!", Expr010 => ActionFn(19);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action19::<>(ignore_errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 16)
    }
    pub(crate) fn __reduce39<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr040 = Expr010 => ActionFn(16);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(ignore_errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce40<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr040 = Expr040, "*", Expr010 => ActionFn(17);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action17::<>(ignore_errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 17)
    }
    pub(crate) fn __reduce41<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr050 = Expr040 => ActionFn(82);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action82::<>(ignore_errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce42<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr050 = Expr040, (r#"(\\+|-)"# Expr040)+ => ActionFn(83);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action83::<>(ignore_errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 18)
    }
    pub(crate) fn __reduce43<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr060 = Expr050 => ActionFn(13);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(ignore_errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce44<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr060 = Expr050, "<", Expr050 => ActionFn(14);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action14::<>(ignore_errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 19)
    }
    pub(crate) fn __reduce45<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr100 = Expr060 => ActionFn(10);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<>(ignore_errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce46<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr100 = Expr060, "in", Expr060 => ActionFn(11);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action11::<>(ignore_errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 20)
    }
    pub(crate) fn __reduce47<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr100 = Expr060, "not", "in", Expr060 => ActionFn(12);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action12::<>(ignore_errors, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (4, 20)
    }
    pub(crate) fn __reduce48<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr105 = Expr100 => ActionFn(7);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(ignore_errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce49<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr105 = Expr100, "==", Expr100 => ActionFn(8);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8::<>(ignore_errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 21)
    }
    pub(crate) fn __reduce50<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr105 = Expr100, "!=", Expr100 => ActionFn(9);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action9::<>(ignore_errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 21)
    }
    pub(crate) fn __reduce51<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr110 = Expr105 => ActionFn(64);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action64::<>(ignore_errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce52<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr110 = Expr105, ("&&" Expr105)+ => ActionFn(65);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action65::<>(ignore_errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 22)
    }
    pub(crate) fn __reduce53<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr115 = Expr110 => ActionFn(72);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action72::<>(ignore_errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce54<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr115 = Expr110, ("||" Expr110)+ => ActionFn(73);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action73::<>(ignore_errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 23)
    }
    pub(crate) fn __reduce55<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr150 = Expr115 => ActionFn(68);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action68::<>(ignore_errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce56<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr150 = Expr115, (".." Expr115)+ => ActionFn(69);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action69::<>(ignore_errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 24)
    }
    pub(crate) fn __reduce57<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr200 = Expr150 => ActionFn(2);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(ignore_errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce58<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr200 = Symbol, "=>", Expr150 => ActionFn(3);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action3::<>(ignore_errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 25)
    }
    pub(crate) fn __reduce59<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr200? = Expr200 => ActionFn(35);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(ignore_errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce60<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr200? =  => ActionFn(36);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action36::<>(ignore_errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 26)
    }
    pub(crate) fn __reduce61<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GroupedArgs = "(", Expr200, ")" => ActionFn(88);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action88::<>(ignore_errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 27)
    }
    pub(crate) fn __reduce62<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GroupedArgs = "(", ")" => ActionFn(89);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action89::<>(ignore_errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 27)
    }
    pub(crate) fn __reduce63<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GroupedArgs = "(", (Expr200 ",")+, Expr200, ")" => ActionFn(90);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action90::<>(ignore_errors, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 27)
    }
    pub(crate) fn __reduce64<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GroupedArgs = "(", (Expr200 ",")+, ")" => ActionFn(91);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action91::<>(ignore_errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 27)
    }
    pub(crate) fn __reduce65<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Literal = r#"\\x22([^\\x22\\x5c]|\\x5c.)*\\x22"# => ActionFn(33);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(ignore_errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce66<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Literal = r#"\\x27([^\\x27\\x5c]|\\x5c.)*\\x27"# => ActionFn(34);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(ignore_errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce67<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Regex = r#"/([^/]|\\x5c.)*/"# => ActionFn(30);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(ignore_errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce68<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Symbol = SymbolName => ActionFn(31);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31::<>(ignore_errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce69<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // SymbolName = r#"[a-zA-Z0-9_$@]+"# => ActionFn(32);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action32::<>(ignore_errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce70<
        'input,
    >(
        ignore_errors: bool,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TopExpr = Expr200 => ActionFn(1);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(ignore_errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 32)
    }
}
pub use self::__parse__TopExpr::TopExprParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use crate::ast;
    use crate::ast::Expr;
    use crate::ast::unquote;
    use crate::objects::protocol::IntoObject;
    use super::desugar_concat;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("^((\\+|\\-))", false),
            ("^(/([\u{0}-\\.0-\u{10ffff}]|\\\\[\u{0}-\t\u{b}-\u{10ffff}])*/)", false),
            ("^([\\$0-9@-Z_a-z]+)", false),
            ("^(\"([\u{0}-!\\#-\\[\\]-\u{10ffff}]|\\\\[\u{0}-\t\u{b}-\u{10ffff}])*\")", false),
            ("^(\'([\u{0}-\\&\\(-\\[\\]-\u{10ffff}]|\\\\[\u{0}-\t\u{b}-\u{10ffff}])*\')", false),
            ("^(!)", false),
            ("^(!=)", false),
            ("^(\\&\\&)", false),
            ("^(\\()", false),
            ("^(\\))", false),
            ("^(\\*)", false),
            ("^(,)", false),
            ("^(\\.)", false),
            ("^(\\.\\.)", false),
            ("^(<)", false),
            ("^(==)", false),
            ("^(=>)", false),
            ("^(\\?)", false),
            ("^(\\[)", false),
            ("^(\\])", false),
            ("^(in)", false),
            ("^(not)", false),
            ("^(\\|\\|)", false),
            (r"^(\s*)", true),
        ];
        __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied()).unwrap()
    }
}
pub use self::__lalrpop_util::lexer::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, name, _): (usize, Expr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, body, _): (usize, Expr, usize),
) -> Expr
{
    {
        // x => body: desugar to lambda(x, body)
        ast!(lambda({name}, {body}))
    }
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, x, _): (usize, Expr, usize),
    (_, xs, _): (usize, ::std::vec::Vec<(&'input str, Expr)>, usize),
) -> Expr
{
    {
        // x .. y .. z: desugar to concat(x, y, z)
        desugar_concat(x, xs, "concat")
    }
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, x, _): (usize, Expr, usize),
    (_, xs, _): (usize, ::std::vec::Vec<(&'input str, Expr)>, usize),
) -> Expr
{
    {
        // x || y || z: desugar to or(x, y, z)
        desugar_concat(x, xs, "or")
    }
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, x, _): (usize, Expr, usize),
    (_, xs, _): (usize, ::std::vec::Vec<(&'input str, Expr)>, usize),
) -> Expr
{
    {
        // x && y && z: desugar to and(x, y, z)
        desugar_concat(x, xs, "and")
    }
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, x, _): (usize, Expr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, y, _): (usize, Expr, usize),
) -> Expr
{
    {
        // x == y: desugar to eq(x, y)
        ast!(eq({x}, {y}))
    }
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, x, _): (usize, Expr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, y, _): (usize, Expr, usize),
) -> Expr
{
    {
        // x != y: desugar to not(eq(x, y))
        ast!(not(eq({x}, {y})))
    }
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, x, _): (usize, Expr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, y, _): (usize, Expr, usize),
) -> Expr
{
    {
        // x in y: desugar to contains(y, x)
        ast!(contains({y}, {x}))
    }
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, x, _): (usize, Expr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, y, _): (usize, Expr, usize),
) -> Expr
{
    {
        // x not in y: desugar to not(contains(y, x))
        ast!(not(contains({y}, {x})))
    }
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, x, _): (usize, Expr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, y, _): (usize, Expr, usize),
) -> Expr
{
    {
        // x < y: desugar to lt(x, y)
        ast!(lt({x}, {y}))
    }
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, x, _): (usize, Expr, usize),
    (_, xs, _): (usize, ::std::vec::Vec<(&'input str, Expr)>, usize),
) -> Expr
{
    {
        if xs.is_empty() {
            x
        } else {
            // x + y - z: desugar to add(x, y, neg(z))
            let args: Vec<Expr> = std::iter::once(x).chain(
                xs.into_iter().map(|(sig, e)| {
                    if sig == "+" {
                        e
                    } else {
                        Expr::Fn("neg".into(), vec![e])
                    }
                })
            ).collect();
            Expr::Fn("add".into(), args)
        }
    }
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, x, _): (usize, Expr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, y, _): (usize, Expr, usize),
) -> Expr
{
    {
        ast!(mul({x}, {y}))
    }
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, x, _): (usize, Expr, usize),
) -> Expr
{
    {
        // !x: desugar to not(x)
        ast!(not({x}))
    }
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    __0
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, func_name, _): (usize, ast::Symbol, usize),
    (_, args, _): (usize, Vec<Expr>, usize),
) -> Expr
{
    {
        // Function call.
        Expr::Fn(func_name, args)
    }
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, this, _): (usize, Expr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, method_name, _): (usize, ast::Symbol, usize),
    (_, args, _): (usize, Vec<Expr>, usize),
) -> Expr
{
    {
        // x.foo(a, b, c): desugar to foo(x, a, b, c)
        let mut args = args;
        args.insert(0, this);
        Expr::Fn(method_name, args)
    }
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, arg, _): (usize, Expr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, attr_name, _): (usize, ast::Symbol, usize),
) -> Expr
{
    {
        // x.foo: desugar to foo(x)
        Expr::Fn(attr_name, vec![arg])
    }
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, x, _): (usize, Expr, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expr
{
    {
        // x?: desugar to try(x)
        ast!(try({x}))
    }
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, items, _): (usize, ::std::vec::Vec<(Expr, &'input str)>, usize),
    (_, last, _): (usize, ::std::option::Option<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expr
{
    {
        // [x, y, z]: desugar to vec(x, y, z)
        let mut item_list: Vec<Expr> = items.into_iter().map(|(e, _)| e).collect();
        if let Some(last_item) = last { item_list.push(last_item); }
        Expr::Fn("vec".into(), item_list)
    }
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, x, _): (usize, Expr, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expr
{
    x
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, args, _): (usize, ::std::vec::Vec<(Expr, &'input str)>, usize),
    (_, last, _): (usize, ::std::option::Option<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Vec<Expr>
{
    {
        let mut arg_list: Vec<Expr> = args.into_iter().map(|(e, _)| e).collect();
        if let Some(last_arg) = last { arg_list.push(last_arg); }
        arg_list
    }
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> Expr
{
    {
        // /foo/: desugar to re("foo")
        ast!(re({unquote(s)}))
    }
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, __0, _): (usize, ast::Symbol, usize),
) -> Expr
{
    Expr::Symbol(__0)
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ast::Symbol
{
    __0.to_string().into()
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> Expr
{
    {
        // "foo"
        Expr::Inlined(unquote(s).into_object())
    }
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> Expr
{
    {
        // 'foo'
        Expr::Inlined(unquote(s).into_object())
    }
}

#[allow(unused_variables)]
fn __action35<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> ::std::option::Option<Expr>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action36<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Expr>
{
    None
}

#[allow(unused_variables)]
fn __action37<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<(Expr, &'input str)>
{
    vec![]
}

#[allow(unused_variables)]
fn __action38<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(Expr, &'input str)>, usize),
) -> ::std::vec::Vec<(Expr, &'input str)>
{
    v
}

#[allow(unused_variables)]
fn __action39<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
    (_, __1, _): (usize, &'input str, usize),
) -> (Expr, &'input str)
{
    (__0, __1)
}

#[allow(unused_variables)]
fn __action40<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<(&'input str, Expr)>
{
    vec![]
}

#[allow(unused_variables)]
fn __action41<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(&'input str, Expr)>, usize),
) -> ::std::vec::Vec<(&'input str, Expr)>
{
    v
}

#[allow(unused_variables)]
fn __action42<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Expr, usize),
) -> (&'input str, Expr)
{
    (__0, __1)
}

#[allow(unused_variables)]
fn __action43<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<(&'input str, Expr)>
{
    vec![]
}

#[allow(unused_variables)]
fn __action44<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(&'input str, Expr)>, usize),
) -> ::std::vec::Vec<(&'input str, Expr)>
{
    v
}

#[allow(unused_variables)]
fn __action45<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Expr, usize),
) -> (&'input str, Expr)
{
    (__0, __1)
}

#[allow(unused_variables)]
fn __action46<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<(&'input str, Expr)>
{
    vec![]
}

#[allow(unused_variables)]
fn __action47<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(&'input str, Expr)>, usize),
) -> ::std::vec::Vec<(&'input str, Expr)>
{
    v
}

#[allow(unused_variables)]
fn __action48<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Expr, usize),
) -> (&'input str, Expr)
{
    (__0, __1)
}

#[allow(unused_variables)]
fn __action49<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<(&'input str, Expr)>
{
    vec![]
}

#[allow(unused_variables)]
fn __action50<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(&'input str, Expr)>, usize),
) -> ::std::vec::Vec<(&'input str, Expr)>
{
    v
}

#[allow(unused_variables)]
fn __action51<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Expr, usize),
) -> (&'input str, Expr)
{
    (__0, __1)
}

#[allow(unused_variables)]
fn __action52<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, __0, _): (usize, (&'input str, Expr), usize),
) -> ::std::vec::Vec<(&'input str, Expr)>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action53<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(&'input str, Expr)>, usize),
    (_, e, _): (usize, (&'input str, Expr), usize),
) -> ::std::vec::Vec<(&'input str, Expr)>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action54<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, __0, _): (usize, (&'input str, Expr), usize),
) -> ::std::vec::Vec<(&'input str, Expr)>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action55<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(&'input str, Expr)>, usize),
    (_, e, _): (usize, (&'input str, Expr), usize),
) -> ::std::vec::Vec<(&'input str, Expr)>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action56<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, __0, _): (usize, (&'input str, Expr), usize),
) -> ::std::vec::Vec<(&'input str, Expr)>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action57<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(&'input str, Expr)>, usize),
    (_, e, _): (usize, (&'input str, Expr), usize),
) -> ::std::vec::Vec<(&'input str, Expr)>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action58<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, __0, _): (usize, (&'input str, Expr), usize),
) -> ::std::vec::Vec<(&'input str, Expr)>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action59<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(&'input str, Expr)>, usize),
    (_, e, _): (usize, (&'input str, Expr), usize),
) -> ::std::vec::Vec<(&'input str, Expr)>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action60<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, __0, _): (usize, (Expr, &'input str), usize),
) -> ::std::vec::Vec<(Expr, &'input str)>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action61<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(Expr, &'input str)>, usize),
    (_, e, _): (usize, (Expr, &'input str), usize),
) -> ::std::vec::Vec<(Expr, &'input str)>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action62<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Expr, usize),
) -> ::std::vec::Vec<(&'input str, Expr)>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action45(
        ignore_errors,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action56(
        ignore_errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action63<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<(&'input str, Expr)>, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Expr, usize),
) -> ::std::vec::Vec<(&'input str, Expr)>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action45(
        ignore_errors,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action57(
        ignore_errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action64<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action43(
        ignore_errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        ignore_errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action65<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, Expr, usize),
    __1: (usize, ::std::vec::Vec<(&'input str, Expr)>, usize),
) -> Expr
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action44(
        ignore_errors,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        ignore_errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action66<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Expr, usize),
) -> ::std::vec::Vec<(&'input str, Expr)>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action51(
        ignore_errors,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action52(
        ignore_errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action67<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<(&'input str, Expr)>, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Expr, usize),
) -> ::std::vec::Vec<(&'input str, Expr)>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action51(
        ignore_errors,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action53(
        ignore_errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action68<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action49(
        ignore_errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        ignore_errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action69<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, Expr, usize),
    __1: (usize, ::std::vec::Vec<(&'input str, Expr)>, usize),
) -> Expr
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action50(
        ignore_errors,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        ignore_errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action70<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Expr, usize),
) -> ::std::vec::Vec<(&'input str, Expr)>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action48(
        ignore_errors,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action54(
        ignore_errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action71<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<(&'input str, Expr)>, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Expr, usize),
) -> ::std::vec::Vec<(&'input str, Expr)>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action48(
        ignore_errors,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action55(
        ignore_errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action72<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action46(
        ignore_errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        ignore_errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action73<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, Expr, usize),
    __1: (usize, ::std::vec::Vec<(&'input str, Expr)>, usize),
) -> Expr
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action47(
        ignore_errors,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        ignore_errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action74<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, Expr, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<(Expr, &'input str)>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action39(
        ignore_errors,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action60(
        ignore_errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action75<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<(Expr, &'input str)>, usize),
    __1: (usize, Expr, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<(Expr, &'input str)>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action39(
        ignore_errors,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action61(
        ignore_errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action76<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, ::std::option::Option<Expr>, usize),
    __2: (usize, &'input str, usize),
) -> Expr
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action37(
        ignore_errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        ignore_errors,
        input,
        __0,
        __temp0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action77<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, ::std::vec::Vec<(Expr, &'input str)>, usize),
    __2: (usize, ::std::option::Option<Expr>, usize),
    __3: (usize, &'input str, usize),
) -> Expr
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action38(
        ignore_errors,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        ignore_errors,
        input,
        __0,
        __temp0,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action78<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, ::std::option::Option<Expr>, usize),
    __2: (usize, &'input str, usize),
) -> Vec<Expr>
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action37(
        ignore_errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        ignore_errors,
        input,
        __0,
        __temp0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action79<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, ::std::vec::Vec<(Expr, &'input str)>, usize),
    __2: (usize, ::std::option::Option<Expr>, usize),
    __3: (usize, &'input str, usize),
) -> Vec<Expr>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action38(
        ignore_errors,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        ignore_errors,
        input,
        __0,
        __temp0,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action80<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Expr, usize),
) -> ::std::vec::Vec<(&'input str, Expr)>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action42(
        ignore_errors,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action58(
        ignore_errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action81<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<(&'input str, Expr)>, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Expr, usize),
) -> ::std::vec::Vec<(&'input str, Expr)>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action42(
        ignore_errors,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action59(
        ignore_errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action82<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, Expr, usize),
) -> Expr
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action40(
        ignore_errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        ignore_errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action83<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, Expr, usize),
    __1: (usize, ::std::vec::Vec<(&'input str, Expr)>, usize),
) -> Expr
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action41(
        ignore_errors,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        ignore_errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action84<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Expr, usize),
    __2: (usize, &'input str, usize),
) -> Expr
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action35(
        ignore_errors,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action76(
        ignore_errors,
        input,
        __0,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action85<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
) -> Expr
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action36(
        ignore_errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action76(
        ignore_errors,
        input,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action86<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, ::std::vec::Vec<(Expr, &'input str)>, usize),
    __2: (usize, Expr, usize),
    __3: (usize, &'input str, usize),
) -> Expr
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action35(
        ignore_errors,
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action77(
        ignore_errors,
        input,
        __0,
        __1,
        __temp0,
        __3,
    )
}

#[allow(unused_variables)]
fn __action87<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, ::std::vec::Vec<(Expr, &'input str)>, usize),
    __2: (usize, &'input str, usize),
) -> Expr
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action36(
        ignore_errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action77(
        ignore_errors,
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action88<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Expr, usize),
    __2: (usize, &'input str, usize),
) -> Vec<Expr>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action35(
        ignore_errors,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action78(
        ignore_errors,
        input,
        __0,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action89<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
) -> Vec<Expr>
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action36(
        ignore_errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action78(
        ignore_errors,
        input,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action90<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, ::std::vec::Vec<(Expr, &'input str)>, usize),
    __2: (usize, Expr, usize),
    __3: (usize, &'input str, usize),
) -> Vec<Expr>
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action35(
        ignore_errors,
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action79(
        ignore_errors,
        input,
        __0,
        __1,
        __temp0,
        __3,
    )
}

#[allow(unused_variables)]
fn __action91<
    'input,
>(
    ignore_errors: bool,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, ::std::vec::Vec<(Expr, &'input str)>, usize),
    __2: (usize, &'input str, usize),
) -> Vec<Expr>
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action36(
        ignore_errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action79(
        ignore_errors,
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

pub trait __ToTriple<'input, > {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>;
}

impl<'input, > __ToTriple<'input, > for (usize, Token<'input>, usize) {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Token<'input>, usize), &'static str> {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
