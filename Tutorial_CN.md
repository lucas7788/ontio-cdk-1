# 快速入门

## 相关工具
- cargo
- rust
- ontio-cdk

## 开始
1. 生成ontio-cdk api文档
克隆`https://github.com/ontio/ontology-wasm-cdt-rust.git`项目到本地，然后进入项目根目录，执行`cargo doc`命令生成api文档。

2. 合约中数据类型转换

3. 合约中验证签名

4. 合约与链交互

## ontio-std介绍

1. abi 模块

`Sink`  : 用于合约中数据类型的序列化

`Source`: 用于合约中数据类型的反序列化

2. abi_codegen模块

为基本的数据类型实现encoder和decoder接口，方便开发者在合约中序列化和反序列化数据。

3. console 模块
`debug`：用于在合约中打印调试信息

4. contract模块
`ong`：封装了在合约中调用ong的相关操作，例如转账、查询余额等。

调用示例：
```
use ostd::contract::ont;
ong::balance_of(address)
```
`ont`:封装了在合约中调用ont的相关操作,调用方法和ong类似。

5. database 模块
- `delete`: 根据key删除数据库中的数据
- `get`   : 根据key查询数据
- `put`   : 根据key存储数据

示例：
```
use ostd::database;
database::put(from, frmbal);
```

6. types 模块
- `Address`: 地址，是长度为20的字节数
- `U256`   : 小端序的大整数。

7. runtime 模块
该模块封装了合约和链交互的api
- `address`:获得当前合约地址
- `block_height`:获得当前区块高度
- `call_height`:调用另外一个合约
- `caller`: 获得调用者的合约地址
- `check_witness`: 校验签名
- `contract_migrate`：合约升级
- `current_blockhash`:获得当前的区块hash
- `current_txhash`: 获得当前的交易hash
- `notify`:合约中推送的事件
- `ret`：合约执行结束时调用，返回执行结果
- `timestamp`: 获得当前区块的时间戳