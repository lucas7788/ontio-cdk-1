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
                    println!("invalid value type: {}", local.value_type());
                    return;
                }
            }
            let instructions = body.code();
            for instruction in instructions.elements() {
                if common::is_invalid_instruction(&instruction) {
                    println!("invalid instruction: {}", instruction);
                    return;
                }
            }
        }
    }
    if let Some(global_section) = module.global_section() {
        let entries = global_section.entries();
        for entry in entries {
            if common::is_invalid_value_type(&entry.global_type().content_type()) {
                println!("global type content type is invalid: {}", &entry.global_type().content_type());
                return;
            }
            if common::is_invalid_init_expr(entry.init_expr()) {
                return;
            }
        }
    }
    if let Some(import_section) = module.import_section() {
        let import_sections = import_section.entries();
        for import_entry in import_sections {
            match import_entry.external() {
                External::Global(external_gloal_type) => {
                    if common::is_invalid_value_type(&external_gloal_type.content_type()) {
                        println!("import section use invalid value type: {}", external_gloal_type.content_type());
                        return;
                    }
                }
                _ => {}
            };
            let func_index_temp = common::get_import_func_index(import_entry);
            match func_index_temp {
                Ok(func_index) =>{

                    if let Some(type_section) = module.type_section() {
                        let types = type_section.types();
                        let mut index = 0u32;
                        for ty in types {
                            match ty {
                                Type::Function(t) => {
                                    let param = t.params();
                                    for value_type in param {
                                        if common::is_invalid_value_type(value_type) {
                                            println!("invalid function parameter type: {}", value_type);
                                            return;
                                        }
                                    }
                                    if let Some(ret_type) = t.return_type() {
                                        if common::is_invalid_value_type(&ret_type) {
                                            println!("invalid function return type: {}", ret_type);
                                            return;
                                        }
                                    }
                                    if let Some(func_name) = func_index.get(&index) {
                                        let res = common::check_type(func_name, t);
                                        match res {
                                            Ok(_) => {}
                                            Err(e) => {
                                                println!("check_type failed: {}", e);
                                                return;
                                            }
                                        }
                                    }
                                    index += 1;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("get_import_func_index failed: {}", e);
                }
            }
        }
    }
    if let Some(data_section) = module.data_section() {
        let data_segments = data_section.entries();
        for data_segment in data_segments {
            if let Some(init_expr) = data_segment.offset() {
                if common::is_invalid_init_expr(init_expr) {
                    return;
                }
            }
        }
    }
    if let Some(elements_section) = module.elements_section() {
        let entries = elements_section.entries();
        for entry in entries {
            if let Some(init_expr) = entry.offset() {
                if common::is_invalid_init_expr(init_expr) {
                    println!("elements use invalid init_expr: init_expr");
                    return;
                }
            }
        }
    }
}

#[test]
fn check_test() {
    check("./token.wasm");
}