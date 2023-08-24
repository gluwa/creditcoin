import { ApiPromise, BN } from "creditcoin-js";

// Returns time in milliseconds contained in a BN
export async function timeTillEra(api: ApiPromise, era: number) {
  const eraNumber = new BN(era);
  const currentEra = new BN((await api.query.staking.currentEra()).toString());
  const eraLength = new BN(api.consts.staking.sessionsPerEra.toString());
  const slotsPerEpoch = new BN(api.consts.babe.epochDuration.toString());
  const slotsPerEra = eraLength.mul(slotsPerEpoch);
  const eraProgress = new BN(
    (await api.derive.session.eraProgress()).toString(),
  );
  const blocksTillNextEra = slotsPerEra.sub(eraProgress);
  const minBlockTime = new BN(api.consts.babe.expectedBlockTime.toString());

  const timeTillTargetEra = eraNumber
    .sub(currentEra.add(new BN(1)))
    .mul(slotsPerEra)
    .add(blocksTillNextEra)
    .mul(minBlockTime);

  return timeTillTargetEra;
}

export async function checkEraIsInHistory(
  era: number,
  api: ApiPromise,
): Promise<boolean> {
  const currentEra = (await api.query.staking.currentEra()).value.toNumber();
  const historyDepth = api.consts.staking.historyDepth.toNumber();
  return eraIsInHistory(era, historyDepth, currentEra);
}

export function eraIsInHistory(
  eraToCheck: number,
  historyDepth: number,
  currentEra: number,
): boolean {
  if (eraToCheck < 0) {
    return false;
  }
  // The oldest era in history is currentEra - historyDepth
  // https://polkadot.js.org/docs/kusama/constants/#historydepth-u32
  const oldestEraInHistory = currentEra - historyDepth;
  if (eraToCheck < oldestEraInHistory || eraToCheck >= currentEra) {
    return false;
  } else {
    return true;
  }
}
