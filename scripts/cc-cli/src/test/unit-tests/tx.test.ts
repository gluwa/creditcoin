import { BN } from "creditcoin-js";
import { AccountBalance } from "../../utils/balance";
import { canPay } from "../../utils/tx";

describe("unit tests: utils/tx", () => {
  test("given a payable amount, canPay returns true", () => {
    const balance: AccountBalance = {
      address: "fakeAddress",
      total: new BN(100),
      bonded: new BN(0),
      unbonding: new BN(0),
      transferable: new BN(100),
      locked: new BN(0),
    };

    const amountUsedInTx = new BN(10);

    const txFee = new BN(1);

    const totalCost = amountUsedInTx.add(txFee);

    const canPayResult = canPay(balance, totalCost);

    expect(canPayResult).toStrictEqual(true);
  });

  test("given an unpayable amount, canPay returns false", () => {
    const balance: AccountBalance = {
      address: "fakeAddress",
      total: new BN(100),
      bonded: new BN(0),
      unbonding: new BN(0),
      transferable: new BN(100),
      locked: new BN(0),
    };

    const amountUsedInTx = new BN(100);

    const txFee = new BN(1);

    const totalCost = amountUsedInTx.add(txFee);

    const canPayResult = canPay(balance, totalCost);

    expect(canPayResult).toStrictEqual(false);
  });
});
