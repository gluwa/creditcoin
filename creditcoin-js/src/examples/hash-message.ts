import { hashMessage } from 'ethers/lib/utils';
import { Keyring } from '../index';

const keyring = new Keyring();
const alice = keyring.createFromUri('//Alice');

console.log(alice.addressRaw);
console.log(hashMessage(alice.addressRaw));
