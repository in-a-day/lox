#![allow(unused_imports)]
use crate::{
    expr::{
        BinaryExpr, Expr, ExprVisitor, GroupingExpr, LiteralExpr, UnaryExpr, Visitor,
    },
    token::{Token, TokenType, LiteralValue},
};

pub struct PrintVisitor;
impl ExprVisitor<String> for PrintVisitor {
    fn visit_literal(&self, expr: &LiteralExpr) -> String {
        (match &expr.value {
            LiteralValue::String(v) => v.to_owned(),
            LiteralValue::Nubmer(v) => v.to_string(),
            LiteralValue::Bool(v) => v.to_string(),
            LiteralValue::Nil => "nil".to_owned(),
        }) + " "
    }

    fn visit_unary(&self, expr: &UnaryExpr) -> String {
        expr.operator.lexeme.to_owned() + " " + &expr.right.visit(self)
    }

    fn visit_binary(&self, expr: &BinaryExpr) -> String {
        expr.left.visit(self) + &expr.operator.lexeme + " " + &expr.right.visit(self)
    }

    fn visit_grouping(&self, expr: &GroupingExpr) -> String {
        "( ".to_owned() + &expr.expression.visit(self) + ")" + " "
    }
}

pub struct AstVisitor;
impl ExprVisitor<String> for AstVisitor {
    fn visit_literal(&self, expr: &LiteralExpr) -> String {
        match &expr.value {
            LiteralValue::Nil => "nil".to_string(),
            LiteralValue::Bool(v) => v.to_string(),
            LiteralValue::Nubmer(v) => v.to_string(),
            LiteralValue::String(v) => v.to_string(),
        }
    }

    fn visit_unary(&self, expr: &UnaryExpr) -> String {
        self.parenthesize(&expr.operator.lexeme, &[&expr.right])
    }

    fn visit_binary(&self, expr: &BinaryExpr) -> String {
        self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right])
    }

    fn visit_grouping(&self, expr: &GroupingExpr) -> String {
        self.parenthesize("Group", &[&expr.expression])
    }
}

impl AstVisitor {
    #[allow(dead_code)]
    fn parenthesize(&self, lexeme: &str, exprs: &[&Expr]) -> String {
        let mut res = String::new();
        res.push('(');
        res.push_str(lexeme);
        for expr in exprs {
            res.push(' ');
            res.push_str(&expr.visit(self));
        }
        res.push(')');

        res
    }
}

pub struct RpnVisitor;
impl ExprVisitor<String> for RpnVisitor {
    fn visit_literal(&self, expr: &LiteralExpr) -> String {
        (match &expr.value {
            LiteralValue::String(s) => s.to_owned(),
            LiteralValue::Nubmer(n) => n.to_string(),
            LiteralValue::Bool(b) => b.to_string(),
            LiteralValue::Nil => "nil".to_owned(),
        }) + " "
    }

    fn visit_unary(&self, expr: &UnaryExpr) -> String {
        expr.right.visit(self) + &expr.operator.lexeme + " "
    }

    fn visit_binary(&self, expr: &BinaryExpr) -> String {
        expr.left.visit(self) + &expr.right.visit(self) + &expr.operator.lexeme + " "
    }

    fn visit_grouping(&self, expr: &GroupingExpr) -> String {
        expr.expression.visit(self)
    }
}

#[test]
fn print_visitor_test() {
    // 1
    let one = Expr::Literal(Box::new(LiteralExpr {
        value: LiteralValue::Nubmer(1.0),
    }));
    // 2
    let two = Expr::Literal(Box::new(LiteralExpr {
        value: LiteralValue::Nubmer(2.0),
    }));
    // 1 + 2
    let b = Expr::Binary(Box::new(BinaryExpr {
        left: one,
        right: two,
        operator: Token::new_not_literal(TokenType::Plus, "+".to_owned(), 1),
    }));
    // (1 + 2)
    let g = Expr::Grouping(Box::new(GroupingExpr { expression: b }));
    // - (1 + 2)
    let u = Expr::Unary(Box::new(UnaryExpr {
        operator: Token::new_not_literal(TokenType::Minus, "-".to_owned(), 1),
        right: g,
    }));

    println!("{}", u.visit(&PrintVisitor));
}

#[test]
fn rpn_test() {
    // 1
    let one = Expr::Literal(Box::new(LiteralExpr {
        value: LiteralValue::Nubmer(1.0),
    }));
    // 2
    let two = Expr::Literal(Box::new(LiteralExpr {
        value: LiteralValue::Nubmer(2.0),
    }));
    // 3
    let three = Expr::Literal(Box::new(LiteralExpr {
        value: LiteralValue::Nubmer(3.0),
    }));
    // 4
    let four = Expr::Literal(Box::new(LiteralExpr {
        value: LiteralValue::Nubmer(4.0),
    }));
    // (1 + 2)
    let a = Expr::Grouping(Box::new(GroupingExpr {
        expression: Expr::Binary(Box::new(BinaryExpr {
            left: one,
            right: two,
            operator: Token::new_not_literal(TokenType::Plus, "+".to_owned(), 1),
        })),
    }));
    // (4 - 3)
    let b = Expr::Grouping(Box::new(GroupingExpr {
        expression: Expr::Binary(Box::new(BinaryExpr {
            left: four,
            right: three,
            operator: Token::new_not_literal(TokenType::Plus, "-".to_owned(), 1),
        })),
    }));
    // (1 + 2) * (4 - 3)
    let c = Expr::Binary(Box::new(BinaryExpr {
        left: a,
        right: b,
        operator: Token::new_not_literal(TokenType::Star, "*".to_owned(), 1),
    }));

    println!("{}", c.visit(&RpnVisitor));
}
