import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { ApiPromise, BN } from "creditcoin-js";
import { readAmount, readAmountFromHex, toCTCString } from "../utils/balance";
import { timeTillEra } from "../utils/era";

export function makeStatusCommand() {
  const cmd = new Command("status");
  cmd.description("Get staking status for an address");
  cmd.option("-a, --address [address]", "Address to get status for");
  cmd.action(statusAction);
  return cmd;
}

async function statusAction(options: OptionValues) {
  const { api } = await newApi(options.url);

  const res = await api.derive.staking.account(options.address);
  const totalStaked = readAmount(res.stakingLedger.total.toString());
  const bonded = totalStaked.gt(new BN(0));

  const controller = res.controllerId ? res.controllerId.toString() : "null";

  let unlocking = res.stakingLedger.unlocking.toJSON();
  const currentEra = await api.query.staking.currentEra();
  unlocking = unlocking.filter((u: any) => u.era > currentEra);

  const redeemable = res.redeemable
    ? readAmountFromHex(res.redeemable.toString())
    : new BN(0);

  const nextUnbondingDate = unlocking.length > 0 ? unlocking[0].era : null;

  const nextUnbondingAmount =
    unlocking.length > 0 ? readAmountFromHex(unlocking[0].value) : null;

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
    controller,
    validating: validatorEntries.includes(options.address),
    waiting: waitingValidators.includes(options.address),
    active: activeValidators.includes(options.address),
    nextUnbondingDate,
    nextUnbondingAmount: nextUnbondingAmount ? nextUnbondingAmount : new BN(0),
    redeemable,
  };

  await printValidatorStatus(validatorStatus, api);

  process.exit(0);
}

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

async function printValidatorStatus(status: Status, api: ApiPromise) {
  console.log("Bonded: ", status.bonded);
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

interface Status {
  bonded: boolean;
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
