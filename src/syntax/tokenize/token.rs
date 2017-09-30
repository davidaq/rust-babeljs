pub struct Token {
  pub token_type: u16,
  pub start: usize,
  pub end: usize,

  pub flag: u32,

  pub content: Option<String>,
}

pub mod token_type {
  // Space, Tab and Line Breaks
  pub const WhiteSpace            : u16 = 0;

  // Seperation
  pub const Colon                 : u16 = 1;
  pub const Semicolon             : u16 = 2;
  pub const Comma                 : u16 = 3;

  // Comment
  pub const LineComment           : u16 = 4;
  pub const BlockComment          : u16 = 5;

  // Scope Wrapping
  pub const Parenthesis           : u16 = 10;
  pub const Bracket               : u16 = 11;
  pub const Brace                 : u16 = 12;

  // Values
  pub const Identifier            : u16 = 20;
  pub const NumericLiteral        : u16 = 21;
  pub const BooleanLiteral        : u16 = 22;
  pub const StringLiteral         : u16 = 23;
  pub const TemplateStringChunk   : u16 = 24;

  // Operation
  pub const Arrow                 : u16 = 30;
  pub const Operator              : u16 = 31;

  // Special Identifiers - Function
  pub const Function              : u16 = 40;
  pub const Return                : u16 = 41;
  pub const Async                 : u16 = 42;
  pub const Await                 : u16 = 43;
  pub const Throw                 : u16 = 44;
  pub const Yield                 : u16 = 45;
  
  // Special Identifiers - Class
  pub const Class                 : u16 = 50;
  pub const Extends               : u16 = 51;

  // Special Identifiers - Flow Control
  pub const If                    : u16 = 60;
  pub const Else                  : u16 = 61;
  pub const While                 : u16 = 62;
  pub const For                   : u16 = 63;
  pub const Break                 : u16 = 64;
  pub const Continue              : u16 = 65;
  pub const In                    : u16 = 66;
  pub const Of                    : u16 = 67;
  pub const Try                   : u16 = 68;
  pub const Catch                 : u16 = 69;

  // Special Identifiers - Variable
  pub const Var                   : u16 = 70;
  pub const Let                   : u16 = 71;
  pub const Const                 : u16 = 72;
  pub const Null                  : u16 = 73;

  // Error
  pub const Unexpected            : u16 = 0xffff;
}
