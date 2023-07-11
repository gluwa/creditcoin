import { BN, parseUnits } from "creditcoin-js";
import Table from "cli-table3";

export const MICROUNITS_PER_CTC = new BN("1000000000000000000");

export function parseCTCString(amount: string): BN {
  try {
    const parsed = positiveBigNumberFromString(amount);
    return new BN(parsed.toString());
  } catch (e) {
    console.error(`Unable to parse CTC amount: ${amount}`);
    process.exit(1);
  }
}

export function toCTCString(amount: BN, decimals = 18): string {
  const CTC = amount.div(MICROUNITS_PER_CTC);
  const remainder = amount.mod(MICROUNITS_PER_CTC);
  const remainderString = remainder
    .toString()
    .padStart(18, "0")
    .slice(0, decimals);
  return `${CTC.toString()}.${remainderString} CTC`;
}

export function readAmount(amount: string): BN {
  return new BN(amount);
}

export function readAmountFromHex(amount: string): BN {
  return new BN(amount.slice(2), 16);
}

export interface AccountBalance {
  address: string;
  transferable: BN;
  locked: BN;
  bonded: BN;
  total: BN;
}

export async function getBalance(address: string, api: any) {
  const account = await api.query.system.account(address);
  return balanceFromData(account.data, address);
}

function balanceFromData(data: any, address: string): AccountBalance {
  return {
    address,
    transferable: data.free.sub(data.miscFrozen),
    bonded: data.miscFrozen,
    locked: data.reserved,
    total: data.free,
  };
}

export function logBalance(balance: AccountBalance, human = true) {
  if (human) {
    printBalance(balance);
  } else {
    printJsonBalance(balance);
  }
}

export function printBalance(balance: AccountBalance) {
  const table = new Table({});

  table.push(
    ["Transferable", toCTCString(balance.transferable, 4)],
    ["Locked", toCTCString(balance.locked, 4)],
    ["Bonded", toCTCString(balance.bonded, 4)],
    ["Total", toCTCString(balance.total, 4)]
  );

  console.log(`Address: ${balance.address}`);
  console.log(table.toString());
}

export function printJsonBalance(balance: AccountBalance) {
  const jsonBalance = {
    balance: {
      address: balance.address,
      transferable: balance.transferable.toString(),
      bonded: balance.bonded.toString(),
      locked: balance.locked.toString(),
      total: balance.total.toString(),
    },
  };
  console.log(JSON.stringify(jsonBalance, null, 2));
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

function positiveBigNumberFromString(amount: string) {
  const parsedValue = parseUnits(amount, 18);

  if (parsedValue.isZero()) {
    console.error("Failed to parse amount, must be greater than 0");
    process.exit(1);
  }

  if (parsedValue.isNegative()) {
    console.error("Failed to parse amount, must be a positive number");
    process.exit(1);
  }

  return parsedValue;
}
