use crate::tokenizer::{Token, TokenKind};
use std::iter::Peekable;
use std::slice::Iter;
use std::vec::Vec;

#[derive(Debug, PartialEq)]
pub enum Node {
    Number(u32),                                 // 整数
    Add { lhs: Box<Node>, rhs: Box<Node> },      // +
    Sub { lhs: Box<Node>, rhs: Box<Node> },      // -
    Mul { lhs: Box<Node>, rhs: Box<Node> },      // *
    Div { lhs: Box<Node>, rhs: Box<Node> },      // /
    Equal { lhs: Box<Node>, rhs: Box<Node> },    // ==
    NotEqual { lhs: Box<Node>, rhs: Box<Node> }, // !=
    Lt { lhs: Box<Node>, rhs: Box<Node> },       // <
    Lte { lhs: Box<Node>, rhs: Box<Node> },      // <=
}

struct ParserContext<'a> {
    token_iter: Peekable<Iter<'a, Token<'a>>>,
}

impl<'a> ParserContext<'a> {
    pub fn new(token_iter: Peekable<Iter<'a, Token<'a>>>) -> ParserContext<'a> {
        return ParserContext { token_iter };
    }

    pub fn consume(&mut self, op: &str) -> bool {
        if let Some(token) = self.token_iter.peek() {
            match token.kind {
                TokenKind::Reserved(token_op) if token_op == op => {
                    self.token_iter.next();
                    return true;
                }
                _ => {}
            }
        }
        return false;
    }

    pub fn expect(&mut self, op: &str) {
        if self.consume(op) {
            return;
        }
        let token = self.token_iter.peek().unwrap();
        token.report_error(&format!("'{}' ではありません", op));
    }

    pub fn expect_number(&mut self) -> u32 {
        let token = self.token_iter.next().unwrap();
        match token.kind {
            TokenKind::Number(n) => return n,
            _ => token.report_error("数ではありません"),
        }
    }
}

fn primary(ctx: &mut ParserContext) -> Node {
    if ctx.consume("(") {
        let node = expr(ctx);
        ctx.expect(")");
        return node;
    }
    return Node::Number(ctx.expect_number());
}

fn unary(ctx: &mut ParserContext) -> Node {
    if ctx.consume("+") {
        return primary(ctx);
    }
    if ctx.consume("-") {
        return Node::Sub {
            lhs: Box::new(Node::Number(0)),
            rhs: Box::new(primary(ctx)),
        };
    }
    return primary(ctx);
}

fn mul(ctx: &mut ParserContext) -> Node {
    let mut node = unary(ctx);

    loop {
        if ctx.consume("*") {
            node = Node::Mul {
                lhs: Box::new(node),
                rhs: Box::new(unary(ctx)),
            };
        } else if ctx.consume("/") {
            node = Node::Div {
                lhs: Box::new(node),
                rhs: Box::new(unary(ctx)),
            };
        } else {
            return node;
        }
    }
}

fn add(ctx: &mut ParserContext) -> Node {
    let mut node = mul(ctx);

    loop {
        if ctx.consume("+") {
            node = Node::Add {
                lhs: Box::new(node),
                rhs: Box::new(mul(ctx)),
            };
        } else if ctx.consume("-") {
            node = Node::Sub {
                lhs: Box::new(node),
                rhs: Box::new(mul(ctx)),
            };
        } else {
            return node;
        }
    }
}

fn relational(ctx: &mut ParserContext) -> Node {
    let mut node = add(ctx);

    loop {
        if ctx.consume("<") {
            node = Node::Lt {
                lhs: Box::new(node),
                rhs: Box::new(add(ctx)),
            };
        } else if ctx.consume("<=") {
            node = Node::Lte {
                lhs: Box::new(node),
                rhs: Box::new(add(ctx)),
            };
        } else if ctx.consume(">") {
            node = Node::Lt {
                lhs: Box::new(add(ctx)),
                rhs: Box::new(node),
            };
        } else if ctx.consume(">=") {
            node = Node::Lte {
                lhs: Box::new(add(ctx)),
                rhs: Box::new(node),
            };
        } else {
            return node;
        }
    }
}

fn equality(ctx: &mut ParserContext) -> Node {
    let mut node = relational(ctx);

    loop {
        if ctx.consume("==") {
            node = Node::Equal {
                lhs: Box::new(node),
                rhs: Box::new(relational(ctx)),
            };
        } else if ctx.consume("!=") {
            node = Node::NotEqual {
                lhs: Box::new(node),
                rhs: Box::new(relational(ctx)),
            };
        } else {
            return node;
        }
    }
}

fn expr(ctx: &mut ParserContext) -> Node {
    return equality(ctx);
}

pub fn parse(tokens: &Vec<Token>) -> Node {
    let mut ctx = ParserContext::new(tokens.iter().peekable());
    return expr(&mut ctx);
}
