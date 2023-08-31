import { isExternalAddressValid, isTxHashValid } from "../../commands/collectCoins";

describe('Test isTxHashValid', () => {
    test('success', () => {
        expect(isTxHashValid('0x2446f1fd773fbb9f080e674b60c6a033c7ed7427b8b9413cf28a2a4a6da9b56c')).toBeTruthy()
    })

    test('Fails with missing prefix', () => {
        expect(isTxHashValid('2446f1fd773fbb9f080e674b60c6a033c7ed7427b8b9413cf28a2a4a6da9b56c')).toBeFalsy()
    })

    test('fails with missing final character', () => {
        expect(isTxHashValid('0x2446f1fd773fbb9f080e674b60c6a033c7ed7427b8b9413cf28a2a4a6da9b56')).toBeFalsy()
    })

    test('fails with empty string', () => {
        expect(isTxHashValid("")).toBeFalsy()
    })

})

describe('Test isExternalAddressValid', () => {
    test('Success', () => {
        expect(isExternalAddressValid('0x71C7656EC7ab88b098defB751B7401B5f6d8976F')).toBeTruthy()
    })

    test('Succeds with missing prefix', () => {
        expect(isExternalAddressValid('71C7656EC7ab88b098defB751B7401B5f6d8976F')).toBeTruthy()
    })

    test('Fails with missing final character', () => {
        expect(isExternalAddressValid('0x71C7656EC7ab88b098defB751B7401B5f6d8976')).toBeFalsy()
    })

    test('Fails with empty string', () => {
        expect(isExternalAddressValid('')).toBeFalsy()
    })
})