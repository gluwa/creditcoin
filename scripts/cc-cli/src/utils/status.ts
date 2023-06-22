import { ApiPromise, BN } from "creditcoin-js";
import { readAmount, readAmountFromHex, toCTCString } from "./balance";
import { timeTillEra } from "./era";

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

export async function getStatus(address: string, api: ApiPromise) {
  const res = await api.derive.staking.account(address);
  const totalStaked = readAmount(res.stakingLedger.total.toString());
  const bonded = totalStaked.gt(new BN(0));

  const controller = res.controllerId ? res.controllerId.toString() : "None";

  const stashRes = await api.query.staking.ledger(address);
  const stash = stashRes.isSome
    ? stashRes.unwrap().stash.toString()
    : undefined;

  const unlockingRes = res.stakingLedger.unlocking;
  const currentEra = await api.query.staking.currentEra();
  const unlocking = unlockingRes
    ? unlockingRes.filter((u: any) => u.era > currentEra)
    : [];

  const redeemable = res.redeemable
    ? readAmountFromHex(res.redeemable.toString())
    : new BN(0);

  const nextUnbondingDate =
    unlocking.length > 0 ? unlocking[0].era.toNumber() : null;

  const nextUnbondingAmount =
    unlocking.length > 0 ? unlocking[0].value.toBn() : null;

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

  const validatorStatus: Status = {
    bonded,
    stash,
    controller,
    validating: validatorEntries.includes(address),
    waiting: waitingValidators.includes(address),
    active: activeValidators.includes(address),
    nextUnbondingDate,
    nextUnbondingAmount: nextUnbondingAmount ? nextUnbondingAmount : new BN(0),
    redeemable,
  };

  return validatorStatus;
}

export async function printValidatorStatus(status: Status, api: ApiPromise) {
  console.log("Bonded: ", status.bonded);
  console.log("Stash: ", status.stash);
  console.log("Controller: ", status.controller);
  console.log("Validating: ", status.validating);
  console.log("Waiting: ", status.waiting);
  console.log("Active: ", status.active);

  let nextUnlocking = "None";
  if (status.nextUnbondingAmount && status.nextUnbondingAmount.eq(new BN(0))) {
    nextUnlocking = "None";
  } else if (status.nextUnbondingAmount && status.nextUnbondingDate) {
    const nextUnbondingAmount = toCTCString(status.nextUnbondingAmount);
    const nextUnbondingDate = await timeTillEra(api, status.nextUnbondingDate);
    nextUnlocking = `${nextUnbondingAmount} in ${formatDaysHoursMinutes(
      nextUnbondingDate.toNumber()
    )}`;
  }
  console.log(`Next unbonding chunk: ${nextUnlocking}`);
}

export function requireStatus(status: Status, condition: keyof Status) {
  if (!status[condition]) {
    console.error(
      `Cannot perform action, validator is not ${condition.toString()}`
    );
    process.exit(1);
  }
}

export interface Status {
  bonded: boolean;
  stash?: string;
  controller: string;
  validating: boolean;
  waiting: boolean;
  active: boolean;
  nextUnbondingDate: Option<EraNumber>;
  nextUnbondingAmount: Option<Balance>;
  redeemable: Balance;
}

type Balance = BN;

type EraNumber = number;

type Option<T> = T | null;
