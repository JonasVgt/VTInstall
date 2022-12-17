mod command;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub calculator1); // synthesized by LALRPOP

#[cfg(test)]
mod tests {


    use crate::command::Command;

    use super::*;

    #[test]
    fn only_instruction() {
        let res = vec![Command {
            instruction: String::from("INST"),
            args: Vec::new(),
        }];
        assert_eq!(
            res,
            calculator1::CommandsParser::new().parse("INST").unwrap()
        );
    }

    #[test]
    fn one_line() {
        let res = vec![
            Command {
                instruction: String::from("INST"),
                args: vec![
                    String::from("arg1"),
                    String::from("arg2"),
                    String::from("arg3"),
                ],
            },
        ];

        assert_eq!(
            res,
            calculator1::CommandsParser::new()
                .parse("INST arg1 arg2 arg3")
                .unwrap()
        );
    }

    #[test]
    fn multi_line() {
        let res = vec![
            Command {
                instruction: String::from("INST"),
                args: vec![
                    String::from("arg1"),
                    String::from("arg2"),
                    String::from("arg3"),
                ],
            },
            Command {
                instruction: String::from("INST"),
                args: vec![
                    String::from("arg1"),
                    String::from("arg2"),
                    String::from("arg3"),
                ],
            },
            Command {
                instruction: String::from("INST"),
                args: vec![
                    String::from("arg1"),
                    String::from("arg2"),
                    String::from("arg3"),
                ],
            },
        ];

        assert_eq!(
            res,
            calculator1::CommandsParser::new()
                .parse("INST arg1 arg2 arg3\nINST arg1 arg2 arg3\nINST arg1 arg2 arg3")
                .unwrap()
        );
        assert_eq!(
            res,
            calculator1::CommandsParser::new()
                .parse("INST arg1 arg2 arg3\r\nINST arg1 arg2 arg3\r\nINST arg1 arg2 arg3")
                .unwrap()
        );
    }


    #[test]
    fn multi_whitespace() {
        let res = vec![
            Command {
                instruction: String::from("INST"),
                args: vec![
                    String::from("arg1"),
                    String::from("arg2"),
                    String::from("arg3"),
                ],
            },
        ];

        assert_eq!(
            res,
            calculator1::CommandsParser::new()
                .parse("INST \t\targ1   arg2 arg3")
                .unwrap()
        );
    }

    #[test]
    fn ignore_whitespace_end() {
        let res1 = vec![
            Command {
                instruction: String::from("INST"),
                args: vec![
                    String::from("arg1"),
                    String::from("arg2"),
                    String::from("arg3"),
                ],
            },
        ];

        let res2 = vec![
            Command {
                instruction: String::from("INST"),
                args: Vec::new(),
            },
        ];


        assert_eq!(
            res1,
            calculator1::CommandsParser::new()
                .parse("INST arg1 arg2 arg3 \t ")
                .unwrap()
        );

        assert_eq!(
            res2,
            calculator1::CommandsParser::new()
                .parse("INST \t ")
                .unwrap()
        );
    }


    #[test]
    fn ignore_whitespace_end_multiline() {
        let res1 = vec![
            Command {
                instruction: String::from("INST"),
                args: vec![
                    String::from("arg1"),
                    String::from("arg2"),
                    String::from("arg3"),
                ],
            },
            Command {
                instruction: String::from("INST"),
                args: vec![
                    String::from("arg1"),
                    String::from("arg2"),
                    String::from("arg3"),
                ],
            },
        ];

        let res2 = vec![
            Command {
                instruction: String::from("INST"),
                args: Vec::new(),
            },
            Command {
                instruction: String::from("INST"),
                args: Vec::new(),
            },
        ];


        assert_eq!(
            res1,
            calculator1::CommandsParser::new()
                .parse("INST arg1 arg2 arg3 \t \nINST arg1 arg2 arg3 \t ")
                .unwrap()
        );

        assert_eq!(
            res2,
            calculator1::CommandsParser::new()
                .parse("INST \t \nINST \t ")
                .unwrap()
        );

        assert_eq!(
            res1,
            calculator1::CommandsParser::new()
                .parse("INST arg1 arg2 arg3 \t \r\nINST arg1 arg2 arg3 \t ")
                .unwrap()
        );

        assert_eq!(
            res2,
            calculator1::CommandsParser::new()
                .parse("INST \t \r\nINST \t ")
                .unwrap()
        );
    }

    #[test]
    fn ignore_whitespace_start() {
        let res1 = vec![
            Command {
                instruction: String::from("INST"),
                args: vec![
                    String::from("arg1"),
                    String::from("arg2"),
                    String::from("arg3"),
                ],
            },
        ];

        let res2 = vec![
            Command {
                instruction: String::from("INST"),
                args: Vec::new(),
            },
        ];


        assert_eq!(
            res1,
            calculator1::CommandsParser::new()
                .parse(" \t INST arg1 arg2 arg3")
                .unwrap()
        );

        assert_eq!(
            res2,
            calculator1::CommandsParser::new()
                .parse("\t INST")
                .unwrap()
        );
    }


    #[test]
    fn comment_whole_line() {


        let res = vec![
            Command {
                instruction: String::from("INST"),
                args: vec![
                    String::from("arg1"),
                    String::from("arg2"),
                    String::from("arg3"),
                ],
            },
            Command {
                instruction: String::from("INST"),
                args: vec![
                    String::from("arg1"),
                    String::from("arg2"),
                    String::from("arg3"),
                ],
            },
        ];

        assert_eq!(
            res,
            calculator1::CommandsParser::new()
                .parse("INST arg1 arg2 arg3\n# test comment \nINST arg1 arg2 arg3")
                .unwrap()
        );
    }


    #[test]
    fn comment_partial_line() {


        let res = vec![
            Command {
                instruction: String::from("INST"),
                args: vec![
                    String::from("arg1"),
                ],
            },
        ];

        assert_eq!(
            res,
            calculator1::CommandsParser::new()
                .parse("INST arg1 #arg2 arg3")
                .unwrap()
        );
    }

    #[test]
    fn quoted_argument() {


        let res = vec![
            Command {
                instruction: String::from("INST"),
                args: vec![
                    String::from("arg1.1 arg1.2"),
                    String::from("arg2.1\targ2.2"),
                    String::from(" arg3 "),
                ],
            },
        ];

        assert_eq!(
            res,
            calculator1::CommandsParser::new()
                .parse("INST 'arg1.1 arg1.2' 'arg2.1\targ2.2' ' arg3 '")
                .unwrap()
        );
    }


    #[test]
    fn comment_quoted_argument() {


        let res = vec![
            Command {
                instruction: String::from("INST"),
                args: vec![
                    String::from("#no comment"),
                ],
            },
        ];

        assert_eq!(
            res,
            calculator1::CommandsParser::new()
                .parse("INST '#no comment' #'comment' ")
                .unwrap()
        );
    }


}
