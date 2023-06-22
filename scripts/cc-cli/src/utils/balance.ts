import { BN, parseUnits } from "creditcoin-js";

export const MICROUNITS_PER_CTC = new BN("1000000000000000000");

export function parseCTCString(amount: string): BN {
  try {
    const parsed = parseUnits(amount, 18);
    return new BN(parsed.toString());
  } catch (e) {
    console.error(`Unable to parse CTC amount: ${amount}`);
    process.exit(1);
  }
}

export function toCTCString(amount: BN): string {
  const CTC = amount.div(MICROUNITS_PER_CTC);
  const remainder = amount.mod(MICROUNITS_PER_CTC);
  const remainderString = remainder.toString().padStart(18, "0");
  return `${CTC.toString()}.${remainderString} CTC`;
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

export function checkAmount(amount: BN) {
  if (!amount) {
    console.log("Must specify amount to bond");
    process.exit(1);
  } else {
    if (amount.lt(new BN(1).mul(MICROUNITS_PER_CTC))) {
      console.log("Bond amount must be at least 1 CTC");
      process.exit(1);
    }
  }
}
