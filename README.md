# lead-lang

Here is my attempt to make a transpiler from Lead (a language I created) to C.

## Versions History

### Version [0.3.0] (current)
- More operation check
- Function Call
- Function Definition
- Return statement
- Variable scope accessibility
- Void type

### Version [0.2.3] 
- Reference, Const reference, Dereference
- Comptime string type

### Version [0.2.2] 
- Variable Call
- Variable Assignment
- Unary BitNot operator

### Version [0.2.1] 
- Casting

### Version [0.2.0]
- Variable Definition
  - Var
  - Let
  - Const
- Basic type (+ comptime version)
  - u8, u16, u32, u64
  - i8, i16, i32, i64
  - char
  - boolean

### Version [0.1.3] 
- Unary operation
  - Minus
  - Plus
  - Not
- Support of xor operation
- Removal of comptime binary optimization 

### Version [0.1.2] 
- More binary operation + comptime optimization
  - bit and               
  - bit or                
  - bit xor               
  - left shift            
  - right shift          
  - greater             
  - greater or equal     
  - less                 
  - less or equal        
  - equal                 
  - not equal 
- Support of parenthesis in binary operation            

### Version [0.1.1] 
- Boolean binary operation
  - and
  - or
  - xor (only supported with comptime values)
- Optimization of comptime boolean operation

### Version [0.1] 
- Basic comptime values
  - Number
  - String
  - Char
  - Boolean
- Simple binary operations
  - Add
  - Subtract
  - Multiply
  - Divide
  - Remainder
- Optimization of comptime number operation
