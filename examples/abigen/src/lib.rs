#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str;
use std::io::BufWriter;

#[derive(Serialize, Deserialize)]
struct Abi {
    CompilerVersion: String,
    hash:String,
    entrypoint:String,
    functions:Vec<Function>,
}
#[derive(Serialize, Deserialize)]
struct Function {
    name:String,
    parameters:Vec<Parameters>,
}

#[derive(Serialize, Deserialize)]
struct Parameters {
    name:String,
    p_type:String,
}

unsafe  fn parse_json_to_go(file_path: String) {
    let struct_name = generate_go_struct_name(file_path.clone());
    let abi = read_file(&file_path);
    generate_go_file(struct_name, abi);
}

unsafe fn generate_go_file(go_struct_name:String, abi: Abi) {
    let file_new = File::create(format!("{}{}",go_struct_name, ".go".to_string())).unwrap();
    let mut f_out = BufWriter::new(file_new);
    f_out.write_all("import (
	\"bytes\"
	\"encoding/hex\"
	\"fmt\"
	sdkcom \"github.com/ontio/ontology-go-sdk/common\"
	\"github.com/ontio/ontology/common\"
	\"github.com/ontio/ontology/core/types\"
	\"github.com/ontio/ontology/smartcontract/states\"
)".as_bytes());
    f_out.write("\n".as_bytes());
    f_out.write_all(format!("type {} struct ", go_struct_name).as_bytes());
    f_out.write("{".as_bytes());
    f_out.write("\n".as_bytes());
    f_out.write("    contractAddr common.Address
	vm WasmVMContract
	gasPrice uint64
	gasLimit uint64
	signer *Account
	version byte".as_bytes());
    f_out.write("\n".as_bytes());
    f_out.write("}".as_bytes());
    f_out.write("\n".as_bytes());
    let deploy = format!("func(this *{}) Deploy(gasPrice, gasLimit uint64,", go_struct_name);
    f_out.write(deploy.as_bytes());
    f_out.write("singer *Account,
	needStorage byte,
	code,
	name,
	version,
	author,
	email,
	desc string) (*types.MutableTransaction, error)".as_bytes());
    f_out.write("{".as_bytes());
    f_out.write("\n".as_bytes());
    f_out.write("    invokeCode, err := hex.DecodeString(code)
	if err != nil {
		return nil, fmt.Errorf(\"code hex decode error:%s\", err)
	}
	tx := this.vm.NewDeployWasmVMCodeTransaction(gasPrice, gasLimit, &sdkcom.SmartContract{
		Code:        invokeCode,
		NeedStorage: needStorage,
		Name:        name,
		Version:     version,
		Author:      author,
		Email:       email,
		Description: desc,
	})
	return tx, nil".as_bytes());
    f_out.write("\n".as_bytes());
    f_out.write("}".as_bytes());
    f_out.write("\n".as_bytes());
    for func in abi.functions {
        let func_name = first_char_to_upper(func.name.clone());
        let fun_str = format!("func (this *{}) {}(", go_struct_name, func_name);
        f_out.write(fun_str.as_bytes());
        let params = build_params(func.parameters);
        f_out.write(params.0.as_bytes());
        f_out.write(") (*types.MutableTransaction, error) {".as_bytes());
        f_out.write("\n".as_bytes());
        f_out.write(format!("    bs,err := this.buildParams(\"{}\",[]interface", func.name).as_bytes());
        f_out.write("{}{".as_bytes());
        f_out.write(format!("{}", params.1).as_bytes());
        f_out.write("})".as_bytes());
        f_out.write("\n".as_bytes());
        f_out.write("    if err != nil {
		return nil, fmt.Errorf(\"buildparams failed:s%\", err)
	}
	tx := this.vm.ontSdk.NewInvokeWasmTransaction(0, 0, bs)
	if err != nil {
		return nil, err
	}
	return tx, nil".as_bytes());
        f_out.write("\n".as_bytes());
        f_out.write("}".as_bytes());
        f_out.write("\n".as_bytes());
    }
    f_out.flush();
}
fn build_params(params:Vec<Parameters>) -> (String, String) {
    let mut res = String::new();
    let mut param_names = String::new();
    for param in params {
        if param_names == "" {
            param_names = format!("{}", param.name);
        } else {
            param_names = format!("{}, {}", param_names, param.name);
        }
        match param.p_type.as_str() {
            "" => {}
            "Address" => {
                if res == "" {
                    res = format!("{} common.Address", param.name);
                } else {
                    res = format!("{}, {} common.Address", res, param.name);
                }
            }
            "String" => {
                if res == "" {
                    res = format!("{} string", param.name);
                } else {
                    res = format!("{}, {} string", res, param.name);
                }
            }
            "U256" => {
                if res == "" {
                    res = format!("{} U256", param.name);
                } else {
                    res = format!("{}, {} U256", res, param.name);
                }
            }
            &_ => {
                panic!("not supported type");
            }
        }
    }
    (res, param_names)
}
unsafe fn generate_go_struct_name(file_path: String) -> String {
    let mut v:Vec<&str> = file_path.split(|c|c=='/' || c=='.').collect();
    if v.len() < 2 {
        panic!("file path is wrong:{}", file_path);
    }
    let mut file_name = v[v.len()-2];
    let res = first_char_to_upper(file_name.to_string());
    format!("{}{}", res, "Contract")
}
unsafe fn first_char_to_upper(temp:String) ->String {
    let mut t_upper = temp.to_uppercase();
    let mut t_string = temp.to_string();
    let t_bs = t_string.as_bytes_mut();
    t_bs[0] = t_upper.as_bytes()[0];
    let res = match str::from_utf8(t_bs) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    res.to_string()
}
fn read_file(file_path: &str) -> Abi {
    let path = Path::new(file_path);
    let display = path.display();
    let mut file = match  File::open(path){
        Err(why) => panic!("couldn't open {}: {}", ),
        Ok(file) => file,
    };
    let mut f = File::open(file_path).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("something went wrong reading the file");
    let abi:Abi = serde_json::from_str(&buf).unwrap();
    abi
}

#[test]
fn it_works() {
    unsafe {
        parse_json_to_go("./oep4_abi.json".to_string());
    }
    assert_eq!(2 + 2, 4);
}
