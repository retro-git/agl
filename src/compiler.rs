use crate::ast::*;

pub struct Compiler {
    code: Vec<String>,
    program: Vec<Declaration>,
}

// fn compile(program: Vec<Declaration>) -> Vec<String> {
//     for declaration in program {
//         match declaration {
//             Declaration::Assign(ident, expr) => {
//                 let mut code = compile_expr(expr);
//                 code.push(format!("mov {}, {}", ident, code.pop().unwrap()));
//                 return code;
//             }
//             Declaration::Statement(statement) => {
//                 return compile_statement(statement);
//             }
//         }
//     }
// }