package ontology
import (
	"bytes"
	"encoding/hex"
	"fmt"
	sdkcom "github.com/ontio/ontology-go-sdk/common"
	"github.com/ontio/ontology/common"
	"github.com/ontio/ontology/core/types"
	"github.com/ontio/ontology/smartcontract/states"
)


type Oep4_abiContract struct {
	contractAddr common.Address
	vm WasmVMContract
	gasPrice uint64
	gasLimit uint64
	signer *Account
	version byte
}

func(this *Oep4_abiContract) Deploy(gasPrice, gasLimit uint64,
	singer *Account,
	needStorage byte,
	code,
	name,
	version,
	author,
	email,
	desc string) (*types.MutableTransaction, error) {
	invokeCode, err := hex.DecodeString(code)
	if err != nil {
		return nil, fmt.Errorf("code hex decode error:%s", err)
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
	return tx, nil
}

func (this *Oep4_abiContract) buildParams(method string, params []interface{}) ([]byte, error) {
	contract := &states.ContractInvokeParam{}
	contract.Address = this.contractAddr
	contract.Method = method
	contract.Version = this.version

	argbytes, err := buildWasmContractParam(params)

	if err != nil {
		return nil, fmt.Errorf("build wasm contract param failed:%s", err)
	}
	contract.Args = argbytes
	bf := bytes.NewBuffer(nil)
	contract.Serialize(bf)
	return bf.Bytes(), nil
}
func (this *DemoContract) Name() (*types.MutableTransaction, error) {
	bs, err := this.buildParams("name", []interface{}{})
	if err != nil {
		return nil, fmt.Errorf("buildparams failed:s%", err)
	}
	tx := this.vm.ontSdk.NewInvokeWasmTransaction(this.gasPrice, this.gasLimit, bs)
	return tx, nil
}
func (this *DemoContract) Symbol() (*types.MutableTransaction, error) {
	bs, err := this.buildParams("symbol", []interface{}{})
	if err != nil {
		return nil, fmt.Errorf("buildparams failed:s%", err)
	}
	tx := this.vm.ontSdk.NewInvokeWasmTransaction(this.gasPrice, this.gasLimit, bs)
	return tx, nil
}
func (this *DemoContract) BalanceOf(owner common.Address) (*types.MutableTransaction, error) {
	bs, err := this.buildParams("balanceOf", []interface{}{owner})
	if err != nil {
		return nil, fmt.Errorf("buildparams failed:s%", err)
	}
	tx := this.vm.ontSdk.NewInvokeWasmTransaction(this.gasPrice, this.gasLimit, bs)
	return tx, nil
}
func (this *DemoContract) Transfer(fromAcct common.Address, toAcct common.Address, amount U256) (*types.MutableTransaction, error) {
	bs, err := this.buildParams("transfer", []interface{}{fromAcct, toAcct, amount})
	if err != nil {
		return nil, fmt.Errorf("buildparams failed:s%", err)
	}
	tx := this.vm.ontSdk.NewInvokeWasmTransaction(this.gasPrice, this.gasLimit, bs)
	return tx, nil
}
func (this *DemoContract) TransferMulti() (*types.MutableTransaction, error) {
	bs, err := this.buildParams("transferMulti", []interface{}{args})
	if err != nil {
		return nil, fmt.Errorf("buildparams failed:s%", err)
	}
	tx := this.vm.ontSdk.NewInvokeWasmTransaction(this.gasPrice, this.gasLimit, bs)
	return tx, nil
}
func (this *DemoContract) Approve(toAcct common.Address) (*types.MutableTransaction, error) {
	bs, err := this.buildParams("approve", []interface{}{toAcct})
	if err != nil {
		return nil, fmt.Errorf("buildparams failed:s%", err)
	}
	tx := this.vm.ontSdk.NewInvokeWasmTransaction(this.gasPrice, this.gasLimit, bs)
	return tx, nil
}
func (this *DemoContract) Init() (*types.MutableTransaction, error) {
	bs, err := this.buildParams("init", []interface{}{})
	if err != nil {
		return nil, fmt.Errorf("buildparams failed:s%", err)
	}
	tx := this.vm.ontSdk.NewInvokeWasmTransaction(this.gasPrice, this.gasLimit, bs)
	return tx, nil
}
func (this *DemoContract) TotalSupply() (*types.MutableTransaction, error) {
	bs, err := this.buildParams("totalSupply", []interface{}{})
	if err != nil {
		return nil, fmt.Errorf("buildparams failed:s%", err)
	}
	tx := this.vm.ontSdk.NewInvokeWasmTransaction(this.gasPrice, this.gasLimit, bs)
	return tx, nil
}
