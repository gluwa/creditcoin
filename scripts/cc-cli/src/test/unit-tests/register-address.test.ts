import { isValidPrivateKey } from "../../commands/registerAddress";

describe('Test isValidPrivateKey', () => {
    test('Success', () => {
        expect(isValidPrivateKey("0x8da4ef21b864d2cc526dbdb2a120bd2874c36c9d0a1fb7f8c63d7f7a8b41de8f")).toBeTruthy()
    });

    test('fails with missing prefix', () => {
        expect(isValidPrivateKey("8da4ef21b864d2cc526dbdb2a120bd2874c36c9d0a1fb7f8c63d7f7a8b41de8f")).toBeFalsy()
    })

    test('fails with missing final character', () => {
        expect(isValidPrivateKey("0x8da4ef21b864d2cc526dbdb2a120bd2874c36c9d0a1fb7f8c63d7f7a8b41de8")).toBeFalsy()
    })

    test('fails with empty string', () => {
        expect(isValidPrivateKey('')).toBeFalsy()
    })
})