use agl::compiler::*;
use clap::Parser;

#[derive(Parser)]
#[command(name = "agl", version = "0.1.0", about = "A DSL for writing GameShark codes")]
struct Cli {
    input_files: Vec<String>,

    #[arg(short, long, value_enum)]
    mode: Mode,
    
    #[arg(short, long, default_value_t = false)]
    concat: bool,
}

fn main() {
    let cli = Cli::parse();

    println!("input: {:?}", cli.input_files);
    println!("input: {:?}", cli.mode);
    println!("input: {:?}", cli.concat);
}

#[test]
fn test() {
    //load code from /agl/block.agl
    let code = fs::read_to_string("agl/block.agl").unwrap();
    let compiled = compile(code, Mode::PSX);
    println!("{:?}", compiled);
    assert!(false);
}