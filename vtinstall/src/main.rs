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


   
   if args.compile {
      if let Err(error) = compiler::compile(Path::new(args.path.as_str()), Path::new(args.target.as_str()), args.dry_run){
         eprintln!("Compilation failed: {}", error);
         return;
      }
   }else{
      //interpreter::interpret(script, args.dry_run);
   }
}