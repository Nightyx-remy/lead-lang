use crate::cnode::{CNode, COperator, CValueNode};
use crate::Positioned;

pub struct Generator {
    ast: Vec<Positioned<CNode>>,
    index: usize
}

impl Generator {

    pub fn new(ast: Vec<Positioned<CNode>>) -> Self {
        return Self {
            ast,
            index: 0
        }
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn current(&self) -> Option<Positioned<CNode>> {
        return self.ast.get(self.index).cloned();
    }

    fn generate_operator(&mut self, operator: Positioned<COperator>) -> String {
        match operator.data {
            COperator::Plus => '+'.to_string(),
            COperator::Minus => '-'.to_string(),
            COperator::Multiply => '*'.to_string(),
            COperator::Divide => '/'.to_string(),
            COperator::Remainder => '%'.to_string(),
            COperator::And => "&&".to_string(),
            COperator::Or => "||".to_string(),
            COperator::LeftShift => "<<".to_string(),
            COperator::RightShift => ">>".to_string(),
            COperator::BitAnd => '&'.to_string(),
            COperator::BitOr => '|'.to_string(),
            COperator::BitXor => '^'.to_string(),
            COperator::Greater => '>'.to_string(),
            COperator::GreaterOrEqual => ">=".to_string(),
            COperator::Less => '<'.to_string(),
            COperator::LessOrEqual => "<=".to_string(),
            COperator::Equal => "==".to_string(),
            COperator::NotEqual => "!=".to_string(),
        }
    }

    fn generate_bin_op(&mut self, left: Positioned<CNode>, operator: Positioned<COperator>, right: Positioned<CNode>) -> String {
        let mut out = String::new();
        out.push('(');
        out.push_str(self.generate_node(left).as_str());
        out.push(' ');
        out.push_str(self.generate_operator(operator).as_str());
        out.push(' ');
        out.push_str(self.generate_node(right).as_str());
        out.push(')');
        return out;
    }

    fn generate_value(&mut self, value: CValueNode) -> String {
        match value {
            CValueNode::Number(num) => num,
            CValueNode::String(str) => {
                let mut out = String::new();
                out.push('\"');
                out.push_str(str.as_str());
                out.push('\"');
                return out;
            }
            CValueNode::Char(chr) => {
                let mut out = String::new();
                out.push('\'');
                out.push_str(chr.as_str());
                out.push('\'');
                return out;
            }
        }
    }

    fn generate_node(&mut self, node: Positioned<CNode>) -> String {
        return match node.data {
            CNode::BinaryOperation(left, op, right) => self.generate_bin_op(*left, op, *right),
            CNode::Value(value) => self.generate_value(value),
        }
    }

    pub fn generate(&mut self) -> String {
        let mut str = String::new();

        while let Some(current) = self.current() {
            str.push_str(self.generate_node(current).as_str());
            str.push_str(";\n");
            self.advance();
        }

        return str;
    }

}