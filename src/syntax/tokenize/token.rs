use syntax::basic_types::SourceLoc;

pub struct Token2<'a> {
  pub content: &'a str,
}

pub struct Token {
  pub token_type: u16,
  pub loc: SourceLoc,

  pub flag: u32,

  pub content: Option<String>,
}

pub mod token_type {

  // MARKER
  pub const COPY_SOURCE           : u16 = 1 << 15;
  pub const BEFORE_EXPR           : u16 = 1 << 14;
  pub const IS_LOOP               : u16 = 1 << 13;
  pub const KEYWORD               : u16 = 1 << 12;

  pub const ALL_MARKER            : u16 = COPY_SOURCE | BEFORE_EXPR | IS_LOOP | KEYWORD;

  // Space, Tab and Line Breaks
  pub const WHITE_SPACE           : u16 = 0 | COPY_SOURCE;

  // Seperation
  pub const COLON                 : u16 = 1 | BEFORE_EXPR;
  pub const SEMI_COLON            : u16 = 2 | BEFORE_EXPR;
  pub const COMMA                 : u16 = 3 | BEFORE_EXPR;

  // Comment
  pub const COMMENT               : u16 = 4 | COPY_SOURCE;
  pub mod comment {
    pub const LINE                  : u32 = 0;
    pub const BLOCK                 : u32 = 1;
    pub const HASHBANG              : u32 = 2;
  }
  // Scope Wrapping
  pub const PARENTHESIS_L         : u16 = 10 | BEFORE_EXPR;
  pub const PARENTHESIS_R         : u16 = 11;
  pub const BRACKET_L             : u16 = 12 | BEFORE_EXPR;
  pub const BRACKET_R             : u16 = 13;
  pub const BRACE_L               : u16 = 14 | BEFORE_EXPR;
  pub const BRACE_R               : u16 = 15;

  // Values
  pub const IDENTIFIER            : u16 = 20 | COPY_SOURCE;
  pub const REGEX_LITERAL         : u16 = 21 | COPY_SOURCE;
  pub const STRING_LITERAL        : u16 = 22 | COPY_SOURCE;
  pub const TPL_STR_LITERAL       : u16 = 23 | COPY_SOURCE;
  pub const TPL_STR_L             : u16 = 24 | COPY_SOURCE | BEFORE_EXPR;
  pub const TPL_STR_R             : u16 = 25 | COPY_SOURCE;
  pub const TPL_STR_RL            : u16 = 26 | COPY_SOURCE | BEFORE_EXPR;
  pub const NUMERIC_LITERAL       : u16 = 27 | COPY_SOURCE;
  pub const BOOLEAN_LITERAL       : u16 = 28;
  pub mod boolean {
    pub const TRUE                : u32 = 1;
    pub const FALSE               : u32 = 0;
  }

  // Operation
  pub const ARROW                 : u16 = 30 | BEFORE_EXPR;
  pub const QUESTION              : u16 = 31 | BEFORE_EXPR; // ?
  pub const DOT                   : u16 = 32 | BEFORE_EXPR; // .
  pub const QUESTION_DOT          : u16 = 33 | BEFORE_EXPR; // .
  pub const ELIPSIS               : u16 = 34 | BEFORE_EXPR; // ...
  pub const HASH                  : u16 = 35; // #
  pub const AT                    : u16 = 36; // @
  pub const OPERATOR              : u16 = 38 | BEFORE_EXPR;
  pub const ASSIGN_OPERATOR       : u16 = 39 | BEFORE_EXPR;
  pub mod operator {
    pub const PLUS                  : u32 = 10; // +
    pub const INCRE                 : u32 = 11; // ++
    pub const MINUS                 : u32 = 12; // -
    pub const DECRE                 : u32 = 13; // --
    pub const STAR                  : u32 = 14; // *
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
  }

  // Keyword - Function
  pub const FUNCTION              : u16 = 50 | KEYWORD;
  pub const RETURN                : u16 = 51 | KEYWORD | BEFORE_EXPR;
  pub const ASYNC                 : u16 = 52 | KEYWORD;
  pub const AWAIT                 : u16 = 53 | KEYWORD | BEFORE_EXPR;
  pub const THROW                 : u16 = 54 | KEYWORD | BEFORE_EXPR;
  pub const YIELD                 : u16 = 55 | KEYWORD | BEFORE_EXPR;
  
  // Keyword - Class
  pub const CLASS                 : u16 = 60 | KEYWORD;
  pub const EXTENDS               : u16 = 61 | KEYWORD | BEFORE_EXPR;
  pub const STATIC                : u16 = 62 | KEYWORD;

  // Keyword - Flow Control
  pub const IF                    : u16 = 70 | KEYWORD;
  pub const ELSE                  : u16 = 71 | KEYWORD | BEFORE_EXPR;
  pub const SWITCH                : u16 = 72 | KEYWORD;
  pub const CASE                  : u16 = 73 | KEYWORD | BEFORE_EXPR;
  pub const WHILE                 : u16 = 74 | KEYWORD | IS_LOOP;
  pub const FOR                   : u16 = 75 | KEYWORD | IS_LOOP;
  pub const BREAK                 : u16 = 76 | KEYWORD;
  pub const CONTINUE              : u16 = 77 | KEYWORD;
  pub const DO                    : u16 = 78 | KEYWORD | BEFORE_EXPR | IS_LOOP;
  pub const WITH                  : u16 = 79 | KEYWORD;

  // Keyword - Variable
  pub const VAR                   : u16 = 80 | KEYWORD;
  pub const LET                   : u16 = 81 | KEYWORD;
  pub const CONST                 : u16 = 82 | KEYWORD;
  pub const TYPEOF                : u16 = 84 | KEYWORD | BEFORE_EXPR;
  pub const INSTANCEOF            : u16 = 85 | KEYWORD | BEFORE_EXPR;
  pub const IN                    : u16 = 86 | KEYWORD | BEFORE_EXPR;
  pub const OF                    : u16 = 87 | KEYWORD | BEFORE_EXPR;
  pub const NEW                   : u16 = 88 | KEYWORD | BEFORE_EXPR;
  pub const DELETE                : u16 = 89 | KEYWORD | BEFORE_EXPR;

  // Keyword - Module
  pub const IMPORT                : u16 = 90 | KEYWORD;
  pub const FROM                  : u16 = 91 | KEYWORD;
  pub const EXPORT                : u16 = 92 | KEYWORD;
  pub const DEFAULT               : u16 = 93 | KEYWORD | BEFORE_EXPR;

  // Keyword - Error Handling
  pub const TRY                   : u16 = 101 | KEYWORD;
  pub const CATCH                 : u16 = 102 | KEYWORD;

  // Keyword - Other
  pub const VOID                  : u16 = 110 | KEYWORD | BEFORE_EXPR;

  // Exception
  pub const UNEXPECTED            : u16 = 0xffff;
  pub const EOF                   : u16 = 0xfffe;
}
