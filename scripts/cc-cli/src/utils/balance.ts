export interface Balance {
  free: number;
  reserved: number;
  miscFrozen: number;
  feeFrozen: number;
}

export async function getBalance(address: string, api: any) {
  const account = await api.query.system.account(address);
  // console.log(data.toHuman);
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
  console.log(
    `Available: ${(balance.free / 1000000000000000000).toString()} CTC`
  );
  console.log(
    `Bonded: ${(balance.miscFrozen / 1000000000000000000).toString()} CTC`
  );
}
