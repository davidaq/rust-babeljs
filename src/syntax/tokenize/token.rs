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
  pub const COMMENT               : u16 = 4 | COPY_SOURCE;
  pub mod comment {
    pub const LINE                  : u32 = 0;
    pub const BLOCK                 : u32 = 1;
  }

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

    // Module
    pub const IMPORT                : u32 = 52;
    pub const FROM                  : u32 = 52;
    pub const EXPORT                : u32 = 52;
    pub const DEFAULT               : u32 = 52;

    // Error Handling
    pub const TRY                   : u32 = 62;
    pub const CATCH                 : u32 = 63;
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
    pub const PLUS                  : u32 = 10; // +
    pub const INCRE                 : u32 = 11; // ++
    pub const MINUS                 : u32 = 12; // -
    pub const DECRE                 : u32 = 13; // --
    pub const STAR                  : u32 = 14; // *
    pub const MULTIPLY              : u32 = 14; // *
    pub const POW                   : u32 = 15; // **
    pub const DIVIDE                : u32 = 16; // /
    pub const MOD                   : u32 = 17; // %
    pub const B_AND                 : u32 = 18; // &
    pub const B_OR                  : u32 = 19; // |
    pub const XOR                   : u32 = 20; // ^
    pub const NOT                   : u32 = 21; // !
    pub const AND                   : u32 = 22; // &&
    pub const OR                    : u32 = 23; // ||
    pub const LT                    : u32 = 24; // <
    pub const GT                    : u32 = 25; // >
    pub const ELT                   : u32 = 26; // <=
    pub const EGT                   : u32 = 27; // >=
    pub const LEFT_SHIFT            : u32 = 28; // <<
    pub const RIGHT_SHIFT           : u32 = 29; // >>
    pub const U_RIGHT_SHIFT         : u32 = 30; // >>>
    pub const QUERY                 : u32 = 31; // ?
    pub const HASH                  : u32 = 32; // #
    pub const STRICT_EQUAL          : u32 = 33; // ===
    pub const EQUAL                 : u32 = 34; // ==
    pub const NOT_EQUAL             : u32 = 35; // !=
    pub const STRICT_NOT_EQUAL      : u32 = 36; // !==
    pub const ASSIGN                : u32 = 37; // =
    pub const PLUS_ASSIGN           : u32 = 38; // +=
    pub const MINUS_ASSIGN          : u32 = 39; // -=
    pub const MULTIPLY_ASSIGN       : u32 = 40; // *=
    pub const POW_ASSIGN            : u32 = 41; // **=
    pub const DIVIDE_ASSIGN         : u32 = 42; // /=
    pub const B_AND_ASSIGN          : u32 = 43; // &=
    pub const B_OR_ASSIGN           : u32 = 44; // |=
    pub const B_INVERT              : u32 = 45; // ~
    pub const AT                    : u32 = 47; // @
    pub const IN                    : u32 = 48;
    pub const OF                    : u32 = 49;
    pub const INSTANCEOF            : u32 = 50;
  }

  // Error
  pub const UNEXPECTED            : u16 = 0xffff;
}
