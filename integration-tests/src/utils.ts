/* eslint-disable */
import { ApiPromise } from 'creditcoin-js';
import type { EventRecord, Balance, DispatchError, DispatchResult } from 'creditcoin-js';

export const testIf = (condition: boolean, name: string, fn: any, timeout = 30000) =>
    condition ? test(name, fn, timeout) : test.skip(name, fn, timeout);

export const expectNoDispatchError = (api: ApiPromise, dispatchError?: DispatchError): void => {
    if (dispatchError) {
        if (dispatchError.isModule) {
            const decoded = api.registry.findMetaError(dispatchError.asModule);
            const { docs, name, section } = decoded;

            expect(`${section}.${name}: ${docs.join(' ')}`).toBe('');
        } else {
            expect(dispatchError.toString()).toBe('');
        }
    }
};

const isDispatchError = (instance: any): instance is DispatchResult => {
    return (instance as DispatchResult) != undefined;
};

export const expectNoEventError = (api: ApiPromise, eventRecord: EventRecord) => {
    const {
        event: { data },
    } = eventRecord;
    if (data[0] && isDispatchError(data[0])) {
        const dispatchResult = data[0] as DispatchResult;
        if (dispatchResult.isErr) {
            expectNoDispatchError(api, dispatchResult.asErr);
        }
    }
};

export const extractFee = async (
    resolve: any,
    reject: any,
    unsubscribe: any,
    api: ApiPromise,
    dispatchError: DispatchError | undefined,
    events: EventRecord[],
    status: any,
): Promise<void> => {
    expectNoDispatchError(api, dispatchError);
    if (status.isInBlock) {
        const balancesWithdraw = events.find(({ event: { method, section } }) => {
            return section === 'balances' && method === 'Withdraw';
        });

        expect(balancesWithdraw).toBeTruthy();

        if (balancesWithdraw) {
            const fee = (balancesWithdraw.event.data[1] as Balance).toBigInt();

            const unsub = await unsubscribe;

            if (unsub) {
                unsub();
                resolve(fee);
            } else {
                reject(new Error('Subscription failed'));
            }
        } else {
            reject(new Error("Fee wasn't found"));
        }
    }
};
