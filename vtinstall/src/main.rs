use clap::Parser;

///Tool for creating install scripts
#[derive(Parser, Debug)]
#[command(author,version, about)]
struct Args {
   /// Path of the install script   
   #[arg(value_name="path")]
   path: String,


}

fn main() {
   let args = Args::parse();
}