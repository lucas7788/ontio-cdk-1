
use parity_wasm::elements::{Instruction, ValueType,InitExpr, ImportEntry,External,FunctionType};
use parity_wasm::elements::opcodes::*;
use std::vec::{Vec};
use std::collections::btree_map::BTreeMap;

const VALID_FIELD :[&str;19]= ["timestamp", "block_height","self_address","caller_address","entry_address","check_witness",
"ret","notify","input_length","get_input","call_contract","call_output_length","get_output","current_blockhash",
"current_txhash","contract_migrate","storage_read","storage_write","storage_delete"];

pub fn get_import_func_index (import_entry:&ImportEntry) -> Option<BTreeMap<&u32, &str>> {
    if import_entry.module() == "env" {
        if !VALID_FIELD.contains(&import_entry.field()) {
            panic!("import section use invalid field: {}", import_entry.field());
        }
        let mut func_index:BTreeMap<&u32, &str> = BTreeMap::new();
        match import_entry.external() {
            External::Function(index) => {
                func_index.insert(index, import_entry.field());
            }
            _ => {}
        }
        return Some(func_index);
    }
    return None;
}
pub fn is_invalid_type(func_name: &str,func_type: &FunctionType) {
    match func_name {
        "timestamp" => {
            let params = func_type.params();
            if params.len() != 0 {
                panic!("function name:{}, length of parameter expected 0, got {}", func_name, params.len());
            }
            if let Some(ret_type) = func_type.return_type() {
                if ret_type != ValueType::I64 {
                    panic!("function name: {}, return value type expected i64, got {}", func_name, ret_type);
                }
            }
        }
        "block_height"|"input_length"|"call_output_length" => {
            let params = func_type.params();
            if params.len() != 0 {
                panic!("function name:{}, length of parameter expected 0, got {}", func_name, params.len());
            }
            if let Some(ret_type) = func_type.return_type() {
                if ret_type != ValueType::I32 {
                    panic!("function name:{}, return value type expected i32, got:{}", func_name, ret_type);
                }
            }
        }
        "self_address"|"caller_address"|"entry_address"|"get_input"|"get_output" => {
            let params = func_type.params();
            if params.len() != 1 {
                panic!("function name: {}, length of parameter expected 1, got {}", func_name, params.len());
            }
            if let Some(ret_type) = func_type.return_type() {
                panic!("function name:{}, return value type expected None, got {}", func_name, ret_type);
            }
        }
        "check_witness"|"current_blockhash"|"current_txhash" => {
            let params = func_type.params();
            if params.len() != 1 {
                panic!("function name:{}, length of parameter expected 1, got {}", func_name, params.len());
            }
            if let Some(ret_type) = func_type.return_type() {
                if ret_type != ValueType::I32 {
                    panic!("function name:{}, return value type expected i32, got {}", func_name, ret_type);
                }
            }
        }
        "ret" => {
            let params = func_type.params();
            if params.len() != 2 {
                panic!("function name:{}, length of parameter expected 2, got {}", func_name, params.len());
            }
            if let Some(ret_type) = func_type.return_type() {
                panic!("function name:{}, return value type expected None, got {}", func_name, ret_type);
            }
        }
        "call_contract" => {
            let params = func_type.params();
            if params.len() != 3 {
                panic!("function name:{}, length of parameter expected 3, got {}", func_name, params.len());
            }
            if let Some(ret_type) = func_type.return_type() {
                if ret_type != ValueType::I32 {
                    panic!("function name: {}, return value expected i32, got {}", func_name, ret_type);
                }
            }
        }
        "contract_migrate" => {
            let params = func_type.params();
            if params.len() != 14 {
                panic!("function name:{}, length of parameter expected 4, got {}", func_name, params.len());
            }
            if let Some(ret_type) = func_type.return_type() {
                if ret_type != ValueType::I32 {
                    panic!("function name:{}, return value type expected i32, got {}", func_name, ret_type);
                }
            }
        }
        "storage_read" => {
            let params = func_type.params();
            if params.len() != 5 {
                panic!("function name:{}, length of parameter expected 4, got {}", func_name, params.len());
            }
            if let Some(ret_type) = func_type.return_type() {
                if ret_type != ValueType::I32 {
                    panic!("function name:{}, return value type expected i32, got {}", func_name, ret_type);
                }
            }
        }
        "storage_write" => {
            let params = func_type.params();
            if params.len() != 4 {
                panic!("function name:{}, length of parameter expected 4, got {}", func_name, params.len());
            }
            if let Some(ret_type) = func_type.return_type() {
                panic!("function name:{}, return value type expected None, got {}", func_name, ret_type);
            }
        }
        "storage_delete" => {
            let params = func_type.params();
            if params.len() != 2 {
                panic!("function timestamp has wrong parameter number");
            }
            if let Some(ret_type) = func_type.return_type() {
                panic!("function name:{}, return value type expected None, got {}", func_name, ret_type);
            }
        }
        _ => {}
    }
}
pub fn is_invalid_instruction(instruction: &Instruction) -> bool {
    let instruction_str = format!("{}", instruction);
    if instruction_str.contains("f32") || instruction_str.contains("f64") {
        return true;
    }
    false
}

pub fn is_invalid_value_type(value_type: &ValueType) -> bool {
    match value_type {
        ValueType::F32|ValueType::F64 => {
            println!("invalid value type: {}", value_type);
            return true
        }
        _ => {
            return false
        }
    }
}
pub fn is_invalid_init_expr(init_expr: &InitExpr) -> bool {
    for expr in init_expr.code(){
        if is_invalid_instruction(&expr) {
            println!("invalid expr: {}", expr);
            return true;
        }
    }
    false
}