pub const Statement               : u32 = 1 << 31;
pub const Declaration             : u32 = (1 << 30) | Statement;
pub const ModuleDeclaration       : u32 = 1 << 29;
pub const Function                : u32 = 1 << 28;

pub const Program                 : u32 = 1;

pub const DebuggerStatement       : u32 = 2 | Statement;
pub const ExpressionStatement     : u32 = 3 | Statement;
pub const BlockStatement          : u32 = 4 | Statement;
pub const EmptyStatement          : u32 = 5 | Statement;
pub const WithStatement           : u32 = 6 | Statement;
pub const ReturnStatement         : u32 = 7 | Statement;
pub const LabeledStatement        : u32 = 8 | Statement;
pub const BreakStatement          : u32 = 9 | Statement;
pub const ContinueStatement       : u32 = 10 | Statement;
pub const IfStatement             : u32 = 11 | Statement;
pub const SwitchStatement         : u32 = 12 | Statement;
pub const ThrowStatement          : u32 = 13 | Statement;
pub const TryStatement            : u32 = 13 | Statement;
pub const WhileStatement          : u32 = 13 | Statement;
pub const DoWhileStatement        : u32 = 13 | Statement;
pub const ForStatement            : u32 = 13 | Statement;
pub const ForInStatement          : u32 = 13 | Statement;
pub const ForOfStatement          : u32 = 13 | Statement;

pub const FunctionDeclaration     : u32 = 13 | Declaration | Function;
pub const VariableDeclaration     : u32 = 14 | Declaration;
