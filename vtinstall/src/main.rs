mod interpreter;
mod compiler;

use std::{fs, path::Path};

use clap::Parser;


///Tool for creating install scripts
#[derive(Parser, Debug)]
#[command(author,version, about)]
struct Args {
   /// path of the install script   
   #[arg(value_name="path")]
   path: String,

   /// performs a dry run with no changes made
   #[arg(short = 'n', long)]
   dry_run: bool,

   /// compile the script into bash instead of running it
   #[arg(short,long)]
   compile : bool,

   /// sets the path of the compilation target
   #[arg(short,long, default_value="./target", id="PATH")]
   target : String
}

fn main() {
   let args = Args::parse();


   let input_path = Path::new(args.path.as_str());
   let input = fs::read_to_string(input_path).unwrap();


   let script = parser::parse(input.as_str(), input_path.file_stem().map_or("script",|x| x.to_str().unwrap_or("script") )).unwrap();
   if args.compile {
      if let Err(error) = compiler::compile(script, Path::new(args.target.as_str()), args.dry_run){
         eprintln!("Compilation failed: {}", error);
         return;
      }
   }else{
      interpreter::interpret(script, args.dry_run);
   }
}