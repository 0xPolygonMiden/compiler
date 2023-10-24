use crate::parser::ast::*;
use crate::{ArgumentExtension, ArgumentPurpose, CallConv, FunctionIdent, Ident, Linkage, Overflow, StructType, Type};

//macro_rules! assert_matches {
//    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => {
//        match $left {
//            $( $pattern )|+ $( if $guard )? => {}
//            ref left_val => {
//                panic!(r#"assertion failed: `(left matches right)`
//                left: `{:?}`,
//                right: `{:?}`"#,
//                            left_val, stringify!($($pattern)|+ $(if $guard)?));
//            }
//        }
//    }
//}

mod utils;
use self::utils::ParseTest;

//macro_rules! assert_module_error {
//    ($source:expr, $pattern:pat_param) => {
//        if let Err(err) = ParseTest::new().parse_module($source) {
//            assert_matches!(err, $pattern)
//        } else {
//            panic!("expected parsing to fail, but it succeeded");
//        }
//    };
//}

macro_rules! ident {
    ($name:ident) => {
        Ident::new(
            crate::Symbol::intern(stringify!($name)),
            miden_diagnostics::SourceSpan::UNKNOWN,
        )
    };
// 
//     ($name:literal) => {
//         Identifier::new(
//             miden_diagnostics::SourceSpan::UNKNOWN,
//             crate::Symbol::intern($name),
//         )
//     };
// 
//     ($module:ident, $name:ident) => {
//         QualifiedIdentifier::new(
//             ident!($module),
//             NamespacedIdentifier::Binding(ident!($name)),
//         )
//     };
}

macro_rules! function_ident {
    ($module:ident, $name:ident) => {
        FunctionIdent {
            module: ident!($module),
            function: ident!($name),
        }
    };
}

//
//#[allow(unused)]
//macro_rules! global {
//    ($name:ident, $offset:literal, $ty:expr) => {
//        ScalarExpr::SymbolAccess(SymbolAccess {
//            span: miden_diagnostics::SourceSpan::UNKNOWN,
//            name: ResolvableIdentifier::Global(ident!($name)),
//            access_type: AccessType::Default,
//            offset: $offset,
//            ty: Some($ty),
//        })
//    };
//
//    ($name:ident, $ty:expr) => {
//        ScalarExpr::SymbolAccess(SymbolAccess {
//            span: miden_diagnostics::SourceSpan::UNKNOWN,
//            name: ResolvableIdentifier::Global(ident!($name)),
//            access_type: AccessType::Default,
//            offset: 0,
//            ty: Some($ty),
//        })
//    };
//
//    ($name:literal, $ty:expr) => {
//        ScalarExpr::SymbolAccess(SymbolAccess {
//            span: miden_diagnostics::SourceSpan::UNKNOWN,
//            name: ResolvableIdentifier::Global(ident!($name)),
//            access_type: AccessType::Default,
//            offset: 0,
//            ty: Some($ty),
//        })
//    };
//}
//
//macro_rules! access {
//    ($name:ident) => {
//        ScalarExpr::SymbolAccess(SymbolAccess::new(
//            miden_diagnostics::SourceSpan::UNKNOWN,
//            ident!($name),
//            AccessType::Default,
//            0,
//        ))
//    };
//
//    ($name:literal) => {
//        ScalarExpr::SymbolAccess(SymbolAccess::new(
//            miden_diagnostics::SourceSpan::UNKNOWN,
//            ident!($name),
//            AccessType::Default,
//            0,
//        ))
//    };
//
//    ($module:ident, $name:ident, $ty:expr) => {
//        ScalarExpr::SymbolAccess(SymbolAccess {
//            span: miden_diagnostics::SourceSpan::UNKNOWN,
//            name: ResolvableIdentifier::Resolved(ident!($module, $name)),
//            access_type: AccessType::Default,
//            offset: 0,
//            ty: Some($ty),
//        })
//    };
//
//    ($name:ident, $offset:literal) => {
//        ScalarExpr::SymbolAccess(SymbolAccess::new(
//            miden_diagnostics::SourceSpan::UNKNOWN,
//            ident!($name),
//            AccessType::Default,
//            $offset,
//        ))
//    };
//
//    ($name:literal, $offset:literal) => {
//        ScalarExpr::SymbolAccess(SymbolAccess::new(
//            miden_diagnostics::SourceSpan::UNKNOWN,
//            ident!($name),
//            AccessType::Default,
//            $offset,
//        ))
//    };
//
//    ($name:ident, $offset:literal, $ty:expr) => {
//        ScalarExpr::SymbolAccess(SymbolAccess {
//            span: miden_diagnostics::SourceSpan::UNKNOWN,
//            name: ResolvableIdentifier::Local(ident!($name)),
//            access_type: AccessType::Default,
//            offset: $offset,
//            ty: Some($ty),
//        })
//    };
//
//    ($name:ident, $ty:expr) => {
//        ScalarExpr::SymbolAccess(SymbolAccess {
//            span: miden_diagnostics::SourceSpan::UNKNOWN,
//            name: ResolvableIdentifier::Local(ident!($name)),
//            access_type: AccessType::Default,
//            offset: 0,
//            ty: Some($ty),
//        })
//    };
//
//    ($name:literal, $ty:expr) => {
//        ScalarExpr::SymbolAccess(SymbolAccess {
//            span: miden_diagnostics::SourceSpan::UNKNOWN,
//            name: ResolvableIdentifier::Local(ident!($name)),
//            access_type: AccessType::Default,
//            offset: 0,
//            ty: Some($ty),
//        })
//    };
//
//    ($name:ident [ $idx:literal ]) => {
//        ScalarExpr::SymbolAccess(SymbolAccess::new(
//            miden_diagnostics::SourceSpan::UNKNOWN,
//            ident!($name),
//            AccessType::Index($idx),
//            0,
//        ))
//    };
//
//    ($name:literal [ $idx:literal ]) => {
//        ScalarExpr::SymbolAccess(SymbolAccess::new(
//            miden_diagnostics::SourceSpan::UNKNOWN,
//            ident!($name),
//            AccessType::Index($idx),
//            0,
//        ))
//    };
//
//    ($name:ident [ $row:literal ] [ $col:literal ]) => {
//        ScalarExpr::SymbolAccess(SymbolAccess::new(
//            miden_diagnostics::SourceSpan::UNKNOWN,
//            ident!($name),
//            AccessType::Matrix($row, $col),
//            0,
//        ))
//    };
//
//    ($name:ident [ $row:literal ] [ $col:literal ], $ty:expr) => {
//        ScalarExpr::SymbolAccess(SymbolAccess {
//            span: miden_diagnostics::SourceSpan::UNKNOWN,
//            name: ResolvableIdentifier::Local(ident!($name)),
//            access_type: AccessType::Matrix($row, $col),
//            offset: 0,
//            ty: Some($ty),
//        })
//    };
//
//    ($module:ident, $name:ident [ $idx:literal ], $ty:expr) => {
//        ScalarExpr::SymbolAccess(SymbolAccess {
//            span: miden_diagnostics::SourceSpan::UNKNOWN,
//            name: ident!($module, $name).into(),
//            access_type: AccessType::Index($idx),
//            offset: 0,
//            ty: Some($ty),
//        })
//    };
//
//    ($module:ident, $name:ident [ $row:literal ] [ $col:literal ], $ty:expr) => {
//        ScalarExpr::SymbolAccess(SymbolAccess {
//            span: miden_diagnostics::SourceSpan::UNKNOWN,
//            name: ident!($module, $name).into(),
//            access_type: AccessType::Matrix($row, $col),
//            offset: 0,
//            ty: Some($ty),
//        })
//    };
//
//    ($name:ident [ $idx:literal ], $offset:literal) => {
//        ScalarExpr::SymbolAccess(SymbolAccess::new(
//            miden_diagnostics::SourceSpan::UNKNOWN,
//            ident!($name),
//            AccessType::Index($idx),
//            $offset,
//        ))
//    };
//
//    ($name:ident [ $idx:literal ], $ty:expr) => {
//        ScalarExpr::SymbolAccess(SymbolAccess {
//            span: miden_diagnostics::SourceSpan::UNKNOWN,
//            name: ResolvableIdentifier::Local(ident!($name)),
//            access_type: AccessType::Index($idx),
//            offset: 0,
//            ty: Some($ty),
//        })
//    };
//
//    ($name:ident [ $idx:literal ], $offset:literal, $ty:expr) => {
//        ScalarExpr::SymbolAccess(SymbolAccess {
//            span: miden_diagnostics::SourceSpan::UNKNOWN,
//            name: ResolvableIdentifier::Local(ident!($name)),
//            access_type: AccessType::Index($idx),
//            offset: $offset,
//            ty: Some($ty),
//        })
//    };
//
//    ($name:literal [ $idx:literal ], $offset:literal) => {
//        ScalarExpr::SymbolAccess(SymbolAccess::new(
//            miden_diagnostics::SourceSpan::UNKNOWN,
//            ident!($name),
//            AccessType::Index($idx),
//            $offset,
//        ))
//    };
//}
//
//macro_rules! expr {
//    ($expr:expr) => {
//        Expr::try_from($expr).unwrap()
//    };
//}
//
//macro_rules! slice {
//    ($name:ident, $range:expr) => {
//        ScalarExpr::SymbolAccess(SymbolAccess {
//            span: miden_diagnostics::SourceSpan::UNKNOWN,
//            name: ResolvableIdentifier::Unresolved(NamespacedIdentifier::Binding(ident!($name))),
//            access_type: AccessType::Slice($range),
//            offset: 0,
//            ty: None,
//        })
//    };
//
//    ($name:ident, $range:expr, $ty:expr) => {
//        ScalarExpr::SymbolAccess(SymbolAccess {
//            span: miden_diagnostics::SourceSpan::UNKNOWN,
//            name: ResolvableIdentifier::Local(ident!($name)),
//            access_type: AccessType::Slice($range),
//            offset: 0,
//            ty: Some($ty),
//        })
//    };
//}
//
//macro_rules! bounded_access {
//    ($name:ident, $bound:expr) => {
//        ScalarExpr::BoundedSymbolAccess(BoundedSymbolAccess::new(
//            miden_diagnostics::SourceSpan::UNKNOWN,
//            SymbolAccess::new(
//                miden_diagnostics::SourceSpan::UNKNOWN,
//                ident!($name),
//                AccessType::Default,
//                0,
//            ),
//            $bound,
//        ))
//    };
//
//    ($name:ident, $bound:expr, $ty:expr) => {
//        ScalarExpr::BoundedSymbolAccess(BoundedSymbolAccess::new(
//            miden_diagnostics::SourceSpan::UNKNOWN,
//            SymbolAccess {
//                span: miden_diagnostics::SourceSpan::UNKNOWN,
//                name: ResolvableIdentifier::Local(ident!($name)),
//                access_type: AccessType::Default,
//                offset: 0,
//                ty: Some($ty),
//            },
//            $bound,
//        ))
//    };
//
//    ($name:ident [ $idx:literal ], $bound:expr) => {
//        ScalarExpr::BoundedSymbolAccess(BoundedSymbolAccess::new(
//            miden_diagnostics::SourceSpan::UNKNOWN,
//            SymbolAccess::new(
//                miden_diagnostics::SourceSpan::UNKNOWN,
//                ident!($name),
//                AccessType::Index($idx),
//                0,
//            ),
//            $bound,
//        ))
//    };
//
//    ($name:ident [ $idx:literal ], $bound:expr, $ty:expr) => {
//        ScalarExpr::BoundedSymbolAccess(BoundedSymbolAccess::new(
//            miden_diagnostics::SourceSpan::UNKNOWN,
//            SymbolAccess {
//                span: miden_diagnostics::SourceSpan::UNKNOWN,
//                name: ResolvableIdentifier::Local(ident!($name)),
//                access_type: AccessType::Index($idx),
//                offset: 0,
//                ty: Some($ty),
//            },
//            $bound,
//        ))
//    };
//}
//
//macro_rules! int {
//    ($value:literal) => {
//        ScalarExpr::Const(miden_diagnostics::Span::new(
//            miden_diagnostics::SourceSpan::UNKNOWN,
//            $value,
//        ))
//    };
//
//    ($value:expr) => {
//        ScalarExpr::Const(miden_diagnostics::Span::new(
//            miden_diagnostics::SourceSpan::UNKNOWN,
//            $value,
//        ))
//    };
//}
//
//macro_rules! call {
//    ($callee:ident ($($param:expr),+)) => {
//        ScalarExpr::Call(Call::new(miden_diagnostics::SourceSpan::UNKNOWN, ident!($callee), vec![$($param),+]))
//    };
//
//    ($module:ident :: $callee:ident ($($param:expr),+)) => {
//        ScalarExpr::Call(Call {
//            span: miden_diagnostics::SourceSpan::UNKNOWN,
//            callee: ResolvableIdentifier::Resolved(function_ident!($module, $callee)),
//            args: vec![$($param),+],
//            ty: None,
//        })
//    }
//}
//
//macro_rules! trace_segment {
//    ($idx:literal, $name:literal, [$(($binding_name:ident, $binding_size:literal)),*]) => {
//        TraceSegment::new(miden_diagnostics::SourceSpan::UNKNOWN, $idx, ident!($name), vec![
//            $(miden_diagnostics::Span::new(miden_diagnostics::SourceSpan::UNKNOWN, (ident!($binding_name), $binding_size))),*
//        ])
//    }
//}
//
//macro_rules! random_values {
//    ($name:literal, $size:literal) => {
//        RandomValues::with_size(miden_diagnostics::SourceSpan::UNKNOWN, ident!($name), $size)
//    };
//
//    ($name:literal, [$(($binding_name:ident, $binding_size:literal)),*]) => {
//        RandomValues::new(miden_diagnostics::SourceSpan::UNKNOWN, ident!($name), vec![
//            $(miden_diagnostics::Span::new(miden_diagnostics::SourceSpan::UNKNOWN, (ident!($binding_name), $binding_size))),*
//        ])
//    }
//}
//
//macro_rules! constant {
//    ($name:ident = $value:literal) => {
//        Constant::new(
//            SourceSpan::UNKNOWN,
//            ident!($name),
//            ConstantExpr::Scalar($value),
//        )
//    };
//
//    ($name:ident = [$($value:literal),+]) => {
//        Constant::new(SourceSpan::UNKNOWN, ident!($name), ConstantExpr::Vector(vec![$($value),+]))
//    };
//
//    ($name:ident = [$([$($value:literal),+]),+]) => {
//        Constant::new(SourceSpan::UNKNOWN, ident!($name), ConstantExpr::Matrix(vec![$(vec![$($value),+]),+]))
//    };
//}
//
//macro_rules! vector {
//    ($($value:literal),*) => {
//        Expr::Const(miden_diagnostics::Span::new(miden_diagnostics::SourceSpan::UNKNOWN, ConstantExpr::Vector(vec![$($value),*])))
//    };
//
//    ($($value:expr),*) => {
//        Expr::Vector(miden_diagnostics::Span::new(miden_diagnostics::SourceSpan::UNKNOWN, vec![$(expr!($value)),*]))
//    }
//}
//
//macro_rules! matrix {
//    ($([$($value:expr),+]),+) => {
//        Expr::Matrix(miden_diagnostics::Span::new(miden_diagnostics::SourceSpan::UNKNOWN, vec![$(vec![$($value),+]),+]))
//    };
//}
//
//macro_rules! let_ {
//    ($name:ident = $value:expr => $($body:expr),+) => {
//        Statement::Let(Let::new(miden_diagnostics::SourceSpan::UNKNOWN, ident!($name), $value, vec![$($body),+]))
//    };
//
//    ($name:literal = $value:expr => $($body:expr),+) => {
//        Statement::Let(Let::new(miden_diagnostics::SourceSpan::UNKNOWN, ident!($name), $value, vec![$($body),+]))
//    };
//}
//
//macro_rules! enforce {
//    ($expr:expr) => {
//        Statement::Enforce($expr)
//    };
//
//    ($expr:expr, when $selector:expr) => {
//        Statement::EnforceIf($expr, $selector)
//    };
//}
//
//macro_rules! enforce_all {
//    ($expr:expr) => {
//        Statement::EnforceAll($expr)
//    };
//}
//
//macro_rules! lc {
//    (($(($binding:ident, $iterable:expr)),+) => $body:expr) => {{
//        let context = vec![
//            $(
//                (ident!($binding), $iterable)
//            ),+
//        ];
//        ListComprehension::new(miden_diagnostics::SourceSpan::UNKNOWN, $body, context, None)
//    }};
//
//    (($(($binding:literal, $iterable:expr)),+) => $body:expr) => {{
//        let context = vec![
//            $(
//                (ident!($binding), $iterable)
//            ),+
//        ];
//        ListComprehension::new(miden_diagnostics::SourceSpan::UNKNOWN, $body, context, None)
//    }};
//
//    (($(($binding:ident, $iterable:expr)),*) => $body:expr, when $selector:expr) => {{
//        let context = vec![
//            $(
//                (ident!($binding), $iterable)
//            ),+
//        ];
//        ListComprehension::new(miden_diagnostics::SourceSpan::UNKNOWN, $body, context, Some($selector))
//    }};
//
//    (($(($binding:literal, $iterable:expr)),*) => $body:expr, when $selector:expr) => {{
//        let context = vec![
//            $(
//                (ident!($binding), $iterable)
//            ),+
//        ];
//        ListComprehension::new(miden_diagnostics::SourceSpan::UNKNOWN, $body, context, Some($selector))
//    }};
//}
//
//macro_rules! range {
//    ($range:expr) => {
//        Expr::Range(miden_diagnostics::Span::new(SourceSpan::UNKNOWN, $range))
//    };
//}
//
//macro_rules! and {
//    ($lhs:expr, $rhs:expr) => {
//        mul!($lhs, $rhs)
//    };
//}
//
//macro_rules! or {
//    ($lhs:expr, $rhs:expr) => {{
//        sub!(add!($lhs, $rhs), mul!($lhs, $rhs))
//    }};
//}
//
//macro_rules! not {
//    ($rhs:expr) => {
//        sub!(int!(1), $rhs)
//    };
//}
//
//macro_rules! eq {
//    ($lhs:expr, $rhs:expr) => {
//        ScalarExpr::Binary(BinaryExpr::new(
//            miden_diagnostics::SourceSpan::UNKNOWN,
//            BinaryOp::Eq,
//            $lhs,
//            $rhs,
//        ))
//    };
//}
//
//macro_rules! add {
//    ($lhs:expr, $rhs:expr) => {
//        ScalarExpr::Binary(BinaryExpr::new(
//            miden_diagnostics::SourceSpan::UNKNOWN,
//            BinaryOp::Add,
//            $lhs,
//            $rhs,
//        ))
//    };
//}
//
//macro_rules! sub {
//    ($lhs:expr, $rhs:expr) => {
//        ScalarExpr::Binary(BinaryExpr::new(
//            miden_diagnostics::SourceSpan::UNKNOWN,
//            BinaryOp::Sub,
//            $lhs,
//            $rhs,
//        ))
//    };
//}
//
//macro_rules! mul {
//    ($lhs:expr, $rhs:expr) => {
//        ScalarExpr::Binary(BinaryExpr::new(
//            miden_diagnostics::SourceSpan::UNKNOWN,
//            BinaryOp::Mul,
//            $lhs,
//            $rhs,
//        ))
//    };
//}
//
//macro_rules! exp {
//    ($lhs:expr, $rhs:expr) => {
//        ScalarExpr::Binary(BinaryExpr::new(
//            miden_diagnostics::SourceSpan::UNKNOWN,
//            BinaryOp::Exp,
//            $lhs,
//            $rhs,
//        ))
//    };
//}
//
//macro_rules! import_all {
//    ($module:ident) => {
//        Import::All {
//            module: ident!($module),
//        }
//    };
//}
//
//macro_rules! import {
//    ($module:ident, $item:ident) => {{
//        let mut items: std::collections::HashSet<Identifier> = std::collections::HashSet::default();
//        items.insert(ident!($item));
//        Import::Partial {
//            module: ident!($module),
//            items,
//        }
//    }};
//}

// FULL MIDEN IR FILE
// ================================================================================================

#[test]
fn full_mir_file() {
    let dummy_sourcespan = miden_diagnostics::SourceSpan::UNKNOWN;

    // module miden_ir_test
    let module_type = ModuleType::Module;
    let module_name = ident!(miden_ir_test);

    // global_1 u32 internal = 0xCAFEBABE
    let global_var_name = ident!(global_1);
    let global_var_type = Type::U32;
    let global_var_linkage = Linkage::Internal;
    let mut global_var = GlobalVarDeclaration::new(dummy_sourcespan,
                                                   global_var_name,
                                                   global_var_type,
                                                   global_var_linkage);
    let global_var_init_val : Vec<u8> = vec![b'C', b'A', b'F', b'E', b'B', b'A', b'B', b'E'];
    let global_var_initializer = GlobalVarInitializer::new(global_var_init_val);
    global_var.with_init(global_var_initializer);
    let global_vars = vec![global_var];

    // pub coc(fast) fn miden_ir_test::test_func (zext u32, sext u32) -> u32 {
    let function_visibility = Visibility::Public;
    let function_call_convention = CallConv::Fast;
    let function_name = function_ident!(miden_ir_test, test_func);

    let fun_arg1_purpose = ArgumentPurpose::Default;
    let fun_arg1_extension = ArgumentExtension::Zext;
    let fun_arg1_type = Type::U32;
    let fun_arg1 = FunctionParameter::new(fun_arg1_purpose,
                                          fun_arg1_extension,
                                          fun_arg1_type);
    
    let fun_arg2_purpose = ArgumentPurpose::Default;
    let fun_arg2_extension = ArgumentExtension::Sext;
    let fun_arg2_type = Type::U32;
    let fun_arg2 = FunctionParameter::new(fun_arg2_purpose,
                                          fun_arg2_extension,
                                          fun_arg2_type);
    let function_params = vec![fun_arg1, fun_arg2];


    let fun_return_extension = ArgumentExtension::None;
    let fun_return_type = Type::U32;
    let function_return = FunctionReturn::new(fun_return_extension, fun_return_type);
    let function_returns = vec![function_return];
    
    let function_signature = FunctionSignature::new(dummy_sourcespan,
                                                    function_visibility,
                                                    function_call_convention,
                                                    function_name,
                                                    function_params,
                                                    function_returns);

    //     blk(v1 : u32, v2 : u32) : {
    let block_label = Label::new(ident!(blk));

    let block_arg1_name = Value::new(ident!(v1));
    let block_arg1_type = Type::U32;
    let block_arg1 = BlockArgument::new(block_arg1_name, block_arg1_type);
    
    let block_arg2_name = Value::new(ident!(v2));
    let block_arg2_type = Type::U32;
    let block_arg2 = BlockArgument::new(block_arg2_name, block_arg2_type);
    
    let block_args = vec![block_arg1, block_arg2];
    
    let block_header = BlockHeader::new(block_label, block_args);

    //         v3 = add.unchecked v1 v2 : u32
    let inst1_value = Value::new(ident!(v3));
    let inst1_values = vec![inst1_value];

    let inst1_overflow = Overflow::Unchecked;
    let inst1_opcode = BinaryOpCode::Add(inst1_overflow);
    let inst1_operand1 = Value::new(ident!(v1));
    let inst1_operand2 = Value::new(ident!(v2));
    let inst1_op = Operation::BinaryOp(inst1_opcode,
                                       inst1_operand1,
                                       inst1_operand2);
    let inst1_type = Type::U32;
    let inst1_types : Vec<Type> = vec![inst1_type];
    let block_instruction1 = Instruction::new(dummy_sourcespan,
                                              inst1_values,
                                              inst1_op,
                                              inst1_types);
    
    //         ret (v1, v3)
    let inst2_values : Vec<Value> = vec![];
    let inst2_return_val1 = Value::new(ident!(v1));
    let inst2_return_val2 = Value::new(ident!(v3));
    let inst2_return_vals = vec![inst2_return_val1, inst2_return_val2];
    let inst2_op = Operation::ReturnOp(inst2_return_vals);
    let inst2_types : Vec<Type> = vec![];
    let block_instruction2 = Instruction::new(dummy_sourcespan,
                                              inst2_values,
                                              inst2_op,
                                              inst2_types);
    
    let block_instructions = vec![block_instruction1, block_instruction2];
       
    let block = Block::new(dummy_sourcespan, block_header, block_instructions);
    let blocks = vec![block];
    
    let function = FunctionDeclaration::new(dummy_sourcespan,
                                            function_signature,
                                            blocks);
    let functions = vec![function];
    
    // cc(kernel) fn exported::f1 (sret { u32, u32 }) -> [i8 ; 42];
    let external_visibility = Visibility::Private;
    let external_call_convention = CallConv::Kernel;
    let external_name = function_ident!(exported, f1);

    let ext_arg1_purpose = ArgumentPurpose::StructReturn;
    let ext_arg1_extension = ArgumentExtension::None;

    let ext_arg1_type_field1 = Type::U32;
    let ext_arg1_type_field2 = Type::U32;
    let ext_arg1_type_fields = vec![ext_arg1_type_field1, ext_arg1_type_field2];
    let ext_arg1_type = Type::Struct(StructType::new(ext_arg1_type_fields));
    let ext_arg1 = FunctionParameter::new(ext_arg1_purpose,
                                          ext_arg1_extension,
                                          ext_arg1_type);
    
    let external_params = vec![ext_arg1];

    let ext_return_extension = ArgumentExtension::None;
    let ext_return_inner_type = Type::I8;
    let ext_return_type = Type::Array(Box::new(ext_return_inner_type), 42);
    let external_return = FunctionReturn::new(ext_return_extension, ext_return_type);
    let external_returns = vec![external_return];

    let external = FunctionSignature::new(dummy_sourcespan,
                                          external_visibility,
                                          external_call_convention,
                                          external_name,
                                          external_params,
                                          external_returns);
    let externals = vec![external];
    
    let expected = Module::new(dummy_sourcespan,
                               module_type,
                               module_name,
                               global_vars,
                               functions,
                               externals);

    ParseTest::new().expect_module_ast_from_file("src/parser/parser/tests/input/system.mir", expected);
}
