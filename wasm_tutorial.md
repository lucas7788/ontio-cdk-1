# Ontology WASM合约教程

## 开始
Ontology WASM合约支持RUST和C++语言开发，本文将会详细介绍如何使用RUST语言开发Ontology WASM合约。为了提高开发的效率，建议在开发合约之前，先准备好如下的环境。
* Rust开发环境（必须）
* IDE(推荐)
Clion是一款功能强大的C/C++/Rust开发工具，支持单步调式，方便WASM合约本地调试。
* 本地WASM合约测试节点（推荐）
搭建本地测试节点，方便合约测试，可以通过在合约中添加`debug`信息，在节点日志中监控合约运行信息。当然，如果觉得自己搭建测试节点较复杂，我们也可以使用测试网来测试合约。

## 环境搭建

### Rust开发环境搭建
1. 安装rustup, 非windows系统可以直接执行下面的命令。
```
curl https://sh.rustup.rs -sSf | sh
```
如果是windows系统，请访问[官网](https://www.rust-lang.org/zh-CN/tools/install)下载合适的版本进行安装。

2. 安装rust编译器
```
rustup install nightly
```

设置默认的编译版本nightly
```
rustup default nightly
```
3. 安装wasm32编译目前
```
rustup target add wasm32-unknown-unknown
```
4. 安装集成开发环境
IDE和编辑工具推荐Clion,IntelliJ,vim等

### 本地测试节点搭建（推荐）
该部分请参考
https://github.com/ontio/ontology#local-privatenet

>注意：编译好的可执行文件在启动的时候，请配置日志级别为debug模式，该模式下方便查看合约运行的debug信息。

## ontology-wasm-cdt-rust介绍
`ontio-cdk`是用于使用rust开发面向ontology的WebAssembly智能合约工具套件, 包含合约编写的标准库，链上交互的运行时api，合约接口abi生成插件，合约测试框架等。
### ontio-std介绍

* 和链交互的运行时API接口
* 合约级别的存储管理
* 合约测试框架
* abi和client端代码生成

#### 生成接口文档
1. 将项目clone大本地
```
git clone https://github.com/ontio/ontology-wasm-cdt-rust.git
```
2. 进入项目目录执行
```
cargo doc
```
生成cdt的接口文档，可以通过浏览器查看。

#### abi模块介绍
abi模块封装了合约中常用数据类型的序列化方法，方便开发者直接使用。
1. `Sink`: 用于合约中数据类型的序列化
对于实现`Encoder`接口的数据类型都可以直接用`sink.write()`方法进行序列化,
`sink`进行初始化的时候,会初始化一个Vec,需要指定其初始化大小。

示例
```
let mut sink = Sink::new(16);
sink.write(83u8);
sink.write("transfer".to_string());
```

`Source`: 用于合约中数据的反序列化

对于实现`Decoder`接口类型的数据类型可以直接用`source.read().unwrap()`方法进行反序列化

示例
```
let input = runtime::input();
let mut source = Source::new(&input);
let (from, to, amount) = source.read().unwrap();
```

2. console 模块

- `debug`：用于在合约中打印调试信息

示例
```
 console::debug("debug");
```
>注意：测试节点启动的时候，日志级别要设置为debug模式，该信息才会打印出来。

3. contract模块
该模块封装了wasm合约中调用`ont`和`ong`的相关方法。
- `ong`：封装了在合约中调用ong的相关操作，例如转账、查询余额等。
   - `allowance(from: &Address, to: &Address)` 查询allowance余额
     示例
    ```
    use ostd::contract::ont;
    ont::allowance(from, to)
    ```
   - `approve(from: &Address, to: &Address, amount: U128)` 一个地址允许另一个地址转移多少资产

     示例
    ```
    use ostd::contract::ont;
    ont::approve(from, to, amount)
    ```
   - `balance_of` 查询余额

     示例：
     ```
     use ostd::contract::ont;
     ong::balance_of(address)
     ```
   - `transfer` 转账,从`from`地址转移`amount`数量的`ong`到`to`地址

     示例
    ```
    let state = ont::State { from: from.clone(), to: to.clone(), amount: amount };
    ont::transfer(&[state])
    ```
   - `transfer_from`

     示例
    ```
    ont::transfer_from(sender, from, to, amount)
    ```
- `ont`:封装了在合约中调用ont的相关操作,调用方法和ong类似。



4. database 模块
- `delete`: 根据key删除数据库中的数据
- `get`   : 根据key查询数据
- `put`   : 根据key存储数据

示例：
```
use ostd::database;
database::put(from, frmbal);
let balance = database::get(owner).unwrap_or(0);
```

5. types 模块
- `Address`: 地址，是长度为20的字节数组
- `U128`   : 小端序的大整数。

6. runtime 模块

该模块封装了合约和链交互的api，开发者可以调用一下接口获得链的信息或者更新链上数据。

- `timestamp() -> u64` 获得当前时间戳

示例
```
runtime::timestamp()
```
- `block_height() -> u32` 获得当前区块高度

示例
```
runtime::block_height()
```
- `address() -> Address` 获得当前合约地址

示例
```
runtime::address()
```

- `caller() -> Address` 获得调用者的合约地址

示例
```
runtime::caller()
```
- `current_blockhash() -> H256` 获得当前区块hash

示例
```
runtime::current_blockhash()
```
- `current_txhash() -> H256` 获得当前交易hash

示例
```
runtime::current_txhash()
```
- `check_witness(addr: &Address) -> bool` 校验签名

示例
```
runtime::check_witness(addr)
```
- `ret(data: &[u8]) -> !` 合约执行结束时调用，返回执行结果

示例
```
let mut dispatcher = ApiTestDispatcher::new(ApiTestInstance);
runtime::ret(&dispatcher.dispatch(&runtime::input()));
```
- `notify(data: &[u8])` 合约中推送事件

示例
```
runtime::notify("success".as_bytes());
```
- `contract_create(
    code: &[u8], need_storage: u32, name: &str, ver: &str, author: &str, email: &str, desc: &str,
) -> Option<Address>` 创建合约
在合约中通过该接口可以创建一个新的合约
`code`:合约字节码
`need_storage`:是否需要存储
`name`:合约名
`version`:合约版本
`author`:作者
`email`:邮箱信息
`desc`:合约描述信息。

示例
```
let code = &[
    0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x12, 0x03, 0x60, 0x00, 0x00,
    0x60, 0x02, 0x7f, 0x7f, 0x00, 0x60, 0x05, 0x7f, 0x7f, 0x7f, 0x7f, 0x7f, 0x01, 0x7f,
    0x02, 0x2d, 0x02, 0x03, 0x65, 0x6e, 0x76, 0x0c, 0x6f, 0x6e, 0x74, 0x69, 0x6f, 0x5f,
    0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x00, 0x01, 0x03, 0x65, 0x6e, 0x76, 0x12, 0x6f,
    0x6e, 0x74, 0x69, 0x6f, 0x5f, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x5f, 0x72,
    0x65, 0x61, 0x64, 0x00, 0x02, 0x03, 0x02, 0x01, 0x00, 0x05, 0x03, 0x01, 0x00, 0x01,
    0x07, 0x0a, 0x01, 0x06, 0x69, 0x6e, 0x76, 0x6f, 0x6b, 0x65, 0x00, 0x02, 0x0a, 0x5b,
    0x01, 0x59, 0x00, 0x41, 0x00, 0x41, 0x25, 0x3a, 0x00, 0x00, 0x41, 0x00, 0x41, 0x01,
    0x41, 0x08, 0x41, 0x08, 0x41, 0x00, 0x10, 0x01, 0x41, 0x08, 0x47, 0x04, 0x40, 0x00,
    0x0b, 0x41, 0x08, 0x29, 0x03, 0x00, 0x42, 0x97, 0x85, 0xce, 0x00, 0x52, 0x04, 0x40,
    0x00, 0x0b, 0x41, 0x00, 0x41, 0x14, 0x3a, 0x00, 0x00, 0x41, 0x00, 0x41, 0x01, 0x41,
    0x08, 0x41, 0x08, 0x41, 0x00, 0x10, 0x01, 0x41, 0x08, 0x47, 0x04, 0x40, 0x00, 0x0b,
    0x41, 0x08, 0x29, 0x03, 0x00, 0x42, 0xb3, 0xce, 0x0c, 0x52, 0x04, 0x40, 0x00, 0x0b,
    0x41, 0x08, 0x41, 0x08, 0x10, 0x00, 0x0b,
];
let contract_addr = runtime::contract_create(code, 1,"oep4","1.0","author","email","desc").unwrap_or(Address::zero());
```

- `fn contract_migrate(
    code: &[u8], vm_type: u32, name: &str, version: &str, author: &str, email: &str, desc: &str,
) -> Option<Address>`
合约升级
`code`:合约字节码
`vm_type`:虚拟机类型
`name`:合约名
`version`:合约版本
`author`:作者
`email`:邮箱信息
`desc`:合约描述信息。
示例
```
let address =runtime::contract_migrate(code, 3, "name", "version", "author", "email", "desc")
    .expect("migrate failed");
```

- `call_contract(addr: &Address, input: &[u8]) -> Option<Vec<u8>>` 跨合约调用
  - `addr`目标合约地址
  - `input` 调用目标合约的参数
由于调用wasm合约和调用neovm合约参数序列化方式不一样，所以，需要区别对待，
1. wasm调用wasm合约
wasm合约调用另外一本wasm合约时，参数序列化规则是，先序列化方法名，在序列化被调用合约方法需要的参数。
示例
```
let mut sink = Sink::new(16);
sink.write(("transfer".to_string(), from, to, amount));
let res = runtime::call_contract(contract, sink.bytes());
if res.is_some() {
    true
} else {
    false
}
```
2. wasm调用neovm合约

待定

### 常用的数据类型转换介绍
- `u64`转换成`string`
示例
```
let s = 123.to_string();
```
- `base58`编码的地址转换成`Address`
示例
```
let address = ostd::base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhPbdYzM");
```
- `u128_to_neo_bytes(data: U128) -> Vec<u8>` U128数据类型转换成字节数组

示例
```
 let bs = u128_to_neo_bytes(256 as U128);
```
- `u128_from_neo_bytes(buf: &[u8]) -> U128`字节数组转换成U128

示例
```
 let bs = u128_to_neo_bytes(256 as U128);
 let u = u128_from_neo_bytes(&bs);
```

## Start writing Contarct
1. 新建合约
```
cargo new --lib oep4-contract
```

生成的目录结构
```
ubuntu@ubuntu oep4-contract $ tree
.
├── Cargo.toml
└── src
    └── lib.rs
```

`cargo.toml` 文件用于配置项目基本信息和项目依赖信息等
`lib.rs`文件用于编写合约逻辑

2. 编辑`Cargo.toml`，添加`ontio-cdk`依赖:
```toml
[package]
name = "mycontract"
version = "0.1.0"
authors = ["laizy <aochyi@126.com>"]
edition = "2018"

[lib]
crate-type = ["cdylib"] #编译为动态链接库

[dependencies]
ontio-std = {git = "https://github.com/ontio/ontology-wasm-cdt-rust"}

[features]
mock = ["ontio-std/mock"]
```

`crate-type = ["cdylib"]` 表示将项目编译动态链接库，用于被其他语言调用。

3. 在src/lib.rs中开发合约，合约的基本结构如下：

```rust
#![no_std]
use ontio_std::runtime;

// 合约的入口函数，使用no_mangle使其在编译后作为wasm合约的导出函数
#[no_mangle]
pub fn invoke() {
    runtime::ret(b"hello, world");//将合约的执行结果返回
}
```
`#![no_std]`表示不使用标准库中的方法
`#[no_mangle]` 函数修饰符，被其修饰的函数，rust编译器不会为他进行函数名混淆。

4. Oep4合约示例

```rust
#![no_std]
extern crate ontio_std as ostd;

use ostd::abi::{Encoder, Sink, Source};
use ostd::prelude::*;
use ostd::{database, runtime};

const KEY_TOTAL_SUPPLY: &str = "total_supply";
const NAME: &str = "wasm_token";
const SYMBOL: &str = "WTK";
const TOTAL_SUPPLY: U128 = 100000000000;

fn initialize() -> bool {
    database::put(KEY_TOTAL_SUPPLY, TOTAL_SUPPLY);
    true
}

fn balance_of(owner: &Address) -> U128 {
    database::get(owner).unwrap_or(0)
}

fn transfer(from: &Address, to: &Address, amount: U128) -> bool {
    assert!(runtime::check_witness(from));

    let frmbal = balance_of(from);
    let tobal = balance_of(to);
    if amount == 0 || frmbal < amount {
        return false;
    }

    database::put(from, frmbal - amount);
    database::put(to, tobal + amount);
    notify(("Transfer", from, to, amount));
    true
}

fn total_supply() -> U128 {
    database::get(KEY_TOTAL_SUPPLY).unwrap()
}

#[no_mangle]
pub fn invoke() {
    let input = runtime::input();
    let mut source = Source::new(&input);
    let action = source.read().unwrap();
    let mut sink = Sink::new(12);
    match action {
        "init" => sink.write(initialize()),
        "name" => sink.write(NAME),
        "symbol" => sink.write(SYMBOL),
        "totalSupply" => sink.write(total_supply()),
        "balanceOf" => {
            let addr = source.read().unwrap();
            sink.write(balance_of(addr));
        }
        "transfer" => {
            let (from, to, amount) = source.read().unwrap();
            sink.write(transfer(from, to, amount));
        }
        _ => panic!("unsupported action!"),
    }

    runtime::ret(sink.bytes())
}

fn notify<T: Encoder>(msg: T) {
    let mut sink = Sink::new(16);
    sink.write(msg);
    runtime::notify(sink.bytes());
}
```

## Contract Deploying
可以通过`ontology`命令行工具将合约部署到链上
```
./ontology contract deploy --vmtype 3 --code ./token.wasm.str --name oep4 --author "author" --email "email" --desc "desc" --gaslimit 22200000
```
>注意，需要先将wasm字节码文件转换成hex文件后，在执行上面的方法

## Contract Testing
调用合约中的方法
```
./main contract invoke --address 51113dbe9e984939c0435eacfcf4c78d50525090 --vmtype 3 --params 'string:init' --version 0 --return boolean
```
