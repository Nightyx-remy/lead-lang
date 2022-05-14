use crate::cnode::{CNode, COperator, CType, CValueNode};
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
            COperator::BitNot => '~'.to_string(),
            COperator::Greater => '>'.to_string(),
            COperator::GreaterOrEqual => ">=".to_string(),
            COperator::Less => '<'.to_string(),
            COperator::LessOrEqual => "<=".to_string(),
            COperator::Equal => "==".to_string(),
            COperator::NotEqual => "!=".to_string(),
            COperator::Not => '!'.to_string(),
            COperator::Ref => '&'.to_string(),
            COperator::Deref => '*'.to_string(),
        }
    }

    fn generate_bin_op(&mut self, left: Positioned<CNode>, operator: Positioned<COperator>, right: Positioned<CNode>) -> String {
        let mut out = String::new();
        out.push('(');
        out.push_str(self.generate_node(left).1.as_str());
        out.push(' ');
        out.push_str(self.generate_operator(operator).as_str());
        out.push(' ');
        out.push_str(self.generate_node(right).1.as_str());
        out.push(')');
        return out;
    }

    fn generate_unary_op(&mut self, operator: Positioned<COperator>, value: Positioned<CNode>) -> String {
        let mut out = String::new();
        out.push('(');
        out.push_str(self.generate_operator(operator).as_str());
        out.push_str(self.generate_node(value).1.as_str());
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

    fn generate_type(&mut self, data_type: Positioned<CType>) -> String {
        return match data_type.data {
            CType::Byte => "byte".to_string(),
            CType::UnsignedByte => "unsigned byte".to_string(),
            CType::Short => "short".to_string(),
            CType::UnsignedShort => "unsigned short".to_string(),
            CType::Int => "int".to_string(),
            CType::UnsignedInt => "unsigned int".to_string(),
            CType::Long => "long".to_string(),
            CType::UnsignedLong => "unsigned long".to_string(),
            CType::Char => "char".to_string(),
            CType::Ref(inner) => {
                let mut str = String::new();
                str.push_str(self.generate_type(*inner).as_str());
                str.push('*');
                str
            }
            CType::ConstRef(inner) => {
                let mut str = String::new();
                str.push_str("const ");
                str.push_str(self.generate_type(*inner).as_str());
                str.push('*');
                str
            }
            CType::Void => "void".to_string(),
        }
    }

    fn generate_variable_def(&mut self, data_type: Positioned<CType>, is_const: bool, name: Positioned<String>, value: Option<Box<Positioned<CNode>>>) -> String {
        let mut str = String::new();

        str.push_str(self.generate_type(data_type).as_str());
        if is_const {
            str.push_str(" const");
        }
        str.push(' ');
        str.push_str(name.data.as_str());
        if let Some(value) = value {
            str.push_str(" = ");
            str.push_str(self.generate_node(*value).1.as_str());
        }

        return str;
    }

    fn generate_cast(&mut self, left: Positioned<CNode>, right: Positioned<CType>) -> String {
        let mut str = String::new();
        str.push('(');
        str.push_str(self.generate_type(right).as_str());
        str.push_str(") ");
        str.push_str(self.generate_node(left).1.as_str());
        return str;
    }

    fn generate_variable_call(&mut self, id: Positioned<String>) -> String {
        return id.data.clone();
    }

    fn generate_variable_assignment(&mut self, deref: bool, id: Positioned<String>, value: Positioned<CNode>) -> String {
        let mut str = String::new();
        if deref {
            str.push_str("*");
        }
        str.push_str(id.data.as_str());
        str.push_str(" = ");
        str.push_str(self.generate_node(value).1.as_str());
        return str;
    }

    fn generate_function_definition(&mut self, return_type: Positioned<CType>, name: Positioned<String>, params: Vec<(Positioned<CType>, Positioned<String>)>, body: Vec<Positioned<CNode>>) -> String {
        let mut str = String::new();
        str.push_str(self.generate_type(return_type).as_str());
        str.push_str(" ");
        str.push_str(name.data.as_str());
        str.push_str("(");
        let mut first = true;
        for (param_type, param_name) in params.iter() {
            if !first {
                str.push_str(", ");
            }
            str.push_str(self.generate_type(param_type.clone()).as_str());
            str.push_str(" ");
            str.push_str(param_name.data.as_str());
            first = false;
        }
        str.push_str(") {\n");
        for node in body {
            for line in self.generate_node(node).1.lines() {
                str.push_str("\t");
                str.push_str(line);
                str.push_str(";\n");
            }
        }
        str.push_str("}");
        str
    }

    fn generate_function_call(&mut self, name: Positioned<String>, params: Vec<Positioned<CNode>>) -> String {
        let mut str = String::new();
        str.push_str(name.data.as_str());
        str.push_str("(");
        let mut first = true;
        for param in params.iter() {
            if !first {
                str.push_str(", ");
            }
            str.push_str(self.generate_node(param.clone()).1.as_str());
            first = false;
        }
        str.push_str(")");
        str
    }

    fn generate_return(&mut self, node: Positioned<CNode>) -> String {
        let mut str = String::new();
        str.push_str("return ");
        str.push_str(self.generate_node(node).1.as_str());
        str
    }

    fn generate_include(&mut self, file: Positioned<String>) -> String {
        return format!("#include <{}.h>", file.data);
    }

    fn generate_node(&mut self, node: Positioned<CNode>) -> (bool, String) {
        return match node.data.clone() {
            CNode::BinaryOperation(left, op, right) => (true, self.generate_bin_op(*left, op, *right)),
            CNode::UnaryOperation(operator, value) => (true, self.generate_unary_op(operator, *value)),
            CNode::Value(value) => (true, self.generate_value(value)),
            CNode::VariableDef(data_type, is_const, name, value) => (true, self.generate_variable_def(data_type, is_const, name, value)),
            CNode::Casting(left, right) => (true, self.generate_cast(*left, right)),
            CNode::VariableCall(id) => (true, self.generate_variable_call(node.convert(id))),
            CNode::VariableAssignment(deref, id, value) => (true, self.generate_variable_assignment(deref, id, *value)),
            CNode::FunctionDefinition(return_type, name, params, body) => (false, self.generate_function_definition(return_type, name, params, body)),
            CNode::FunctionCall(name, params) => (true, self.generate_function_call(name, params)),
            CNode::Return(node) => (true, self.generate_return(*node)),
            CNode::Include(file) => (false, self.generate_include(file)),
        }
    }

    pub fn generate(&mut self) -> String {
        let mut str = String::new();

        while let Some(current) = self.current() {
            let res = self.generate_node(current);
            str.push_str(res.1.as_str());
            if res.0 {
                str.push_str(";");
            }
            str.push_str("\n");
            self.advance();
        }

        return str;
    }

}