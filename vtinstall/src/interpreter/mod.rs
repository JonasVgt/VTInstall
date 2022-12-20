use parser::script::Script;


pub fn interpret(script: Script, dry_run:bool){
    for statement in script.statements(){
        //statement.execute(dry_run);
    }
}
