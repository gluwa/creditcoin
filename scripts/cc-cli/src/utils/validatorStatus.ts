import { ApiPromise, BN } from "creditcoin-js";
import { readAmount, readAmountFromHex, toCTCString } from "./balance";
import { timeTillEra } from "./era";
import Table from "cli-table3";

function formatDaysHoursMinutes(ms: number) {
  const days = Math.floor(ms / (24 * 60 * 60 * 1000));
  const daysms = ms % (24 * 60 * 60 * 1000);
  const hours = Math.floor(daysms / (60 * 60 * 1000));
  const hoursms = ms % (60 * 60 * 1000);
  const minutes = Math.floor(hoursms / (60 * 1000));
  const minutesms = ms % (60 * 1000);
  const sec = Math.floor(minutesms / 1000);

  const daysString = days > 0 ? `${days} days, ` : ``;
  const hoursString = hours > 0 ? `${hours} hours, ` : ``;
  const minutesString = minutes > 0 ? `${minutes} minutes, ` : ``;
  const secString = sec > 0 ? `${sec} seconds` : ``;

  return `${daysString}${hoursString}${minutesString}${secString}`;
}

export interface StashControllerPair {
  stash: string;
  controller?: string;
}

export interface ControllerStatus {
  isController: boolean;
  stash?: string;
}

export async function getControllerStatus(
  address: string,
  api: ApiPromise
): Promise<ControllerStatus> {
  const stashRes = await api.query.staking.ledger(address);
  const stash = stashRes.isSome
    ? stashRes.unwrap().stash.toString()
    : undefined;

  let status;
  if (stash) {
    status = {
      isController: true,
      stash,
    };
  } else {
    status = {
      isController: false,
      stash: undefined,
    };
  }
  return status;
}

export async function getValidatorStatus(address: string, api: ApiPromise) {
  // Check if address is a controller and get its stash
  const controllerStatus = await getControllerStatus(address, api);

  // If it is a controller, set the stash to its stash address
  let stash;
  if (controllerStatus.isController && controllerStatus.stash) {
    console.log(
      `Address belongs to the Controller account for validator ${controllerStatus.stash}`
    );
    console.log(`Showing status for ${controllerStatus.stash}...`);
    stash = controllerStatus.stash;
  } else {
    stash = address;
  }

  // Get the staking information for the stash
  const res = await api.derive.staking.account(stash);

  // Get the controller address
  const controller = res.controllerId ? res.controllerId.toString() : undefined;

  // Get the total staked amount
  const totalStaked = readAmount(res.stakingLedger.total.toString());
  const bonded = totalStaked.gt(new BN(0));

  // Get information about any unbonding tokens and unlocked chunks
  const unlockingRes = res.stakingLedger.unlocking;
  const currentEra = (await api.query.staking.currentEra()).unwrap();
  const unlocking = unlockingRes
    ? unlockingRes.filter((u: any) => u.era > currentEra)
    : [];

  const redeemable = res.redeemable
    ? readAmountFromHex(res.redeemable.toString())
    : new BN(0);

  // Get the unlocked chunks that are ready for withdrawal
  // by comparing the era of each chunk to the current era
  const readyForWithdraw = res.stakingLedger.unlocking
    .map((u: any) => {
      const chunk: UnlockChunk = {
        era: u.era.toNumber(),
        value: u.value.toBn(),
      };
      return chunk;
    })
    .filter((u: UnlockChunk) => u.era < currentEra.toNumber());

  const canWithdraw = readyForWithdraw.length > 0;

  const nextUnbondingDate =
    unlocking.length > 0 ? unlocking[0].era.toNumber() : null;

  const nextUnbondingAmount =
    unlocking.length > 0 ? unlocking[0].value.toBn() : null;

  // Get lists of all validators, active validators, and waiting validators
  const validatorEntries = await api.query.staking.validators
    .entries()
    .then((r) => r.map((v) => v[0].toHuman()?.toString()));
  const activeValidatorsRes = await api.derive.staking.validators();
  const activeValidators: string[] = activeValidatorsRes.validators.map((v) =>
    v.toString()
  );
  const waitingValidators = validatorEntries.filter((v) => {
    if (v !== undefined) {
      return !activeValidators.includes(v);
    } else {
      return false;
    }
  });

  // Check if the validator is validating, waiting, or active
  const validating = validatorEntries.includes(stash);
  const waiting = waitingValidators.includes(stash);
  const active = activeValidators.includes(stash);

  const validatorStatus: Status = {
    bonded,
    stash,
    controller,
    validating,
    waiting,
    active,
    canWithdraw,
    readyForWithdraw,
    nextUnbondingDate,
    nextUnbondingAmount: nextUnbondingAmount || new BN(0),
    redeemable,
  };

  return validatorStatus;
}

export async function printValidatorStatus(status: Status, api: ApiPromise) {
  const table = new Table({
    head: ["Status"],
  });

  table.push(["Bonded", status.bonded ? "Yes" : "No"]);
  table.push(["Stash", status.stash ? status.stash : "None"]);
  table.push(["Controller", status.controller]);
  table.push(["Validating", status.validating ? "Yes" : "No"]);
  table.push(["Waiting", status.waiting ? "Yes" : "No"]);
  table.push(["Active", status.active ? "Yes" : "No"]);
  table.push(["Can withdraw", status.canWithdraw ? "Yes" : "No"]);
  if (status.canWithdraw) {
    status.readyForWithdraw.forEach((chunk) => {
      table.push([`Unlocked since era ${chunk.era}`, toCTCString(chunk.value)]);
    });
  }
  let nextUnlocking;
  if (status.nextUnbondingAmount?.eq(new BN(0))) {
    nextUnlocking = "None";
  } else if (status.nextUnbondingAmount && status.nextUnbondingDate) {
    const nextUnbondingAmount = toCTCString(status.nextUnbondingAmount);
    const nextUnbondingDate = await timeTillEra(api, status.nextUnbondingDate);
    nextUnlocking = `${nextUnbondingAmount} in ${formatDaysHoursMinutes(
      nextUnbondingDate.toNumber()
    )}`;
  }
  table.push(["Next unlocking", nextUnlocking]);

  console.log(table.toString());
}

export function requireStatus(
  status: Status,
  condition: keyof Status,
  message?: string
) {
  if (!status[condition]) {
    console.error(
      message ||
        `Cannot perform action, validator is not ${condition.toString()}`
    );
    process.exit(1);
  }
}

export interface Status {
  bonded: boolean;
  stash?: string;
  controller?: string;
  validating: boolean;
  waiting: boolean;
  active: boolean;
  canWithdraw: boolean;
  readyForWithdraw: UnlockChunk[];
  nextUnbondingDate: Option<EraNumber>;
  nextUnbondingAmount: Option<Balance>;
  redeemable: Balance;
}

interface UnlockChunk {
  era: EraNumber;
  value: Balance;
}

type Balance = BN;

type EraNumber = number;

type Option<T> = T | null;
