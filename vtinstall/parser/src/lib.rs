use script::Script;

pub mod script;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser); // synthesized by LALRPOP

pub fn parse<'a>(
    input: &'a str,
    name: &'a str,
) -> Result<Script, lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token<'a>, &'a str>> {
    parser::ScriptParser::new().parse(name, input)
}

/*
#[cfg(test)]
mod tests {

    use crate::{
        script::{builder, command::Command, statement::Statement, Script},
    };

    use super::*;

    fn test_script(num_statements: usize, num_args: usize) -> Script {
        let mut builder = Script::builder();
        for _ in 0..num_statements {
            let mut args: Vec<String> = Vec::new();
            for i in 1..=num_args {
                let mut arg = String::from("arg");
                arg.push_str(i.to_string().as_str());
                args.push(arg);
            }
            builder = builder.add_statement(Statement::COMMAND(Command::get_run_instruction(args.join(" "))));
        }
        builder.name(String::from("name")).build()
    }

    #[test]
    fn only_instruction() {
        assert_eq!(
            test_script(1, 0),
            parser::ScriptParser::new().parse("name", "RUN").unwrap()
        );
    }

    #[test]
    fn one_line() {
        assert_eq!(
            test_script(1, 3),
            parser::ScriptParser::new()
                .parse("name", "RUN arg1 arg2 arg3")
                .unwrap()
        );
    }

    #[test]
    fn multi_line() {
        assert_eq!(
            test_script(3, 3),
            parser::ScriptParser::new()
                .parse(
                    "name",
                    "RUN arg1 arg2 arg3\nRUN arg1 arg2 arg3\nRUN arg1 arg2 arg3"
                )
                .unwrap()
        );
        assert_eq!(
            test_script(3, 3),
            parser::ScriptParser::new()
                .parse(
                    "name",
                    "RUN arg1 arg2 arg3\r\nRUN arg1 arg2 arg3\r\nRUN arg1 arg2 arg3"
                )
                .unwrap()
        );
    }

    #[test]
    fn multi_whitespace() {
        assert_eq!(
            test_script(1, 3),
            parser::ScriptParser::new()
                .parse("name", "RUN \t\targ1   arg2 arg3")
                .unwrap()
        );
    }

    /*#[test]
    fn ignore_whitespace_end() {
        assert_eq!(
            test_script(1, 3),
            parser::ScriptParser::new()
                .parse("name", "RUN arg1 arg2 arg3 \t ")
                .unwrap()
        );

        assert_eq!(
            test_script(1, 0),
            parser::ScriptParser::new()
                .parse("name", "RUN \t ")
                .unwrap()
        );
    }*/

    /*#[test]
    fn ignore_whitespace_end_multiline() {
        assert_eq!(
            test_script(2, 3),
            parser::ScriptParser::new()
                .parse("name", "RUN arg1 arg2 arg3 \t \nRUN arg1 arg2 arg3 \t ")
                .unwrap()
        );

        assert_eq!(
            test_script(2, 0),
            parser::ScriptParser::new()
                .parse("name", "RUN \t \nRUN \t ")
                .unwrap()
        );

        assert_eq!(
            test_script(2, 3),
            parser::ScriptParser::new()
                .parse("name", "RUN arg1 arg2 arg3 \t \r\nRUN arg1 arg2 arg3 \t ")
                .unwrap()
        );

        assert_eq!(
            test_script(2, 0),
            parser::ScriptParser::new()
                .parse("name", "RUN \t \r\nRUN \t ")
                .unwrap()
        );
    }*/

    #[test]
    fn ignore_whitespace_start() {
        assert_eq!(
            test_script(1, 3),
            parser::ScriptParser::new()
                .parse("name", " \t RUN arg1 arg2 arg3")
                .unwrap()
        );

        assert_eq!(
            test_script(1, 0),
            parser::ScriptParser::new().parse("name", "\t RUN").unwrap()
        );
    }

    /*#[test]
    fn comment_whole_line() {
        assert_eq!(
            test_script(2, 3),
            parser::ScriptParser::new()
                .parse(
                    "name",
                    "RUN arg1 arg2 arg3\n# test comment \nRUN arg1 arg2 arg3"
                )
                .unwrap()
        );
    }

    #[test]
    fn comment_partial_line() {
        assert_eq!(
            test_script(2, 3),
            parser::ScriptParser::new()
                .parse(
                    "name",
                    "RUN arg1 arg2 arg3 #comment\nRUN arg1 arg2 arg3 #comment"
                )
                .unwrap()
        );
    }*/

    #[test]
    fn quoted_argument() {
        assert_eq!(
            Script::builder()
                .name(String::from("name"))
                .add_statement(Statement::COMMAND(Command::get_run_instruction(
                    String::from("'arg1.1 arg1.2' 'arg2.1\targ2.2' ' arg3 '")
                )))
                .build(),
            parser::ScriptParser::new()
                .parse("name", "RUN 'arg1.1 arg1.2' 'arg2.1\targ2.2' ' arg3 '")
                .unwrap()
        );
    }

    #[test]
    fn comment_quoted_argument() {
        assert_eq!(
            Script::builder()
                .name(String::from("name"))
                .add_statement(Statement::COMMAND(Command::get_run_instruction(
                    String::from("'#no comment ' ")
                )))
                .build(),
            parser::ScriptParser::new()
                .parse("name", "RUN '#no comment' #'comment' ")
                .unwrap()
        );
    }

    #[test]
    fn comment_in_argument() {
        assert_eq!(
            Script::builder()
                .name(String::from("name"))
                .add_statement(Statement::COMMAND(Command::get_run_instruction(
                    String::from("no#comment ")
                )))
                .build(),
            parser::ScriptParser::new()
                .parse("name", "RUN no#comment #comment ")
                .unwrap()
        );
    }
}
*/