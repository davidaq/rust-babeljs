pub struct Token {
  pub token_type: u16,
  pub start: usize,
  pub end: usize,

  pub flag: u32,

  pub content: Option<String>,
}

pub mod token_type {
  pub const COPY_SOURCE           : u16 = 0b1000000000000000;

  // Space, Tab and Line Breaks
  pub const WHITE_SPACE           : u16 = 0 | COPY_SOURCE;

  // Seperation
  pub const COLON                 : u16 = 1;
  pub const SEMI_COLON            : u16 = 2;
  pub const COMMA                 : u16 = 3;

  // Comment
  pub const LINE_COMMENT          : u16 = 4 | COPY_SOURCE;
  pub const BLOCK_COMMENT         : u16 = 5 | COPY_SOURCE;

  // Keyword
  pub const KEYWORD               : u16 = 6;
  pub mod keyword {
    // Function
    pub const FUNCTION              : u32 = 10;
    pub const RETURN                : u32 = 11;
    pub const ASYNC                 : u32 = 12;
    pub const AWAIT                 : u32 = 13;
    pub const THROW                 : u32 = 14;
    pub const YIELD                 : u32 = 15;
    
    // Class
    pub const CLASS                 : u32 = 20;
    pub const EXTENDS               : u32 = 21;
    pub const STATIC                : u32 = 22;

    // Flow Control
    pub const IF                    : u32 = 30;
    pub const ELSE                  : u32 = 31;
    pub const WHILE                 : u32 = 32;
    pub const FOR                   : u32 = 33;
    pub const BREAK                 : u32 = 34;
    pub const CONTINUE              : u32 = 35;
    pub const DO                    : u32 = 36;

    // Variable
    pub const VAR                   : u32 = 40;
    pub const LET                   : u32 = 41;
    pub const CONST                 : u32 = 42;

    // Other
    pub const IN                    : u32 = 50;
    pub const OF                    : u32 = 51;
    pub const TRY                   : u32 = 52;
    pub const CATCH                 : u32 = 53;
  }

  // Scope Wrapping
  pub const PARENTHESIS           : u16 = 10;
  pub const BRACKET               : u16 = 11;
  pub const BRACE                 : u16 = 12;
  pub mod brace {
    pub const LEFT                  : u32 = 0;
    pub const RIGHT                 : u32 = 0;
  }

  // Values
  pub const IDENTIFIER            : u16 = 20 | COPY_SOURCE;
  pub const REGEX_LITERAL         : u16 = 21 | COPY_SOURCE;
  pub const STRING_LITERAL        : u16 = 21 | COPY_SOURCE;
  pub const TPL_STRING_LITERAL    : u16 = 22 | COPY_SOURCE;
  pub const NUMERIC_LITERAL       : u16 = 23 | COPY_SOURCE;
  pub const BOOLEAN_LITERAL       : u16 = 24;
  pub mod boolean {
    pub const TRUE                : u32 = 1;
    pub const FALSE               : u32 = 0;
  }

  // Operation
  pub const ARROW                 : u16 = 30;
  pub const OPERATOR              : u16 = 31;
  pub mod operator {
    pub const PLUS                  : u32 = 10;
    pub const MINUS                 : u32 = 11;
    pub const STAR                  : u32 = 12;
    pub const MULTIPLY              : u32 = 12;
    pub const DIVIDE                : u32 = 13;
    pub const B_AND                 : u32 = 14;
    pub const B_OR                  : u32 = 10;
    pub const XOR                   : u32 = 10;
    pub const NOT                   : u32 = 10;
    pub const AND                   : u32 = 10;
    pub const OR                    : u32 = 10;
    pub const POW                   : u32 = 10;
    pub const LT                    : u32 = 10;
    pub const GT                    : u32 = 10;
    pub const ELT                   : u32 = 10;
    pub const EGT                   : u32 = 10;
    pub const QUERY                 : u32 = 10;
    pub const HASH                  : u32 = 10;
    pub const AT                    : u32 = 10;
  }

  // Error
  pub const UNEXPECTED            : u16 = 0xffff;
}
