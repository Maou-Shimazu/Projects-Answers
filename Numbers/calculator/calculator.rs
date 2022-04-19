// Made by dquat - an improved calculator in Rust
// there are possibly many things that can be fixed to either improve code-style or bugs
// will compile without errors on Rust v1.60.0

extern crate core;

use std::iter::Peekable;
use std::ops::Range;
use std::str::Chars;
use std::collections::HashMap;
use std::fmt::{ Debug, Display, Formatter };
use std::rc::Rc;

/// Operators used in this calculator
#[derive(Copy, Clone, Debug, PartialEq)]
enum Operator {
    Add, // add (+)
    Sub, // subtract (-)
    Mul, // multiply (*)
    Div, // divide (/)
    Mod, // modulus (%)
    Exp, // exponent (^)
    Asn, // assign (=)
}

impl Display for Operator {
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Div => "/",
            Operator::Mul => "*",
            Operator::Mod => "%",
            Operator::Exp => "^",
            Operator::Asn => "=",
        })
    }
}

/// Types of tokens created by the [`Lexer`]
#[derive(Copy, Clone, Debug, PartialEq)]
enum Type {
    Operator(Operator),
    Number  (f64),

    Identifier,
    ParenRight,
    ParenLeft,
    Semicolon,

    Invalid,
}

impl Type {
    pub fn operator(&self) -> Operator {
        match self {
            Type::Operator(op) => *op,
            _ => panic!("The type is not of type `Operator`! It is: {:?}", &self),
        }
    }
}

/// A copy-able version of Range<usize> in order to avoid unnecessary complicating of code
///
/// start -> the start position of the span
///
/// end -> the end position of the span
#[derive(Clone, Copy, PartialEq)]
struct Span {
    start : usize,
    end   : usize,
}

impl Span {
    pub fn from(start : usize, end : usize) -> Self {
        Self { start, end }
    }

    pub fn rng(rng : Range<usize>) -> Self {
        Self { start : rng.start, end : rng.end }
    }

    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }
}

impl Debug for Span {
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

/// This stores the type and the span of the given lexed token
#[derive(Clone, Copy, Debug)]
struct Token {
    token_type  : Type,
    range       : Span,
}

impl Token {
    pub fn show(&self, source : &str) -> String {
        format! (
            "Token {{ Type: {:?}, Pos: {:?}, Value: {:?} }}",
            self.token_type,
            self.range,
            &source[self.range.range()]
        )
    }
}

/// A simple implementation of a visual errorer. Assumes that the source has no newlines
struct Err<'a> {
    message  : &'a str,
    range    : Span,
    src      : &'a str,
    side_msg : &'a str,
}

impl<'a> Err<'a> {
    pub fn from(src : &'a str, message : &'a str, side_msg : &'a str, range : Span) -> Self {
        Self { message, src, range, side_msg }
    }

    pub fn show(&self) -> String {
        format!(
            "Error: {}\n{}\n{}{}{}{}",
            self.message,
            self.src,
            " ".repeat(self.range.start),
            "^".repeat(self.range.end - self.range.start),
            if self.side_msg.is_empty() { "" } else { " - " },
            self.side_msg,
        )
    }
}

/// Lexes the given input and returns [`Token`]s for the next stage of text processing, the [`Parser`]
#[derive(Debug, Clone)]
struct Lexer<'a> {
    pub src : &'a str,
    pub pos : usize,
    pub it  : Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn from(source : &'a str) -> Self {
        Self {
            src : source,
            pos : 0,
            it  : source.chars().peekable()
        }
    }

    fn take_while(&mut self, mut predicate : impl FnMut(char) -> bool) -> Range<usize> {
        let start   = self.pos;
        let mut end = start;
        while let Some(c) = self.it.next_if(|&c| predicate(c)) {
            end += c.len_utf8();
        }
        self.pos = end;
        start..end
    }

    fn adv(&mut self) -> Option<char> {
        let next = self.it.next()?;
        self.pos += next.len_utf8();
        Some(next)
    }

    fn char_token(&mut self, token_type : Type) -> Option<Token> {
        let start = self.pos;
        self.adv();
        Some(Token { token_type, range : Span::from(start, self.pos) })
    }

    #[inline]
    pub fn next(&mut self) -> Option<Token> {
        match *self.it.peek()? {
            // skip whitespace
            c if c.is_whitespace() => {
                self.adv();
                self.next()
            }

            // get numbers, allows trailing and ending decimal points like .6 and 6.
            c if c.is_digit(10) || c == '.' => {
                let mut seen_dot = false;
                let range =
                    self.take_while(|c| {
                        if c == '.' {
                            if seen_dot { return false; }
                            seen_dot = true;
                            return true;
                        }
                        c.is_digit(10)
                    });
                let (start, end) = (range.start, range.end);
                let mut string = (&self.src[start..end]).to_owned();
                if string == "." {
                    println!("{}", Err::from(
                        self.src,
                        "Expected a number, found only a dot (`.`)!",
                        "Add a number to this dot on either side",
                        Span::rng(start..end)
                    ).show());
                    string.push('0');
                }
                let number = string.parse::<f64>().ok()?;
                Some(Token {
                    token_type : Type::Number(number),
                    range      : Span::from(start, end)
                })
            },

            c if c.is_alphanumeric() || matches!(c, '_' | '\'' | '~' | '#')  => {
                // allows identifiers such as `_919#`, `_~#.`, `ident`, `_'_~_.` etc
                let range =
                    self.take_while(|c|
                        c.is_alphanumeric() || matches!(c, '_' | '~' | '#' | '\'')
                    );
                Some(Token {
                    token_type : Type::Identifier,
                    range      : Span::rng(range),
                })
            },

            '(' => self.char_token(Type::ParenLeft),
            ')' => self.char_token(Type::ParenRight),
            ';' => self.char_token(Type::Semicolon),

            '+' => self.char_token(Type::Operator(Operator::Add)),
            '-' => self.char_token(Type::Operator(Operator::Sub)),
            '/' => self.char_token(Type::Operator(Operator::Div)),
            '*' => self.char_token(Type::Operator(Operator::Mul)),
            '%' => self.char_token(Type::Operator(Operator::Mod)),
            '^' => self.char_token(Type::Operator(Operator::Exp)),
            '=' => self.char_token(Type::Operator(Operator::Asn)),

            _   => self.char_token(Type::Invalid),
        }
    }
}

/// This is the enum that the parser uses to generate the syntax tree
///
/// BinaryOp -> a binary operator: 1 + 2, 2 / 3, etc.
///
/// UnaryOp -> a unary operator: -1, +2, etc.
///
/// Number -> a number: 5, 10., 9.19, .12, etc.
///
/// Invoke -> a variable that has been called
///
/// Assign -> a variable that has been created or re-assigned a value
///
/// Invalid -> an invalid statement or token
#[derive(Debug)]
enum AST<'a> {
    BinaryOp { lhs : Rc<AST<'a>>, rhs : Rc<AST<'a>>, op : Operator, span : Span },
    UnaryOp  {                    rhs : Rc<AST<'a>>, op : Operator, span : Span },
    Number   { value : f64,                                         span : Span },
    Invoke   { value : &'a str,                                     span : Span },
    Assign   { value : &'a str,   rhs : Rc<AST<'a>>,                span : Span },
    Invalid  {                                                      span : Span },
}

impl AST<'_> {
    pub fn span(&self) -> Span {
        match self {
            AST::BinaryOp { span, .. } => *span,
            AST::UnaryOp  { span, .. } => *span,
            AST::Number   { span, .. } => *span,
            AST::Invoke   { span, .. } => *span,
            AST::Assign   { span, .. } => *span,
            AST::Invalid  { span     } => *span,
        }
    }
}

impl Display for AST<'_> {
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AST::BinaryOp { lhs, rhs, op, .. } =>
                write!(f, "({} {} {})", *lhs, *op, *rhs),

            AST::UnaryOp { rhs, op, .. } =>
                write!(f, "[{}{}]", *op, *rhs),

            AST::Assign { value, rhs, .. } =>
                write!(f, "[{{{}}} = {}]", value, rhs),

            AST::Number { value, .. } => write!(f, "{}", value),

            AST::Invoke { value, .. } => write!(f, "{{{}}}", value),

            AST::Invalid { .. } => write!(f, "Invalid Syntax"),
        }
    }
}

/// This is where most of the computation occurs, and the parser generates a syntax tree for the given input
struct Parser<'a> {
    tokens : Vec<Token>,
    exprs  : Vec<Rc<AST<'a>>>,
    token  : Token,
    error  : bool,
    pos    : usize,
    src    : &'a str,
}

impl<'a> Parser<'a> {

    pub fn from(tokens : Vec<Token>, src : &'a str) -> Option<Self> {
        Some(Self {
            exprs  : Vec::with_capacity(tokens.len()),
            token  : *tokens.get(0)?,
            error  : false,
            pos    : 0,
            tokens,
            src,
        })
    }

    pub fn parse(mut self) -> (Vec<Rc<AST<'a>>>, bool){
        while !self.eof() {
            let expr = self.expression();
            self.exprs.push(expr);
        }
        (self.exprs, self.error)
    }

    fn eof(&mut self) -> bool { self.tokens.get(self.pos).is_none() }

    fn adv(&mut self) -> Option<Token> {
        self.pos += 1;
        let &token = self.tokens.get(self.pos)?;
        self.token = token;
        Some(token)
    }

    fn expression(&mut self) -> Rc<AST<'a>> {
        let term = self.term();
        if self.token.token_type == Type::Semicolon {
            self.adv();
        } else if !self.eof() {
            // tell user about semicolons
            let end = term.span().end;
            let mut string = self.src.to_string();
            string.insert(end, ';');
            let string = format!(
                "Warn: {}\n{}\n{}^ - Put this semicolon next time to reduce ambiguity",
                "Add a semicolon after statements to separate them. \n\
                 Some statements like `5.5.5` get implicitly separated to `5.5; 5` but not all do!\n\
                 So be sure to add semicolons to reduce ambiguity.",
                string,
                " ".repeat(end),
            );
            println!("{}", string);
        }
        term
    }

    fn term(&mut self) -> Rc<AST<'a>> {
        let mut result = self.factor();
        while [Type::Operator(Operator::Add), Type::Operator(Operator::Sub)]
            .contains(&self.token.token_type) && !self.eof() {
            let op = self.token.token_type.operator();
            let start = self.token.range.start;
            self.adv();
            if self.eof() {
                self.err(
                    format!("Expected a expression or value after `{}`!", op),
                    "Add an expression or value here. Ex: 2",
                    Span::from(start + 1, self.token.range.end + 1)
                );
            } else {
                let factor = self.factor();
                let span = Span::from(result.span().start, factor.span().end);
                result = Rc::new(AST::BinaryOp {
                    lhs : Rc::clone(&result),
                    rhs : factor,
                    span,
                    op,
                });
            }
        }
        result
    }

    fn factor(&mut self) -> Rc<AST<'a>> {
        let mut result = self.exponent();
        while [Type::Operator(Operator::Mul), Type::Operator(Operator::Div), Type::Operator(Operator::Mod)]
            .contains(&self.token.token_type) && !self.eof() {
            let op = self.token.token_type.operator();
            let start = self.token.range.start;
            self.adv();
            if self.eof() {
                self.err(
                    format!("Expected a expression or value after `{}`!", op),
                    "Add an expression or value here. Ex: 2",
                    Span::from(start + 1, self.token.range.end + 1)
                );
            } else {
                let exponent = self.exponent();
                let span = Span::from(result.span().start, exponent.span().end);
                result = Rc::new(AST::BinaryOp {
                    lhs : Rc::clone(&result),
                    rhs : exponent,
                    span,
                    op,
                });
            }
        }
        result
    }

    fn exponent(&mut self) -> Rc<AST<'a>> {
        let mut result = self.unary();
        while self.token.token_type == Type::Operator(Operator::Exp) && !self.eof() {
            let op = self.token.token_type.operator();
            let start = self.token.range.start;
            self.adv();
            if self.eof() {
                self.err(
                    "Expected a expression or value after `^`!".to_owned(),
                    "Add an expression or value here. Ex: 2",
                    Span::from(start + 1, self.token.range.end + 1)
                );
            } else {
                let unary = self.unary();
                let span = Span::from(result.span().start, unary.span().end);
                result = Rc::new(AST::BinaryOp {
                    lhs : Rc::clone(&result),
                    rhs : unary,
                    span,
                    op,
                });
            }
        }
        result
    }

    fn unary(&mut self) -> Rc<AST<'a>> {
        match self.token.token_type {
            Type::Operator(op @ (Operator::Add | Operator::Sub)) => {
                let rng = self.token.range;
                let start = self.token.range.start;
                self.adv();
                if self.eof() {
                    self.err(format!(
                        "Invalid or unexpected token: `{}`!", &self.src[rng.range()],
                    ), "", Span::from(rng.start, rng.end));
                    self.adv();
                    return Rc::new(AST::Invalid { span : Span::from(rng.start, rng.end) });
                }
                let unary = self.unary();
                Rc::new(AST::UnaryOp { span : Span::from(start, unary.span().end), rhs : unary, op })
            },

            _ => self.base(),
        }
    }

    fn base(&mut self) -> Rc<AST<'a>> {
        // this should never trigger but here just to prevent any possible infinite recursions
        if self.eof() {
            self.err(
                "Unexpected EOF (end of file)!".to_owned(),
                "", self.token.range
            );
            return Rc::new(AST::Invalid { span : self.token.range })
        };
        match self.token.token_type {
            Type::Number(value) => {
                let span = self.token.range;
                self.adv();
                Rc::new(AST::Number { value, span })
            },

            Type::ParenLeft => {
                let start = self.token.range.start;
                self.adv();
                let expr = self.term();
                if self.token.token_type != Type::ParenRight {
                    self.err(
                        "Expected parenthesis to be closed!".to_owned(),
                        "Add a parenthesis at the end of your expression",
                        Span::from(start, self.token.range.end)
                    );
                }
                self.adv();
                expr
            }

            Type::Identifier => {
                let start = self.token.range.start;
                let value = &self.src[self.token.range.range()];
                let mut ast = Rc::new(AST::Invoke { value, span : self.token.range });
                self.adv();
                if self.token.token_type == Type::Operator(Operator::Asn) {
                    self.adv();
                    if self.eof() {
                        self.err(
                            "Expected a value but found nothing!".to_owned(),
                            "Add a value here. Ex: 2",
                            Span::from(self.token.range.start + 1, self.token.range.end + 1)
                        );
                        return Rc::new(AST::Invalid { span : Span::from(start, self.token.range.end) });
                    }
                    ast = Rc::new(AST::Assign { value, rhs : self.term(), span : self.token.range })
                }
                ast
            }

            _ => {
                let span = self.token.range;
                self.err(format!(
                    "Invalid or unexpected token: `{}`!",
                    &self.src[span.range()],
                ), "", span);
                self.adv();
                Rc::new(AST::Invalid { span })
            },
        }
    }

    fn err(&mut self, message : String, smessage : &str, range : Span) {
        self.error = true;
        println!("{}",
                 Err::from(
                     self.src,
                     message.as_str(),
                     smessage,
                     range
                 ).show()
        );
    }
}

/// These are what the [`Interpreter`] can return.
///
/// Number -> a number (f64)
///
/// Invalid -> Returned when a runtime error occurs
#[derive(Copy, Clone)]
enum Results {
    Number (f64),
    Invalid,
}

impl Results {
    pub fn number(&self) -> Option<f64> {
        match self {
            Results::Number(val) => Some(*val),
            _ => None
        }
    }
}

/// Interprets the parsed AST structure and returns a [`Results`]
struct Interpreter<'a> {
    vars  : HashMap<&'a str, f64>,
    src   : &'a str,
}

impl<'a> Interpreter<'a> {
    pub fn from(vars : HashMap<&'a str, f64>, src : &'a str) -> Self {
        Self { vars, src }
    }

    pub fn run(mut self, exprs : Vec<Rc<AST<'a>>>) -> (Vec<Option<Results>>, HashMap<&'a str, f64>) {
        let mut values = Vec::new();
        for expr in exprs {
            values.push(self.visit(expr));
        }
        (values, self.vars)
    }

    // crude visitor implementation
    fn visit(&mut self, ast : Rc<AST<'a>>) -> Option<Results> {
        match &*ast {
            AST::BinaryOp { lhs, rhs, op, span } =>
                self.binary(Rc::clone(lhs), Rc::clone(rhs), *op, *span),

            AST::UnaryOp {                 rhs, op, span } =>
                self.unary(Rc::clone(rhs), *op, *span),

            AST::Number { value, .. } => Some(Results::Number(*value)),

            AST::Assign { value, rhs, span } =>
                self.assign(*value, Rc::clone(rhs), *span),

            AST::Invoke { value, span } =>
                self.invoke(*value, *span),

            // should never reach here
            AST::Invalid { span } => {
                self.err("Invalid token!".to_owned(), "Remove this token", *span);
                Some(Results::Invalid)
            },
        }
    }

    fn invoke(&self, value : &str, span : Span) -> Option<Results> {
        let get = self.vars.get(value);
        Some(match get {
            Some(value) => Results::Number(*value),
            None => {
                self.err(
                    format!("Variable `{}` has no value (undefined)!", value),
                    "Remove this variable, or assign a value to it first",
                    span
                );
                Results::Invalid
            }
        })
    }

    fn assign(&mut self, value : &'a str, rhs : Rc<AST<'a>>, _ : Span) -> Option<Results> {
        let rhs = match self.visit(Rc::clone(&rhs))?.number() {
            Some(value) => value,
            None => {
                self.err(
                    "Assignment done to invalid expression!".to_owned(),
                    "Fix this expression to return a valid value", rhs.span()
                );
                return None;
            }
        };
        self.vars
            .entry(value)
            .and_modify(|v| *v = rhs)
            .or_insert(rhs);
        Some(Results::Number(rhs))
    }

    fn binary(&mut self, lhs : Rc<AST<'a>>, rhs : Rc<AST<'a>>, op : Operator, span : Span) -> Option<Results> {
        let lhs = self.visit(lhs)?.number()?;
        let rhsn = self.visit(Rc::clone(&rhs))?.number()?;
        Some(match op {
            Operator::Add => Results::Number(lhs + rhsn),
            Operator::Sub => Results::Number(lhs - rhsn),
            Operator::Mul => Results::Number(lhs * rhsn),
            Operator::Div => {
                if rhsn == 0.0 {
                    self.err(
                        "Cannot divide a number by zero!".to_owned(),
                        "Use another number here, not zero",
                        rhs.span()
                    );
                    return Some(Results::Invalid);
                }
                Results::Number(lhs / rhsn)
            },
            Operator::Exp => Results::Number(lhs.powf(rhsn)),
            Operator::Mod => {
                if rhsn == 0.0 {
                    self.err(
                        "Cannot get the remainder of a number by zero!".to_owned(),
                        "Use another number here, not zero",
                        rhs.span()
                    );
                    return Some(Results::Invalid);
                }
                Results::Number(lhs % rhsn)
            },
            _             => {
                self.err(
                    format!("Unexpected operator `{}`!", op),
                    "Remove this operator",
                    span
                );
                Results::Invalid
            },
        })
    }

    fn unary(&mut self, rhs : Rc<AST<'a>>, op : Operator, span : Span) -> Option<Results> {
        let rhs = self.visit(rhs)?.number()?;
        Some(match op {
            Operator::Add => Results::Number( rhs),
            Operator::Sub => Results::Number(-rhs),
            // should never reach here
            _ => {
                self.err("Invalid unary operator!".to_owned(), "Remove this operator", span);
                Results::Invalid
            }
        })
    }

    fn err(&self, message : String, smessage : &str, range : Span) {
        println!("{}",
                 Err::from(
                     self.src,
                     message.as_str(),
                     smessage,
                     range
                 ).show()
        );
    }
}

// read user input
fn read_input() -> String {
    use std::io::{ stdin, stdout, Write };
    let mut buffer = String::new();
    print!("calculator> ");
    stdout().flush().ok();
    stdin().read_line(&mut buffer).ok();
    if buffer.ends_with('\n') { buffer.pop(); }
    buffer
}

fn main() {
    let commands =
        "Type `quit` | `exit` to exit the program.\n\
         Type `token` to toggle the display of processed text as tokens.\n\
         Type `tree` to toggle the display of processed text as abstract syntax trees (AST).\n\
         Type `time` to toggle the display of compute time from lexing to interpreting the input\n\
         Type `compact` to toggle compact results. Results will show horizontally in compact mode.\n\
         - Note: This also includes the time spent printing out the trees, tokens and errors\n\
         Type `delete [variable], [variable], ...` to delete variables.\n\
         Type `repeat` | `prev` to re-run the previous computation.\n\
         Type `command` to display this message.";
    let instructions =
        "Semicolons separate expressions i.e. `5; 10` would result in 2 answers, 5 and 10\n\
         You can do basic math with the operators:\n plus `+`\n minus `-`\n divide `/`\n multiply `*`\n exponent `^`\n remainder `%`\n\
         You can create variables and access them: `var = 5; var + 1` -> 5 and 6\n\
         Shorthand of above: `(var = 5) + 1` or `1 + var = 5` -> 6\n\
         Note: Variables that have the same name as a command cannot be used individually since they will invoke the command instead\n\
         Type `help` to display all of the above messages";
    println!("Welcome to this calculator!\n{}\n{}", commands, instructions);
    let mut show_lexed  = false;
    let mut show_parsed = false;
    let mut show_time = false;
    let mut compact_mode = false;
    let mut prev = "";
    // store variables persistently until the end of the session
    let mut vars = HashMap::new();
    loop {
        let mut src = read_input();
        // commands
        match &*src.trim().to_lowercase() {
            "quit" | "exit" => {
                println!("Bye bye!");
                std::process::exit(0);
            },

            "token" => {
                show_lexed = !show_lexed;
                println!("Now {} showing lexemes.", if show_lexed { "" } else { "not" });
                continue;
            }

            "tree" => {
                show_parsed = !show_parsed;
                println!("Now{} showing parsed trees.", if show_parsed { "" } else { " not" });
                continue;
            }

            "time" => {
                show_time = !show_time;
                println!("Now{} showing elapsed time for computing.", if show_time { "" } else { " not" });
                continue;
            }

            "compact" => {
                compact_mode = !compact_mode;
                println!("Now{} using compact mode.", if compact_mode { "" } else { " not" });
                continue;
            }

            "command" => {
                println!("{}", commands);
                continue;
            }

            "help" => {
                println!("{}\n{}", commands, instructions);
                continue;
            }

            v if v.starts_with("delete") => {
                let vals = v.split_once(' ').unwrap().1.split(',');
                for val in vals {
                    let trimmed = val.trim();
                    if trimmed.is_empty() { continue; }
                    if vars.remove(val).is_some() {
                        println!("Removed variable `{}`.", trimmed);
                    } else {
                        println!("Variable `{}` does not exist.", trimmed);
                    };
                }
                println!("Done removing variables specified!");
                continue;
            }

            "repeat" | "prev" => {
                if !prev.trim().is_empty() {
                    println!("Running previous input: {}", prev);
                    src = prev.to_owned();
                } else {
                    println!("Nothing to repeat!");
                    continue;
                }
            }

            _ => (),
        };
        let elapsed = std::time::Instant::now();
        // leaks source string so that it lives long enough (until the program ends) for the variables to be persistent
        let src = Box::leak(src.into_boxed_str());
        let mut lexer = Lexer::from(src);
        let mut tokens = Vec::with_capacity(src.len());
        while let Some(token) = lexer.next() {
            tokens.push(token);
            if show_lexed {
                println!("{}", token.show(src));
            }
        }
        let parser = Parser::from(tokens, src);
        // parse if not empty
        if let Some(parser) = parser {
            let (exprs, errored) = parser.parse();
            if show_parsed {
                for expr in &exprs {
                    println!("Tree: {}", expr);
                }
            }
            // run interpreter if no errors were found
            if !errored {
                let expr_len = exprs.len();
                let interpreter = Interpreter::from(vars, src);
                let (values, new_vars) =
                    interpreter.run(exprs);
                let time = elapsed.elapsed();

                let mut vec = Vec::with_capacity(expr_len);
                for value in values {
                    let out = match value {
                        Some(Results::Number(num)) => {
                            if num.is_infinite() || num.is_nan() {
                                "Invalid - Infinite | Not a Number".to_owned()
                            } else {
                                num.to_string()
                            }
                        },
                        _ => "Runtime Error".to_owned(),
                    };
                    if !compact_mode {
                        println!("Result: {}", out);
                    } else {
                        vec.push(out);
                    }
                }
                if compact_mode {
                    println!("Result(s): [ {} ]", vec.join(", "));
                }
                if show_time {
                    println!("Time taken to compute: {:?}", time);
                }
                vars = new_vars;
            }
        } else {
            println!("Empty Input.\nResult: 0");
        }
        prev = src;
    }
}
// 741 loc - cloc
