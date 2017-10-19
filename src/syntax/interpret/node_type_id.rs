pub const STATEMENT               : u32 = 1 << 31;
pub const DECLARATION             : u32 = (1 << 30) | STATEMENT;
pub const MODULE_DECLARATION      : u32 = 1 << 29;
pub const FUNCTION                : u32 = 1 << 28;
pub const EXPRESSION              : u32 = 1 << 27;
pub const LITERAL                 : u32 = (1 << 26) | EXPRESSION;

pub const PROGRAM                 : u32 = 1;

// statement
pub const DEBUGGER_STMT           : u32 = 2 | STATEMENT;
pub const EXPRESSION_STMT         : u32 = 3 | STATEMENT;
pub const BLOCK_STMT              : u32 = 4 | STATEMENT;
pub const EMPTY_STMT              : u32 = 5 | STATEMENT;
pub const WITH_STMT               : u32 = 6 | STATEMENT;
pub const RETURN_STMT             : u32 = 7 | STATEMENT;
pub const LABELED_STMT            : u32 = 8 | STATEMENT;
pub const BREAK_STMT              : u32 = 9 | STATEMENT;
pub const CONTINUE_STMT           : u32 = 10 | STATEMENT;
pub const IF_STMT                 : u32 = 11 | STATEMENT;
pub const SWITCH_STMT             : u32 = 12 | STATEMENT;
pub const THROW_STMT              : u32 = 13 | STATEMENT;
pub const TRY_STMT                : u32 = 14 | STATEMENT;
pub const WHILE_STMT              : u32 = 15 | STATEMENT;
pub const DOWHILE_STMT            : u32 = 16 | STATEMENT;
pub const FOR_STMT                : u32 = 17 | STATEMENT;
pub const FORIN_STMT              : u32 = 18 | STATEMENT;
pub const FOROF_STMT              : u32 = 19 | STATEMENT;

pub const FUNCTION_DECL           : u32 = 20 | DECLARATION | FUNCTION;
pub const VARIABLE_DECL           : u32 = 21 | DECLARATION;

// literal
pub const REG_EXP_LITERAL         : u32 = 51 | LITERAL;
pub const NULL_LITERAL            : u32 = 52 | LITERAL;
pub const STRING_LITERAL          : u32 = 53 | LITERAL;
pub const BOOLEAN_LITERAL         : u32 = 54 | LITERAL;
pub const NUMERIC_LITERAL         : u32 = 55 | LITERAL;
pub const DIRECTIVE_LITERAL       : u32 = 56 | LITERAL;

// MISC
pub const HASHBANG                : u32 = 1000;
pub const DIRECTIVE               : u32 = 1001;
