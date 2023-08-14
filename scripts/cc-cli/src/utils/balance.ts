import { ApiPromise, BN, parseUnits } from "creditcoin-js";
import Table from "cli-table3";

import type { DeriveStakingAccount } from "@polkadot/api-derive/types";

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
  unbonding: BN;
}

export async function getBalance(address: string, api: any) {
  const balacesAll = await getBalancesAll(address, api);
  const stakingInfo = await getStakingInfo(address, api);

  const balance: AccountBalance = {
    address,
    transferable: balacesAll.availableBalance,
    bonded: stakingInfo?.stakingLedger.active?.unwrap() || new BN(0),
    locked: balacesAll.lockedBalance,
    total: balacesAll.freeBalance.add(balacesAll.reservedBalance),
    unbonding: calcUnbonding(stakingInfo),
  };

  return balance;
}

async function getBalancesAll(address: string, api: ApiPromise) {
  const balance = await api.derive.balances.all(address);
  return balance;
}

async function getStakingInfo(address: string, api: ApiPromise) {
  const stakingInfo = await api.derive.staking.account(address);
  return stakingInfo;
}

function calcUnbonding(stakingInfo?: DeriveStakingAccount) {
  if (!stakingInfo?.unlocking) {
    return new BN(0);
  }

  const filtered = stakingInfo.unlocking
    .filter(
      ({ remainingEras, value }) =>
        value.gt(new BN(0)) && remainingEras.gt(new BN(0))
    )
    .map((unlock) => unlock.value);
  const unbonding = filtered.reduce(
    (total, value) => total.iadd(value),
    new BN(0)
  );

  return unbonding;
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
    ["Unbonding", toCTCString(balance.unbonding, 4)],
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
      unbonding: balance.unbonding.toString(),
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
