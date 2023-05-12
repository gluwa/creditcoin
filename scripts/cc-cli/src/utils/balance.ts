export interface Balance {
    free: number,
    reserved: number,
    miscFrozen: number,
    feeFrozen: number,
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
    }
}

export function balanceIsZero(balance: Balance): boolean {
    return !(balance.free > 0) && !(balance.reserved > 0) && !(balance.miscFrozen > 0) && !(balance.feeFrozen > 0);
}

export function balanceFreeIsZero(balance: Balance): boolean {
    return !(balance.free > 0);
}

export function printBalance(balance: Balance) {
    console.log("Available: " + balance.free / 1000000000000000000, "CTC");
    console.log("Bonded: " + balance.miscFrozen / 1000000000000000000, "CTC");
}




//    // @ts-ignore TODO fix errror related to augmented-apis
//     const {nonce, data: balance} = await api.query.system.account(address);

//     console.log("Account address:", address);
//     console.log(balance.toHuman());
//     console.log("Available: " + balance.free / 1000000000000000000, "CTC");
//     console.log("Bonded: " + balance.miscFrozen / 1000000000000000000, "CTC");
