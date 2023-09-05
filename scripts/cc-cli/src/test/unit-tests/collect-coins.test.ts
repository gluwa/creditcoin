import {
  isExternalAddressValid,
  isTxHashValid,
} from "../../commands/collectCoins";

describe(isTxHashValid, () => {
  test("should return true with a valid hash", () => {
    expect(
      isTxHashValid(
        "0x2446f1fd773fbb9f080e674b60c6a033c7ed7427b8b9413cf28a2a4a6da9b56c",
      ),
    ).toBe(true);
  });

  test("Should return false with missing '0x' prefix", () => {
    expect(
      isTxHashValid(
        "2446f1fd773fbb9f080e674b60c6a033c7ed7427b8b9413cf28a2a4a6da9b56c",
      ),
    ).toBe(false);
  });

  test("Should return false when hash is missing final character", () => {
    expect(
      isTxHashValid(
        "0x2446f1fd773fbb9f080e674b60c6a033c7ed7427b8b9413cf28a2a4a6da9b56",
      ),
    ).toBe(false);
  });

  test("should return false when called with empty", () => {
    expect(isTxHashValid("")).toBe(false);
  });

  test("Should return false when input has non hexadecimal character", () => {
    expect(
      isTxHashValid(
        "0x2446f1fd773fbb9f080e674b60c6a033c7ed7427b8b9413cf28a2a4a6da9b56Z",
      ),
    ).toBe(false);
  });
});

describe(isExternalAddressValid, () => {
  test("should return true with proper address", () => {
    expect(
      isExternalAddressValid("0x71C7656EC7ab88b098defB751B7401B5f6d8976F"),
    ).toBe(true);
  });

  test("should return true with proper address missing prefix", () => {
    expect(
      isExternalAddressValid("71C7656EC7ab88b098defB751B7401B5f6d8976F"),
    ).toBe(true);
  });

  test("should return false when argument is missing final character", () => {
    expect(
      isExternalAddressValid("0x71C7656EC7ab88b098defB751B7401B5f6d8976"),
    ).toBe(false);
  });

  test("Should return false when called with empty string", () => {
    expect(isExternalAddressValid("")).toBe(false);
  });

  test("Should return false when input has non hexadecimal characters", () => {
    expect(
      isExternalAddressValid("0x71C7656EC7ab88b098defB751B7401B5f6d8976Z"),
    ).toBe(false);
  });

  test("Should return false when called with input with length > 42", () => {
    expect(
      isExternalAddressValid("0x71C7656EC7ab88b098defB751B7401B5f6d8976FF"),
    ).toBe(false);
  });
});
