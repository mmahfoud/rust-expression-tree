use std::fmt::{self};

#[derive(Clone, Copy)]
enum BinaryOperation {
    Addition,
    Substraction,
    Multiplication,
    Division,
    Power,
}
impl fmt::Display for BinaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperation::Addition => write!(f, "{}", '+'),
            BinaryOperation::Substraction => write!(f, "{}", '-'),
            BinaryOperation::Multiplication => write!(f, "{}", '*'),
            BinaryOperation::Division => write!(f, "{}", '/'),
            BinaryOperation::Power => write!(f, "{}", '^'),
        }
    }
}
impl fmt::Debug for BinaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperation::Addition => write!(f, "{}", '+'),
            BinaryOperation::Substraction => write!(f, "{}", '-'),
            BinaryOperation::Multiplication => write!(f, "{}", '*'),
            BinaryOperation::Division => write!(f, "{}", '/'),
            BinaryOperation::Power => write!(f, "{}", '^'),
        }
    }
}
struct BinaryNode {
    operation: BinaryOperation,
    left: Box<Node>,
    right: Box<Node>,
}
impl fmt::Display for BinaryNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.left, self.operation, self.right)
    }
}
impl BinaryNode {
    fn evaluate(&self) -> f64 {
        let l = self.left.evaluate();
        println!("Evaluating {} = {}", self.left, l);
        let r = self.right.evaluate();
        println!("Evaluating {} = {}", self.right, r);
        println!("{} {} {}", l, self.operation, r);
        match self.operation {
            BinaryOperation::Addition => l + r,
            BinaryOperation::Substraction => l - r,
            BinaryOperation::Multiplication => l * r,
            BinaryOperation::Division => l / r,
            BinaryOperation::Power => l.powf(r),
        }
    }

    fn to_postfix(&self) -> Vec<Token> {
        let mut l = self.left.to_postfix();
        let mut r = self.right.to_postfix();
        l.append(&mut r);
        l.push(Token::Binary(self.operation));
        l
    }
}

#[derive(Clone, Copy)]
enum UnaryOperation {
    Minus,
    Exp,
    Log,
    SquareRoot,
}
impl fmt::Display for UnaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperation::Minus => write!(f, "{}", '-'),
            UnaryOperation::Exp => write!(f, "{}", "e^"),
            UnaryOperation::Log => write!(f, "{}", "ln"),
            UnaryOperation::SquareRoot => write!(f, "{}", "√"),
        }
    }
}
impl fmt::Debug for UnaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperation::Minus => write!(f, "{}", '-'),
            UnaryOperation::Exp => write!(f, "{}", "e^"),
            UnaryOperation::Log => write!(f, "{}", "ln"),
            UnaryOperation::SquareRoot => write!(f, "{}", "√"),
        }
    }
}
struct UnaryNode {
    operation: UnaryOperation,
    operand: Box<Node>,
}
impl fmt::Display for UnaryNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.operation, self.operand)
    }
}
impl UnaryNode {
    fn evaluate(&self) -> f64 {
        let v = self.operand.evaluate();
        println!("Evaluating {} = {}", self.operand, v);
        println!("{}of {}", self.operation, v);
        match self.operation {
            UnaryOperation::Minus => -v,
            UnaryOperation::Exp => v.exp(),
            UnaryOperation::Log => v.ln(),
            UnaryOperation::SquareRoot => v.sqrt(),
        }
    }

    fn to_postfix(&self) -> Vec<Token> {
        let mut v = self.operand.to_postfix();
        v.push(Token::Uniary(self.operation));
        v
    }
}

enum Node {
    Binary(BinaryNode),
    Unary(UnaryNode),
    Value(f64),
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Binary(b) => write!(f, "{}", b),
            Node::Unary(u) => write!(f, "{}", u),
            Node::Value(v) => write!(f, "{}", v),
        }
    }
}
impl Node {
    fn evaluate(&self) -> f64 {
        match self {
            Node::Binary(b) => b.evaluate(),
            Node::Unary(u) => u.evaluate(),
            Node::Value(v) => *v,
        }
    }

    fn to_postfix(&self) -> Vec<Token> {
        match self {
            Node::Binary(b) => b.to_postfix(),
            Node::Unary(u) => u.to_postfix(),
            Node::Value(v) => vec![Token::Operand(*v)],            
        }
    }
}

enum Token {
    Binary(BinaryOperation),
    Uniary(UnaryOperation),
    Operand(f64)
}
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Binary(b) => write!(f, "{:?}", b),
            Token::Uniary(u) => write!(f, "{:?}", u),
            Token::Operand(o) => write!(f, "{:?}", o)
        }
    }
}

fn main() {
    let mut expression = Node::Binary(BinaryNode {
        operation: BinaryOperation::Addition,
        left: Box::new(Node::Binary(BinaryNode {
            operation: BinaryOperation::Multiplication,
            left: Box::new(Node::Value(5.0)),
            right: Box::new(Node::Unary(UnaryNode {
                operation: UnaryOperation::Minus,
                operand: Box::new(Node::Value(3.0)),
            })),
        })),
        right: Box::new(Node::Binary(BinaryNode {
            operation: BinaryOperation::Multiplication,
            left: Box::new(Node::Value(7.0)),
            right: Box::new(Node::Value(2.0)),
        })),
    });
    expression = Node::Unary(UnaryNode {
        operation: UnaryOperation::Log,
        operand: Box::new(Node::Binary(BinaryNode {
            operation: BinaryOperation::Power,
            left: Box::new(Node::Value(2.0)),
            right: Box::new(expression),
        })),
    });

    println!("{}", expression);
    println!("\tPOSTFIX {:?}", expression.to_postfix());
    let v = expression.evaluate();
    println!("{}", v);

    println!("=============\nNew Expression");
    let n = Node::Unary(UnaryNode {
        operation: UnaryOperation::SquareRoot,
        operand: Box::new(Node::Unary(UnaryNode {
            operation: UnaryOperation::Minus,
            operand: Box::new(Node::Value(10.0)),
        })),
    });
    println!("\t{}", n);
    println!("\tPOSTFIX {:?}", n.to_postfix());
    println!("\t={:?}", n.evaluate());
}
