use std::{fs::File, io::Write, str::FromStr};

#[derive(Debug)]
struct Type {
    name: String,
    fields: Vec<String>,
}

impl Type {
    fn snake_name(&self) -> String {
        let mut result = String::new();
        let mut prev_is_upper = false;

        for c in self.name.chars() {
            if c.is_uppercase() {
                if !prev_is_upper && !result.is_empty()  {
                    result.push('_');
                }
                prev_is_upper = true;
                result.push(c.to_ascii_lowercase());
            } else {
                prev_is_upper = false;
                result.push(c);
            }
        }

        result
    }
}

impl Default for Type {
    fn default() -> Self {
        Type {
            name: "".to_owned(),
            fields: vec![],
        }
    }
}

impl FromStr for Type {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tp = Type::default();
        let (left, right) = s.split_once(':').ok_or("parse err")?;
        tp.name = left.trim().to_owned();
        tp.fields = right.split(',').map(|s| s.trim().to_owned()).collect();

        Ok(tp)
    }
}

#[allow(dead_code)]
fn gen(path: &str) {
    let mut file = File::create(path).unwrap();

    let base_name = "Expr";
    let types: Vec<Type> = vec![
        "Literal: value: LiteralValue",
        "Unary: operator: Token, right: Expr",
        "Binary: left: Expr, right: Expr, operator: Token",
        "Grouping: expression: Expr",
    ]
    .into_iter()
    .map(|s| s.parse().unwrap())
    .collect();

    gen_base(&mut file).unwrap();
    gen_visitor(&mut file, base_name, &types).unwrap();
    gen_types(&mut file, base_name, &types).unwrap();
}

#[allow(dead_code)]
fn gen_base(writer: &mut impl Write) -> std::io::Result<()> {
    writeln!(writer, "use crate::token::{{Token, LiteralValue}};")?;
    writeln!(writer)?;
    Ok(())
}

#[allow(dead_code)]
fn gen_visitor(writer: &mut impl Write, base_name: &str, types: &Vec<Type>) -> std::io::Result<()> {
    writeln!(writer, "pub trait Visitor<R> {{")?;
    writeln!(writer, "    fn visit(&self, expr: &Expr) -> R;")?;
    writeln!(writer, "}}")?;

    writeln!(writer, "pub trait ExprVisitor<R> {{")?;
    for tp in types {
        writeln!(
            writer,
            "    fn visit_{}(&self, expr: &{}) -> R;",
            tp.snake_name(),
            tp.name.clone() + base_name,
        )?;
    }
    writeln!(writer, "}}")?;

    writeln!(writer, "impl<T, R> Visitor<R> for T where T: ExprVisitor<R> {{")?;
    writeln!(writer, "    fn visit(&self, expr: &Expr) -> R {{")?;
    writeln!(writer, "        match expr {{")?;
    for tp in types {
        writeln!(writer, "            Expr::{}(v) => self.visit_{}(v),", tp.name, tp.snake_name())?;
    }
    writeln!(writer, "        }}")?;
    writeln!(writer, "    }}")?;
    writeln!(writer, "}}")?;

    Ok(())
}

#[allow(dead_code)]
fn gen_types(writer: &mut impl Write, base_name: &str, types: &Vec<Type>) -> std::io::Result<()> {
    // enum
    writeln!(writer, "#[derive(Debug)]")?;
    writeln!(writer, "pub enum Expr {{")?;
    for tp in types {
        writeln!(writer, "    {}(Box({})),", tp.name, tp.name.clone() + base_name)?;
    }
    writeln!(writer, "}}")?;
    writeln!(writer, "impl {} {{", base_name)?;
    writeln!(writer, "    fn visit<R>(&self, visitor: &dyn Visitor<R>) -> R {{")?;
    writeln!(writer, "        visitor.visit(self)")?;
    writeln!(writer, "    }}")?;
    writeln!(writer, "}}")?;

    // struct
    for tp in types {
        writeln!(writer, "#[derive(Debug)]")?;
        writeln!(writer, "pub struct {} {{", tp.name.to_owned() + base_name)?;
        for field in &tp.fields {
            writeln!(writer, "    {},", field)?;
        }
        writeln!(writer, "}}")?;
    }

    Ok(())
}

#[test]
fn gen_test() {
    gen("./hh.rs");
}
