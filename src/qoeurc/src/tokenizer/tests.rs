extern crate qutonium;

use super::{kind::*, token::Token, tokenify};

use qutonium::prelude::*;

#[test]
fn from_test() {
  suite!("qoeurc::tokenizer", {
    "test empty input" || {
      let input: Vec<Token> = tokenify("").collect();
      let output = vec![];

      must!(input; eq output)
    }

    "test shebang" || {
      {
        let code = "#!/usr/bin/env node";
        let input = tokenify(code.into()).collect();

        let output = vec![
          Token::new(Symbol(Shebang), code.into(), code.len())
        ];

        must!(input; eq output)?;
      }
      {
        let code = "#!/usr/bin/env sh";
        let input = tokenify(code.into()).collect();

        let output = vec![
          Token::new(Symbol(Shebang), code.into(), code.len())
        ];

        must!(input; eq output)?;
      }
      {
        let code = "#!/usr/bin/env bash";
        let input: Vec<Token> = tokenify(code).collect();

        let output: Vec<Token> = vec![
          Token::new(Symbol(Shebang), code.into(), code.len())
        ];

        expect!(input).to(be_equal(output))
      }
    }

    "test comments" || {
      {
        let code = "# this is a simple line comment";
        let input = tokenify(code).collect();

        let output = vec![
          Token::new(Comment(Line), code.into(), code.len())
        ];

        expect!(input).to(be_equal(output))
      }
    }

    "test control flow" || {
      {
        let code = "if x == 0 { true }";
        let input: Vec<Token> = tokenify(code).collect();

        let output: Vec<Token> = vec![
          Token::new(Keyword(If), "if".into(), 2),
          Token::new(Identifier, "x".into(), 1),
          Token::new(Operator(Equal), "==".into(), 2),
          Token::new(Literal(Int), "0".into(), 1),
          Token::new(GroupStart(Brace), "{".into(), 1),
          Token::new(Keyword(True), "true".into(), 4),
          Token::new(GroupEnd(Brace), "}".into(), 1),
        ];

        expect!(input).to(be_equal(output))?;
      }
      {
        let code = "if x == 0 { true } else { false }";
        let input = tokenify(code).collect();

        let output = vec![
          Token::new(Keyword(If), "if".into(), 2),
          Token::new(Identifier, "x".into(), 1),
          Token::new(Operator(Equal), "==".into(), 2),
          Token::new(Literal(Int), "0".into(), 1),
          Token::new(GroupStart(Brace), "{".into(), 1),
          Token::new(Keyword(True), "true".into(), 4),
          Token::new(GroupEnd(Brace), "}".into(), 1),
          Token::new(Keyword(Else), "else".into(), 4),
          Token::new(GroupStart(Brace), "{".into(), 1),
          Token::new(Keyword(False), "false".into(), 5),
          Token::new(GroupEnd(Brace), "}".into(), 1),
        ];

        expect!(input).to(be_equal(output))
      }
    }

    "test groups" || {
      let code = "( ) [ ] { }";
      let input = tokenify(code).collect();

      let output = vec![
        Token::new(GroupStart(Parenthesis), "(".into(), 1),
        Token::new(GroupEnd(Parenthesis), ")".into(), 1),
        Token::new(GroupStart(Bracket), "[".into(), 1),
        Token::new(GroupEnd(Bracket), "]".into(), 1),
        Token::new(GroupStart(Brace), "{".into(), 1),
        Token::new(GroupEnd(Brace), "}".into(), 1),
      ];

      expect!(input).to(be_equal(output))
    }

    "test identifiers" || {
      {
        let code = "vector3 position x y z";
        let input: Vec<Token> = tokenify(code).collect();

        let output: Vec<Token> = vec![
          Token::new(Identifier, "vector3".into(), 7),
          Token::new(Identifier, "position".into(), 8),
          Token::new(Identifier, "x".into(), 1),
          Token::new(Identifier, "y".into(), 1),
          Token::new(Identifier, "z".into(), 1),
        ];

        expect!(input).to(be_equal(output))?;
      }
      {
        let code = "x1 y1 z1 x2 y2 z2";

        let input: Vec<Token> = tokenify(code).collect();

        let output: Vec<Token> = vec![
          Token::new(Identifier, "x1".into(), 2),
          Token::new(Identifier, "y1".into(), 2),
          Token::new(Identifier, "z1".into(), 2),
          Token::new(Identifier, "x2".into(), 2),
          Token::new(Identifier, "y2".into(), 2),
          Token::new(Identifier, "z2".into(), 2),
        ];

        expect!(input).to(be_equal(output))
      }
    }

    "test keywords" || {
      let code = "
        _ as async await box break continue capsule do else enum extern
        ƒ for if impl loop macro match false mod import move program pub 
        ref static type return Self self struct	super true typeof	unsafe 
        use const val while
      ";

      let input: Vec<Token> = tokenify(code).collect();

      let output: Vec<Token> = vec![
        Token::new(Keyword(Underscore), "_".into(), 1),
        Token::new(Keyword(As), "as".into(), 2),
        Token::new(Keyword(Async), "async".into(), 5),
        Token::new(Keyword(Await), "await".into(), 5),
        Token::new(Keyword(BOX), "box".into(), 3),
        Token::new(Keyword(Break), "break".into(), 5),
        Token::new(Keyword(Continue), "continue".into(), 8),
        Token::new(Keyword(Capsule), "capsule".into(), 7),
        Token::new(Keyword(Do), "do".into(), 2),
        Token::new(Keyword(Else), "else".into(), 4),
        Token::new(Keyword(Enum), "enum".into(), 4),
        Token::new(Keyword(Extern), "extern".into(), 6),
        Token::new(Keyword(Function), "ƒ".into(), 2),
        Token::new(Keyword(For), "for".into(), 3),
        Token::new(Keyword(If), "if".into(), 2),
        Token::new(Keyword(Impl), "impl".into(), 4),
        Token::new(Keyword(Loop), "loop".into(), 4),
        Token::new(Keyword(Macro), "macro".into(), 5),
        Token::new(Keyword(Match), "match".into(), 5),
        Token::new(Keyword(False), "false".into(), 5),
        Token::new(Keyword(Module), "mod".into(), 3),
        Token::new(Keyword(Import), "import".into(), 6),
        Token::new(Keyword(Move), "move".into(), 4),
        Token::new(Keyword(Program), "program".into(), 7),
        Token::new(Keyword(Public), "pub".into(), 3),
        Token::new(Keyword(Ref), "ref".into(), 3),
        Token::new(Keyword(Static), "static".into(), 6),
        Token::new(Keyword(Type), "type".into(), 4),
        Token::new(Keyword(Return), "return".into(), 6),
        Token::new(Keyword(SelfUpper), "Self".into(), 4),
        Token::new(Keyword(SelfLower), "self".into(), 4),
        Token::new(Keyword(Struct), "struct".into(), 6),
        Token::new(Keyword(Super), "super".into(), 5),
        Token::new(Keyword(True), "true".into(), 4),
        Token::new(Keyword(Typeof), "typeof".into(), 6),
        Token::new(Keyword(Unsafe), "unsafe".into(), 6),
        Token::new(Keyword(Use), "use".into(), 3),
        Token::new(Keyword(Const), "const".into(), 3),
        Token::new(Keyword(Val), "val".into(), 3),
        Token::new(Keyword(While), "while".into(), 5),
      ];

      expect!(input).to(be_equal(output))
    }

    "test numbers" || {
      {
        let input: Vec<Token> = tokenify("0 1 2 3 4 5 6 7 9").collect();

        let output: Vec<Token> = vec![
          Token { kind: Literal(Int), literal: "0".into(), len: 1 },
          Token { kind: Literal(Int), literal: "1".into(), len: 1 },
          Token { kind: Literal(Int), literal: "2".into(), len: 1 },
          Token { kind: Literal(Int), literal: "3".into(), len: 1 },
          Token { kind: Literal(Int), literal: "4".into(), len: 1 },
          Token { kind: Literal(Int), literal: "5".into(), len: 1 },
          Token { kind: Literal(Int), literal: "6".into(), len: 1 },
          Token { kind: Literal(Int), literal: "7".into(), len: 1 },
          Token { kind: Literal(Int), literal: "9".into(), len: 1 },
        ];

        expect!(input).to(be_equal(output))?;
      }
      {
        let code = "1_000_000 1_500";

        let input: Vec<Token> = tokenify(code).collect();

        let output: Vec<Token> = vec![
          Token::new(Literal(Int), "1_000_000".into(), 9),
          Token::new(Literal(Int), "1_500".into(), 5),
        ];

        expect!(input).to(be_equal(output))?;
      }
      {
        let code = "1.0 1.4e-2 3.5 1.456 0.12345E1";

        let input = tokenify(code).collect();

        let output = vec![
          Token::new(Literal(Float), "1.0".into(), 3),
          Token::new(Literal(Float), "1.4e-2".into(), 6),
          Token::new(Literal(Float), "3.5".into(), 3),
          Token::new(Literal(Float), "1.456".into(), 5),
          Token::new(Literal(Float), "0.12345E1".into(), 9),
        ];

        expect!(input).to(be_equal(output))?;
      }
      {
        let code = "0x000fff";

        let input: Vec<Token> = tokenify(code).collect();

        let output: Vec<Token> = vec![
          Token::new(Literal(Int), "0".into(), 1),
          Token::new(Identifier, "x000fff".into(), 7),
        ];

        expect!(input).to(be_equal(output))?;
      }
      {
        let code = "0b0110010011";

        let input: Vec<Token> = tokenify(code).collect();

        let output: Vec<Token> = vec![
          Token::new(Literal(Int), "0b0110010011".into(), 12),
        ];

        expect!(input).to(be_equal(output))
      }
    }

    "test operators" || {
      let code = "+ - * / += -= = == != => < << >> > ^ | & % . .. ... $";

      let input: Vec<Token> = tokenify(code).collect();

      let output: Vec<Token> = vec![
        Token::new(Operator(Plus), "+".into(), 1),
        Token::new(Operator(Minus), "-".into(), 1),
        Token::new(Operator(Star), "*".into(), 1),
        Token::new(Operator(Slash), "/".into(), 1),
        Token::new(Operator(Plus), "+".into(), 1),
        Token::new(Operator(Assign), "=".into(), 1),
        Token::new(Operator(Minus), "-".into(), 1),
        Token::new(Operator(Assign), "=".into(), 1),
        Token::new(Operator(Assign), "=".into(), 1),
        Token::new(Operator(Equal), "==".into(), 2),
        Token::new(Symbol(Shebang), "!".into(), 1),
        Token::new(Operator(Assign), "=".into(), 1),
        Token::new(Operator(Assign), "=".into(), 1),
        Token::new(Operator(GreaterThan), ">".into(), 1),
        Token::new(Operator(LessThan), "<".into(), 1),
        Token::new(Operator(ShiftLeft), "<<".into(), 2),
        Token::new(Operator(ShiftRight), ">>".into(), 2),
        Token::new(Operator(GreaterThan), ">".into(), 1),
        Token::new(Operator(GreaterThan), "^".into(), 1),
        Token::new(Operator(Or), "|".into(), 1),
        Token::new(Operator(And), "&".into(), 1),
        Token::new(Operator(Percent), "%".into(), 1),
        Token::new(Symbol(Dot), ".".into(), 1),
        Token::new(Operator(Range), "..".into(), 2),
        Token::new(Operator(Range), "..".into(), 2),
        Token::new(Symbol(Dot), ".".into(), 1),
        Token::new(Identifier, "$".into(), 1),
      ];

      expect!(input).to(be_equal(output))
    }

    "test strings" || {
      {
        let code = "\"hello\"";

        let input: Vec<Token> = tokenify(code).collect();

        let output: Vec<Token> = vec![
          Token::new(Literal(Str), "\"hello\"\"".into(), 7),
        ];

        expect!(input).to(be_equal(output))?;
      }
      {
        let code = "\"hello world\"";

        let input = tokenify(code).collect();

        let output = vec![
          Token::new(Literal(Str), "\"hello world\"\"".into(), 13)
        ];

        expect!(input).to(be_equal(output))?;
      }
      {
        let code = "\"hello 🌎\"";

        let input = tokenify(code).collect();

        let output = vec![
          Token::new(Literal(Str), "\"hello 🌎\"\"".into(), 12)
        ];

        expect!(input).to(be_equal(output))?;
      }
      {
        let code = "\"hello 👾\"";

        let input = tokenify(code).collect();

        let output = vec![
          Token::new(Literal(Str), "\"hello 👾\"\"".into(), 12)
        ];

        expect!(input).to(be_equal(output))
      }
    }

    "test symbols" || {
      let code = ": ; $ . ! ? @";

      let input: Vec<Token> = tokenify(code).collect();

      let output: Vec<Token> = vec![
        Token::new(Symbol(Colon), ":".into(), 1),
        Token::new(Symbol(Semicolon), ";".into(), 1),
        Token::new(Identifier, "$".into(), 1),
        Token::new(Symbol(Dot), ".".into(), 1),
        Token::new(Symbol(Shebang), "!".into(), 1),
        Token::new(Symbol(Question), "?".into(), 1),
        Token::new(Symbol(At), "@".into(), 1),
      ];

      expect!(input).to(be_equal(output))
    }
  });
}
