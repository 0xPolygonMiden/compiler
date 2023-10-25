use crate::parser::ast::*;
use crate::{ArgumentExtension, ArgumentPurpose, CallConv, FunctionIdent, Ident, Linkage, Overflow, StructType, Type};

mod utils;
use self::utils::ParseTest;

macro_rules! ident {
    ($name:ident) => {
        Ident::new(
            crate::Symbol::intern(stringify!($name)),
            miden_diagnostics::SourceSpan::UNKNOWN,
        )
    };
}

macro_rules! function_ident {
    ($module:ident, $name:ident) => {
        FunctionIdent {
            module: ident!($module),
            function: ident!($name),
        }
    };
}

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
