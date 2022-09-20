import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { Codec } from '@polkadot/types-codec/types';
import { EventReturnType } from '../model';
import { DispatchError, DispatchResult, EventRecord } from '@polkadot/types/interfaces';
import { TxCallback, TxFailureCallback } from 'src';
import { AugmentedEvent, VoidFn } from '@polkadot/api/types';
import { AnyTuple } from '@polkadot/types/types';

export const handleTransaction = (
    api: ApiPromise,
    unsubscribe: () => void,
    result: SubmittableResult,
    onSuccess: TxCallback,
    onFail: TxFailureCallback,
) => {
    const { status, events, dispatchError } = result;
    console.log(`current status is ${status.toString()}`);

    try {
        expectNoDispatchError(api, dispatchError);
        if (events) events.forEach((event) => expectNoEventError(api, event));
    } catch (error) {
        unsubscribe();
        onFail(error as Error);
        // we need to return here, otherwise we'll run the onSuccess handler below
        return;
    }

    if (status.isInBlock) {
        events.forEach(({ event }) => {
            const types = event.typeDef;
            event.data.forEach((data, index) =>
                console.log(
                    `pallet: ${event.section}, name: ${event.method}, types: ${
                        types[index].type
                    }, data: ${data.toString()}`,
                ),
            );
        });

        unsubscribe();
        onSuccess(result);
    }
};

export const processEvents = <IdType, ItemType, SourceType extends Codec>(
    api: ApiPromise,
    result: SubmittableResult,
    eventMethod: string,
    creditcoinType: string,
    transform: (data: SourceType) => ItemType,
): EventReturnType<IdType, ItemType> => {
    const { events } = result;
    const sourceEvents = events.find(({ event }) => event.method === eventMethod);
    if (!sourceEvents) throw new Error(`No ${eventMethod} events found`);

    const [id, codecItem] = sourceEvents.event.data;

    const itemId = id.toJSON() as unknown as IdType;

    const transformWrapper = (dataItem: Codec, transformFn: (data: SourceType) => ItemType) => {
        const sourceItem = api.createType(creditcoinType, dataItem) as SourceType;
        return transformFn(sourceItem);
    };

    return codecItem ? { itemId, item: transformWrapper(codecItem, transform) } : { itemId };
};

const isDispatchError = (instance: any): instance is DispatchResult => {
    return (instance as DispatchResult) !== undefined;
};

export const expectNoEventError = (api: ApiPromise, eventRecord: EventRecord) => {
    const {
        event: { data },
    } = eventRecord;
    if (data[0] && isDispatchError(data[0])) {
        const dispatchResult = data[0];
        if (dispatchResult.isErr) {
            expectNoDispatchError(api, dispatchResult.asErr);
        }
    }
};

const parseModuleError = (api: ApiPromise, dispatchError: DispatchError): string => {
    const decoded = api.registry.findMetaError(dispatchError.asModule);
    const { docs, name, section } = decoded;
    return `${section}.${name}: ${docs.join(' ')}`;
};

export const expectNoDispatchError = (api: ApiPromise, dispatchError?: DispatchError): void => {
    if (dispatchError) {
        const errString = dispatchError.isModule ? parseModuleError(api, dispatchError) : dispatchError.toString();
        throw new Error(errString);
    }
};

export const listenForVerificationOutcome = <T extends AnyTuple, U extends AnyTuple, O>(
    api: ApiPromise,
    options: {
        successEvent: AugmentedEvent<'promise', T, unknown>;
        failEvent: AugmentedEvent<'promise', U, unknown>;
        processSuccessEvent: (data: T) => Promise<O | undefined>;
        processFailEvent: (data: U) => any | undefined;
    },
    timeout = 60_000,
) => {
    const { failEvent, processFailEvent, processSuccessEvent, successEvent } = options;
    return new Promise<O>((resolve, reject) => {
        const timer: NodeJS.Timeout = setTimeout(() => reject(new Error('Verification timed out')), timeout);
        const clearAndCall = <V, Out>(fun: (arg: V) => Out) => {
            return (value: V) => {
                clearTimeout(timer);
                return fun(value);
            };
        };
        const ifDefined = <V, Out>(fun: (arg: V) => Out) => {
            return (value: V | undefined) => {
                if (value !== undefined) {
                    return fun(value);
                }
            };
        };
        let unsub: VoidFn | undefined;
        const unsubAnd = <V, Out>(fun: (arg: V) => Out) => {
            return (value: V) => {
                if (unsub) {
                    unsub();
                }
                return fun(value);
            };
        };
        const unsubAndResolve = unsubAnd(resolve);
        const unsubAndReject = unsubAnd(reject);

        api.query.system
            .events((events) => {
                events.forEach(({ event }) => {
                    if (successEvent.is(event)) {
                        processSuccessEvent(event.data)
                            .then(ifDefined(clearAndCall(unsubAndResolve)))
                            .catch(clearAndCall(unsubAndReject));
                    } else if (failEvent.is(event)) {
                        try {
                            // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
                            const result = processFailEvent(event.data);
                            ifDefined(clearAndCall(unsubAndReject))(result);
                        } catch (e) {
                            clearAndCall(unsubAndReject)(e);
                        }
                    }
                });
            })
            .then((us) => (unsub = us))
            .catch(reject);
    });
};
