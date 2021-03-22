#[derive(Debug)]
pub enum AsciiKind {
  EOF,
  Comment,
  Group,
  Identifier,
  Number,
  Operator,
  Quote,
  Symbol,
  Unknown(char),
}

pub struct Ascii;

impl Ascii {
  pub fn to_kind(ascii: char) -> AsciiKind {
    match ascii {
      a if Ascii::is_end_of_file(a) => AsciiKind::EOF,
      a if Ascii::is_comment(a) => AsciiKind::Comment,
      a if Ascii::is_group(a) => AsciiKind::Group,
      a if Ascii::is_id_start(a) => AsciiKind::Identifier,
      a if Ascii::is_number(a) => AsciiKind::Number,
      a if Ascii::is_operator(a) => AsciiKind::Operator,
      a if Ascii::is_quote(a) => AsciiKind::Quote,
      a if Ascii::is_symbol(a) => AsciiKind::Symbol,
      _ => AsciiKind::Unknown(ascii),
    }
  }

  pub fn is_carriage_return(ascii: char) -> bool {
    ascii == '\u{000D}' // \r
  }

  pub fn is_comment(ascii: char) -> bool {
    ascii == '\u{0023}' // #
  }

  pub fn is_dollar(ascii: char) -> bool {
    ascii == '\u{0024}' // $
  }

  pub fn is_double_quote(ascii: char) -> bool {
    ascii == '\u{0022}' // "
  }

  pub fn is_end_of_file(ascii: char) -> bool {
    ascii == '\u{0}'
  }

  pub fn is_end_of_line(ascii: char) -> bool {
    ascii == '\u{000A}' // \n
  }

  pub fn is_group(ascii: char) -> bool {
    match ascii {
      '[' | ']' | '(' | ')' | '{' | '}' => true,
      _ => false,
    }
  }

  pub fn is_form_feed(ascii: char) -> bool {
    ascii == '\u{000C}' // \f
  }

  pub fn is_horizontal_tabulation(ascii: char) -> bool {
    ascii == '\u{0009}' // \t
  }

  pub fn is_identifier(ascii: char) -> bool {
    ascii.is_alphabetic() || Ascii::is_underscore(ascii)
  }

  pub fn is_id_continue(ascii: char) -> bool {
    Ascii::is_identifier(ascii)
      || Ascii::is_number(ascii)
      || (ascii > '\x7f' && Ascii::is_xid_continue(ascii))
  }

  pub fn is_id_start(ascii: char) -> bool {
    Ascii::is_identifier(ascii)
      || (ascii > '\x7f' && Ascii::is_xid_start(ascii))
  }

  pub fn is_number(ascii: char) -> bool {
    ascii.is_digit(10)
  }

  pub fn is_operator(ascii: char) -> bool {
    match ascii {
      '+' | '-' | '*' | '/' | '=' | '<' | '>' | '^' | '|' | '&' | '%' => true,
      _ => false,
    }
  }

  pub fn is_punctuation(ascii: char) -> bool {
    ascii.is_ascii_punctuation()
  }

  pub fn is_quote(ascii: char) -> bool {
    Ascii::is_double_quote(ascii) || Ascii::is_single_quote(ascii)
  }

  pub fn is_shebang(ascii: char) -> bool {
    ascii == '\u{0021}'
  }

  pub fn is_single_quote(ascii: char) -> bool {
    ascii == '\u{0027}' // '
  }

  pub fn is_space(ascii: char) -> bool {
    ascii == '\u{0020}' // " "
  }

  pub fn is_symbol(ascii: char) -> bool {
    match ascii {
      ':' | ',' | ';' | '.' | '!' | '?' | '$' | '@' | '#' => true,
      _ => false,
    }
  }

  pub fn is_underscore(ascii: char) -> bool {
    ascii == '\u{005F}' // _
  }

  pub fn is_vertical_tab(ascii: char) -> bool {
    ascii == '\u{000B}' // \v
  }

  pub fn is_whitespace(ascii: char) -> bool {
    match ascii {
      | '\u{0009}' // \t
      | '\u{000A}' // \n
      | '\u{000B}' // vertical tab
      | '\u{000C}' // form feed
      | '\u{000D}' // \r
      | '\u{0020}' // space
      | '\u{0085}' // NEXT LINE from latin1
      | '\u{200E}' // LEFT-TO-RIGHT MARK
      | '\u{200F}' // RIGHT-TO-LEFT MARK
      | '\u{2028}' // LINE SEPARATOR
      | '\u{2029}' // PARAGRAPH SEPARATOR
      => true,
      _ => false,
    }
  }

  pub fn is_xid_start(ascii: char) -> bool {
    unicode_xid::UnicodeXID::is_xid_start(ascii)
  }

  pub fn is_xid_continue(ascii: char) -> bool {
    unicode_xid::UnicodeXID::is_xid_continue(ascii)
  }
}
