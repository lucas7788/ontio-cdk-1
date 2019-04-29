extern crate parity_wasm;

use parity_wasm::elements::{FuncBody, Local, ValueType,GlobalEntry,Type, FunctionType,Instruction,ImportEntry,External,InitExpr};
use std::vec::Vec;
use std::fs::{File};
use std::io::{Error,BufWriter};
use std::io::prelude::*;
use super::common;
use std::collections::BTreeMap;

pub fn check(p: &str) {
    let modul = parity_wasm::deserialize_file(p);
    if modul.is_err() {
        println!("parity_wasm::deserialize_file error");
        return;
    }
    let module = modul.unwrap();
    if let Some(code_section) = module.code_section() {
        let bodies:&[FuncBody] = code_section.bodies();
        for body in bodies {
            let locals = body.locals();
            for local in locals {
                if common::is_invalid_value_type(&local.value_type()) {
                    panic!("function body local value type is invalid");
                }
            }
            let instructions = body.code();
            for instruction in instructions.elements() {
                if common::is_invalid_instruction(&instruction) {
                    panic!("invalid instruction");
                }
            }
        }
    }
    if let Some(global_section) = module.global_section() {
        let entries = global_section.entries();
        for entry in entries {
            if common::is_invalid_value_type(&entry.global_type().content_type()) {
                panic!("global entry value type is invalid");
            }
            if common::is_invalid_init_expr(entry.init_expr()) {
                panic!("invalid global init expression instruction");
            }
        }
    }
    if let Some(import_section) = module.import_section() {
        let import_sections = import_section.entries();
        for import_entry in import_sections {
            println!("import_entry:{:?}", import_entry);
            match import_entry.external() {
                External::Global(external_gloal_type) => {
                    if common::is_invalid_value_type(&external_gloal_type.content_type()) {
                        panic!("import section use invalid value type");
                    }
                }
                _ => {}
            };
            if let Some(func_index) = common::get_import_func_index(import_entry) {
                if let Some(type_section) = module.type_section() {
                    let types = type_section.types();
                    let mut index = 0u32;
                    for ty in types {
                        match ty {
                            Type::Function(t) => {
                                let param = t.params();
                                for value_type in param {
                                    if common::is_invalid_value_type(value_type) {
                                        panic!("[function parameter type is wrong], not supported data type");
                                    }
                                }
                                if let Some(ret_type) = t.return_type() {
                                    if common::is_invalid_value_type(&ret_type) {
                                        panic!("function return type is wrong")
                                    }
                                }
                                if let Some(func_name) = func_index.get(&index) {
                                    common::is_invalid_type(func_name, t);
                                }
                                index += 1;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    if let Some(type_section) = module.type_section() {
        let types = type_section.types();
        for ty in types {
            match ty {
                Type::Function(t) => {
                    let param = t.params();
                    for value_type in param {
                        if common::is_invalid_value_type(value_type) {
                            panic!("[function parameter type is wrong], not supported data type");
                        }
                    }
                    if let Some(ret_type) = t.return_type() {
                        if common::is_invalid_value_type(&ret_type) {
                            panic!("function return type is wrong")
                        }
                    }
                }
                _ => {}
            }
        }
    }
    if let Some(data_section) = module.data_section() {
        let data_segments = data_section.entries();
        for data_segment in data_segments {
            if let Some(init_expr) = data_segment.offset() {
                if common::is_invalid_init_expr(init_expr) {
                    panic!("data section use invalid init expr");
                }
            }
        }
    }
    if let Some(elements_section) = module.elements_section() {
        let entries = elements_section.entries();
        for entry in entries {
            if let Some(init_expr) = entry.offset() {
                if common::is_invalid_init_expr(init_expr) {
                    panic!("elements use invalid init_expr");
                }
            }
        }
    }
}

#[test]
fn check_test() {
    check("./token.wasm");
}