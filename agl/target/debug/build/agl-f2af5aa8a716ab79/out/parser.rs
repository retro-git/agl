// auto-generated: "lalrpop 0.19.9"
// sha3: 86140aae1bb31310e34d8d2320609fcb6f199c63521bfe494a1913c02fc4990e
use std::str::FromStr;
use crate::compiler::*;
use std::collections::HashMap;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::all)]

    use std::str::FromStr;
    use crate::compiler::*;
    use std::collections::HashMap;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(usize),
        Variant2(BitWidth),
        Variant3(Vec<GSInstruction>),
        Variant4(i32),
        Variant5(GSInstruction),
        Variant6(String),
        Variant7(LogicalOp),
        Variant8(alloc::vec::Vec<Vec<GSInstruction>>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 25, 3, 0, 0, 0, 26,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 25, 3, 0, 0, 0, 26,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 25, 3, 28, 0, 0, 26,
        // State 3
        0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 37, 26,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 39, 40, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 39, 40, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 25, 3, 43, 0, 0, 26,
        // State 7
        0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 37, 26,
        // State 8
        0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 37, 26,
        // State 9
        0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 37, 26,
        // State 10
        0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 37, 26,
        // State 11
        0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 37, 26,
        // State 12
        0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 37, 26,
        // State 13
        0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 37, 26,
        // State 14
        52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53, 0, 54, 55, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 37, 26,
        // State 16
        0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 37, 26,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 25, 3, 0, 0, 0, 26,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26, -26, -26, -26, 0, 0, -26,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, -29, -29, -29, 0, 0, -29,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, -32, -32, -32, 0, 0, -32,
        // State 23
        0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        -14, 0, -14, -14, -14, -14, -14, -14, 0, 0, 0, -14, -14, -14, -14, -14, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, -33, -33, -33, 0, 0, -33,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -6, -6, -6, -6, 0, 0, -6,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        -36, 0, -36, 9, -36, -36, -36, 10, 0, 0, 0, -36, -36, 0, -36, -36, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        -21, 0, -21, -21, -21, -21, -21, -21, 0, 0, 0, -21, -21, 0, -21, -21, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        -22, 0, -22, -22, -22, -22, -22, -22, 0, 0, 0, -22, -22, 0, -22, -22, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        -20, 0, -20, -20, -20, -20, -20, -20, 0, 0, 0, -20, -20, 0, -20, -20, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        -11, 0, -11, -11, -11, -11, -11, -11, 0, 0, 0, -11, -11, 0, -11, -11, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        -8, 0, -8, 0, 11, -8, 12, 0, 0, 0, 0, -8, -8, 0, -8, -8, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        -13, 0, -13, -13, -13, -13, -13, -13, 0, 0, 0, -13, -13, 0, -13, -13, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        -19, 0, -19, -19, -19, -19, -19, -19, 0, 0, 0, -19, -19, 0, -19, -19, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, -3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -7, -7, -7, -7, 0, 0, -7,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, -27, -27, -27, 0, 0, -27,
        // State 44
        0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        -9, 0, -9, -9, -9, -9, -9, -9, 0, 0, 0, -9, -9, 0, -9, -9, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        -10, 0, -10, -10, -10, -10, -10, -10, 0, 0, 0, -10, -10, 0, -10, -10, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        -34, 0, -34, 9, -34, -34, -34, 10, 0, 0, 0, -34, -34, 0, -34, -34, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        -35, 0, -35, 9, -35, -35, -35, 10, 0, 0, 0, -35, -35, 0, -35, -35, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        -23, 0, -23, -23, -23, -23, -23, -23, 0, 0, 0, -23, -23, 0, -23, -23, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, -16, -16,
        // State 52
        0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, -17, -17,
        // State 53
        0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, -15, -15,
        // State 54
        0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, -18, -18,
        // State 55
        0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, -28, -28, -28, 0, 0, -28,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, -12, -12, -12, 0, 0, -12,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 23 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        -24,
        // State 1
        -25,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
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
        -26,
        // State 19
        -29,
        // State 20
        0,
        // State 21
        -37,
        // State 22
        -32,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        -33,
        // State 27
        -6,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        -7,
        // State 43
        -27,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        -28,
        // State 59
        -12,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => match state {
                5 => 41,
                _ => 37,
            },
            3 => 18,
            4 => match state {
                3 => 28,
                7 => 44,
                13 => 50,
                15 => 55,
                16 => 56,
                _ => 14,
            },
            5 => match state {
                10 => 47,
                11 => 48,
                _ => 29,
            },
            6 => 19,
            7 => 30,
            8 => match state {
                3 | 7..=13 | 15..=16 => 31,
                _ => 20,
            },
            9 => 15,
            10 => 32,
            11 => match state {
                8 => 45,
                9 => 46,
                _ => 33,
            },
            12 => 21,
            13 => match state {
                1 | 6 => 26,
                17 => 58,
                _ => 22,
            },
            15 => match state {
                2 => 6,
                _ => 1,
            },
            16 => 34,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!=""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""/""###,
            r###""16""###,
            r###""32""###,
            r###""8""###,
            r###"";""###,
            r###""<""###,
            r###""=""###,
            r###""==""###,
            r###"">""###,
            r###""if""###,
            r###""write""###,
            r###""{""###,
            r###""}""###,
            r###"r#"0[xX][0-9a-fA-F]+"#"###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[a-zA-Z_][a-zA-Z0-9_]*"#"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'input, '__1>
    where 
    {
        symbols: &'__1 mut HashMap<String, i32>,
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input ())>,
    }
    impl<'input, '__1> __state_machine::ParserDefinition for __StateMachine<'input, '__1>
    where 
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Vec<GSInstruction>;
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
            __token_to_integer(token, core::marker::PhantomData::<(&())>)
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
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
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
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.symbols,
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&())>,
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
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(6, _) if true => Some(0),
            Token(7, _) if true => Some(1),
            Token(8, _) if true => Some(2),
            Token(9, _) if true => Some(3),
            Token(10, _) if true => Some(4),
            Token(11, _) if true => Some(5),
            Token(12, _) if true => Some(6),
            Token(13, _) if true => Some(7),
            Token(14, _) if true => Some(8),
            Token(15, _) if true => Some(9),
            Token(16, _) if true => Some(10),
            Token(17, _) if true => Some(11),
            Token(18, _) if true => Some(12),
            Token(19, _) if true => Some(13),
            Token(20, _) if true => Some(14),
            Token(21, _) if true => Some(15),
            Token(22, _) if true => Some(16),
            Token(23, _) if true => Some(17),
            Token(24, _) if true => Some(18),
            Token(25, _) if true => Some(19),
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
        _: core::marker::PhantomData<(&'input ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 => match __token {
                Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(21, __tok0) | Token(22, __tok0) | Token(23, __tok0) | Token(24, __tok0) | Token(25, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct ProgramParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl ProgramParser {
        pub fn new() -> ProgramParser {
            let __builder = super::__intern_token::new_builder();
            ProgramParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
        >(
            &self,
            symbols: &mut HashMap<String, i32>,
            input: &'input str,
        ) -> Result<Vec<GSInstruction>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    symbols,
                    input,
                    __phantom: core::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<Result<Vec<GSInstruction>,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            26 => {
                __reduce26(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            27 => {
                __reduce27(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            28 => {
                __reduce28(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            29 => {
                __reduce29(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            30 => {
                __reduce30(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            31 => {
                __reduce31(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            32 => {
                __reduce32(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            33 => {
                __reduce33(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            34 => {
                __reduce34(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            35 => {
                __reduce35(symbols, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            36 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(symbols, input, __sym0);
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
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BitWidth, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, GSInstruction, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, LogicalOp, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<GSInstruction>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Vec<GSInstruction>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i32, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
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
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(30);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action30::<>(symbols, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (0, 0)
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(29);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action29::<>(symbols, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BitWidth = "8" => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(symbols, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BitWidth = "16" => ActionFn(27);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(symbols, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BitWidth = "32" => ActionFn(28);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(symbols, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Block = "{", "}" => ActionFn(37);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action37::<>(symbols, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Block = "{", Statement+, "}" => ActionFn(38);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action38::<>(symbols, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = Term => ActionFn(8);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(symbols, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "*", Primary => ActionFn(12);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action12::<>(symbols, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "/", Primary => ActionFn(13);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action13::<>(symbols, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Factor = Primary => ActionFn(14);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(symbols, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GSInstruction = "write", "(", BitWidth, ",", Expr, ",", Expr, ")", ";" => ActionFn(7);
        assert!(__symbols.len() >= 9);
        let __sym8 = __pop_Variant0(__symbols);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant4(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant4(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym8.2.clone();
        let __nt = super::__action7::<>(symbols, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (9, 6)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Hex = r#"0[xX][0-9a-fA-F]+"# => ActionFn(20);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(symbols, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Ident = r#"[a-zA-Z_][a-zA-Z0-9_]*"# => ActionFn(21);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(symbols, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LogicalOp = "==" => ActionFn(22);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(symbols, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LogicalOp = "!=" => ActionFn(23);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(symbols, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LogicalOp = "<" => ActionFn(24);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(symbols, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LogicalOp = ">" => ActionFn(25);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(symbols, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Num = r#"[0-9]+"# => ActionFn(19);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(symbols, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Primary = Num => ActionFn(15);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(symbols, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Primary = Hex => ActionFn(16);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(symbols, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Primary = Ident => ActionFn(17);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(symbols, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Primary = "(", Expr, ")" => ActionFn(18);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action18::<>(symbols, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(39);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action39::<>(symbols, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 12)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = Statement+ => ActionFn(40);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(symbols, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement = Block => ActionFn(36);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(symbols, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement = Ident, "=", Expr, ";" => ActionFn(3);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action3::<>(symbols, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (4, 13)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement = "if", "(", BitWidth, ",", Expr, LogicalOp, Expr, ")", Statement => ActionFn(4);
        assert!(__symbols.len() >= 9);
        let __sym8 = __pop_Variant3(__symbols);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant4(__symbols);
        let __sym5 = __pop_Variant7(__symbols);
        let __sym4 = __pop_Variant4(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym8.2.clone();
        let __nt = super::__action4::<>(symbols, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (9, 13)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement = GSInstruction => ActionFn(5);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(symbols, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement* =  => ActionFn(31);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action31::<>(symbols, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 14)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement* = Statement+ => ActionFn(32);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action32::<>(symbols, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement => ActionFn(33);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(symbols, input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement+, Statement => ActionFn(34);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action34::<>(symbols, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 15)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Term = Term, "+", Factor => ActionFn(9);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action9::<>(symbols, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Term = Term, "-", Factor => ActionFn(10);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action10::<>(symbols, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        symbols: &mut HashMap<String, i32>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Term = Factor => ActionFn(11);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(symbols, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 16)
    }
}
pub use self::__parse__Program::ProgramParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use std::str::FromStr;
    use crate::compiler::*;
    use std::collections::HashMap;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("^(//[\0-\t\u{b}-\u{10ffff}]*)", true),
            ("^(/\\*([\0-\\)\\+-\u{10ffff}]*\\*+[\0-\\)\\+-\\.0-\u{10ffff}])*([\0-\\)\\+-\u{10ffff}]*\\*+|[\0-\\)\\+-\u{10ffff}])*\\*/)", true),
            ("^(0[Xx][0-9A-Fa-f]+)", false),
            ("^([0-9]+)", false),
            ("^([A-Z_a-z][0-9A-Z_a-z]*)", false),
            ("^([\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}-\u{2029}\u{202f}\u{205f}\u{3000}]*)", true),
            ("^(!=)", false),
            ("^(\\()", false),
            ("^(\\))", false),
            ("^(\\*)", false),
            ("^(\\+)", false),
            ("^(,)", false),
            ("^(\\-)", false),
            ("^(/)", false),
            ("^(16)", false),
            ("^(32)", false),
            ("^(8)", false),
            ("^(;)", false),
            ("^(<)", false),
            ("^(=)", false),
            ("^(==)", false),
            ("^(>)", false),
            ("^(if)", false),
            ("^(write)", false),
            ("^(\\{)", false),
            ("^(\\})", false),
        ];
        __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied()).unwrap()
    }
}
pub(crate) use self::__lalrpop_util::lexer::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, __0, _): (usize, Vec<GSInstruction>, usize),
) -> Vec<GSInstruction>
{
    __0
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, __0, _): (usize, alloc::vec::Vec<Vec<GSInstruction>>, usize),
) -> Vec<GSInstruction>
{
    __0.into_iter().flatten().collect::<Vec<_>>()
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, b, _): (usize, Vec<GSInstruction>, usize),
    (_, r, _): (usize, usize, usize),
) -> Vec<GSInstruction>
{
    {
        println!("l: {:?}, r: {:?}", l, r);
        b
    }
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, i, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, i32, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Vec<GSInstruction>
{
    { 
        symbols.insert(i, e);
        vec![]
    }
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, b, _): (usize, BitWidth, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, addr, _): (usize, i32, usize),
    (_, op, _): (usize, LogicalOp, usize),
    (_, e, _): (usize, i32, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, s, _): (usize, Vec<GSInstruction>, usize),
) -> Vec<GSInstruction>
{
    {
        // intersperse the conditional with the statements
        s.into_iter().map(|i| vec![GSInstruction::Conditional(b, op, addr, e), i]).flatten().collect::<Vec<_>>()
    }
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, __0, _): (usize, GSInstruction, usize),
) -> Vec<GSInstruction>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, alloc::vec::Vec<Vec<GSInstruction>>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Vec<GSInstruction>
{
    __0.into_iter().flatten().collect::<Vec<_>>()
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, b, _): (usize, BitWidth, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, addr, _): (usize, i32, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, i32, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> GSInstruction
{
    GSInstruction::Write(b, addr, e)
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, __0, _): (usize, i32, usize),
) -> i32
{
    __0
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, l, _): (usize, i32, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, i32, usize),
) -> i32
{
    l + r
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, l, _): (usize, i32, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, i32, usize),
) -> i32
{
    l - r
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, __0, _): (usize, i32, usize),
) -> i32
{
    __0
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, l, _): (usize, i32, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, i32, usize),
) -> i32
{
    l * r
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, l, _): (usize, i32, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, i32, usize),
) -> i32
{
    l / r
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, __0, _): (usize, i32, usize),
) -> i32
{
    __0
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, __0, _): (usize, i32, usize),
) -> i32
{
    __0
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, __0, _): (usize, i32, usize),
) -> i32
{
    __0
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, i, _): (usize, String, usize),
) -> i32
{
    {
        // lookup the value of the identifier in the symbol table
        *symbols.get(&i).unwrap()
    }
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, i32, usize),
    (_, _, _): (usize, &'input str, usize),
) -> i32
{
    __0
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> i32
{
    i32::from_str(__0).unwrap()
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> i32
{
    i32::from_str_radix(__0.trim_start_matches("0x"), 16).unwrap()
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> String
{
    String::from_str(s).unwrap()
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> LogicalOp
{
    LogicalOp::Equal
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> LogicalOp
{
    LogicalOp::NotEqual
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> LogicalOp
{
    LogicalOp::LessThan
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> LogicalOp
{
    LogicalOp::GreaterThan
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BitWidth
{
    BitWidth::Bit8
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BitWidth
{
    BitWidth::Bit16
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BitWidth
{
    BitWidth::Bit32
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookbehind.clone()
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookahead.clone()
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Vec<GSInstruction>>
{
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Vec<GSInstruction>>, usize),
) -> alloc::vec::Vec<Vec<GSInstruction>>
{
    v
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, __0, _): (usize, Vec<GSInstruction>, usize),
) -> alloc::vec::Vec<Vec<GSInstruction>>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Vec<GSInstruction>>, usize),
    (_, e, _): (usize, Vec<GSInstruction>, usize),
) -> alloc::vec::Vec<Vec<GSInstruction>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action35<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    __0: (usize, Vec<GSInstruction>, usize),
    __1: (usize, usize, usize),
) -> Vec<GSInstruction>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action30(
        symbols,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        symbols,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action36<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    __0: (usize, Vec<GSInstruction>, usize),
) -> Vec<GSInstruction>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action29(
        symbols,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action35(
        symbols,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action37<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
) -> Vec<GSInstruction>
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action31(
        symbols,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        symbols,
        input,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action38<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, alloc::vec::Vec<Vec<GSInstruction>>, usize),
    __2: (usize, &'input str, usize),
) -> Vec<GSInstruction>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action32(
        symbols,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        symbols,
        input,
        __0,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action39<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<GSInstruction>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action31(
        symbols,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        symbols,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action40<
    'input,
>(
    symbols: &mut HashMap<String, i32>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Vec<GSInstruction>>, usize),
) -> Vec<GSInstruction>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action32(
        symbols,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        symbols,
        input,
        __temp0,
    )
}

pub trait __ToTriple<'input, >
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>;
}

impl<'input, > __ToTriple<'input, > for (usize, Token<'input>, usize)
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Token<'input>, usize), &'static str>
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
