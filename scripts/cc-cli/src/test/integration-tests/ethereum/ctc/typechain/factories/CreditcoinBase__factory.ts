/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */

import { Signer, utils, Contract, ContractFactory, Overrides } from "ethers";
import { Provider, TransactionRequest } from "@ethersproject/providers";
import type {
  CreditcoinBase,
  CreditcoinBaseInterface,
} from "../CreditcoinBase";

const _abi = [
  {
    constant: true,
    inputs: [],
    name: "decimals",
    outputs: [
      {
        name: "",
        type: "uint8",
      },
    ],
    payable: false,
    stateMutability: "view",
    type: "function",
  },
  {
    constant: true,
    inputs: [],
    name: "creditcoinLimitInFrac",
    outputs: [
      {
        name: "",
        type: "uint256",
      },
    ],
    payable: false,
    stateMutability: "view",
    type: "function",
  },
  {
    payable: true,
    stateMutability: "payable",
    type: "fallback",
  },
];

const _bytecode =
  "0x6080604052336000806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555061015c806100536000396000f3fe608060405260043610610046576000357c010000000000000000000000000000000000000000000000000000000090048063313ce567146100be578063d1479575146100ef575b600015156100bc576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260198152602001807f657468207472616e736665722069732064697361626c65642e0000000000000081525060200191505060405180910390fd5b005b3480156100ca57600080fd5b506100d361011a565b604051808260ff1660ff16815260200191505060405180910390f35b3480156100fb57600080fd5b5061010461011f565b6040518082815260200191505060405180910390f35b601281565b601260ff16600a0a6377359400028156fea165627a7a7230582095b6ebdd1c43f0b5d3df1fd9cf5100cc84620d0abd808ef5b2fd2249c40d32790029";

export class CreditcoinBase__factory extends ContractFactory {
  constructor(
    ...args: [signer: Signer] | ConstructorParameters<typeof ContractFactory>
  ) {
    if (args.length === 1) {
      super(_abi, _bytecode, args[0]);
    } else {
      super(...args);
    }
  }

  deploy(
    overrides?: Overrides & { from?: string | Promise<string> },
  ): Promise<CreditcoinBase> {
    return super.deploy(overrides || {}) as Promise<CreditcoinBase>;
  }
  getDeployTransaction(
    overrides?: Overrides & { from?: string | Promise<string> },
  ): TransactionRequest {
    return super.getDeployTransaction(overrides || {});
  }
  attach(address: string): CreditcoinBase {
    return super.attach(address) as CreditcoinBase;
  }
  connect(signer: Signer): CreditcoinBase__factory {
    return super.connect(signer) as CreditcoinBase__factory;
  }
  static readonly bytecode = _bytecode;
  static readonly abi = _abi;
  static createInterface(): CreditcoinBaseInterface {
    return new utils.Interface(_abi) as CreditcoinBaseInterface;
  }
  static connect(
    address: string,
    signerOrProvider: Signer | Provider,
  ): CreditcoinBase {
    return new Contract(address, _abi, signerOrProvider) as CreditcoinBase;
  }
}