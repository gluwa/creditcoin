import { BN } from "creditcoin-js";

const MICROUNITS_PER_CTC = new BN("1000000000000000000");

export function parseCTCString(amount: string): BN {
  return new BN(amount).mul(MICROUNITS_PER_CTC);
}

export function toMicrounits(amount: number | BN): BN {
  return new BN(amount).mul(MICROUNITS_PER_CTC);
}

export function toCTCString(amount: BN): string {
  return amount.div(MICROUNITS_PER_CTC).toString() + " CTC";
}

export function readAmount(amount: string): BN {
  return new BN(amount);
}

export function readAmountFromHex(amount: string): BN {
  return new BN(amount.slice(2), 16);
}

export interface Balance {
  free: BN;
  reserved: BN;
  miscFrozen: BN;
  feeFrozen: BN;
}

export async function getBalance(address: string, api: any) {
  const account = await api.query.system.account(address);
  return balanceFromData(account.data);
}

function balanceFromData(data: any): Balance {
  return {
    free: data.free,
    reserved: data.reserved,
    miscFrozen: data.miscFrozen,
    feeFrozen: data.feeFrozen,
  };
}

export function printBalance(balance: Balance) {
  console.log("Available:", toCTCString(balance.free.sub(balance.miscFrozen)));
  console.log("Free:", toCTCString(balance.free));
  console.log("Reserved:", toCTCString(balance.reserved));
  console.log("Misc Frozen:", toCTCString(balance.miscFrozen));
  console.log("Fee Frozen:", toCTCString(balance.feeFrozen));
}
