import { mnemonicGenerate } from "@polkadot/util-crypto/mnemonic/generate";
import { initECDSAKeyringPairFromPK, initKeyringPair } from "../../utils/account";
import { newApi } from "../../api";
import { signSendAndWatch } from "../../utils/tx";
import { ApiPromise, BN } from "creditcoin-js";
import * as secp from '@noble/secp256k1';


export const ALICE_NODE_URL = "ws://localhost:9944";
export const BOB_NODE_URL = "ws://localhost:9945";

export function randomAccount() {
  const seed = mnemonicGenerate();
  const keyring = initKeyringPair(seed);
  const address = keyring.address;
  return { seed, keyring, address };
}

export async function fundAccounts(amount: BN) {
  const { api } = await newApi("ws://localhost:9944");
  const stash = randomAccount();
  const controller = randomAccount();
  const tx = await fundAddressesFromSudo(
    [stash.address, controller.address],
    amount,
  );
  await signSendAndWatch(tx, api, initKeyringPair("//Alice"));

  return { stash, controller };
}

export async function fundFromSudo(
  address: string,
  amount: BN,
  url = "ws://localhost:9944",
) {
  const { api } = await newApi(url);
  const call = api.tx.balances.setBalance(address, amount.toString(), "0");
  const tx = api.tx.sudo.sudo(call);
  return tx;
}

export async function fundAddressesFromSudo(
  addresses: string[],
  amount: BN,
  url = "ws://localhost:9944",
) {
  const { api } = await newApi(url);
  const txs = addresses.map((address) => {
    const fundTx = api.tx.balances.setBalance(address, amount.toString(), "0");
    return api.tx.sudo.sudo(fundTx);
  });
  const tx = api.tx.utility.batchAll(txs);
  return tx;
}

export async function waitEras(eras: number, api: ApiPromise, force = true) {
  if (force) {
    await forceNewEra(api);
  }
  let eraInfo = await api.derive.session.info();
  let currentEra = eraInfo.currentEra.toNumber();
  const targetEra = currentEra + eras;
  const blockTime = api.consts.babe.expectedBlockTime.toNumber();
  while (currentEra < targetEra) {
    console.log(`Waiting for era ${targetEra}, currently at ${currentEra}`);
    await new Promise((resolve) => setTimeout(resolve, blockTime));
    eraInfo = await api.derive.session.info();
    currentEra = eraInfo.currentEra.toNumber();
  }
}

export async function forceNewEra(api: ApiPromise) {
  const tx = api.tx.staking.forceNewEraAlways();
  const sudoTx = api.tx.sudo.sudo(tx);
  await signSendAndWatch(sudoTx, api, initKeyringPair("//Alice"));
}

export function randomTestAccount(ecdsa = false) {
  const secret = ecdsa ? "0x".concat(Buffer.from(secp.utils.randomPrivateKey()).toString('hex')) : mnemonicGenerate();
  const keyring = ecdsa ? initECDSAKeyringPairFromPK(secret) : initKeyringPair(secret);
  const address = keyring.address;
  return { secret, keyring, address };
}