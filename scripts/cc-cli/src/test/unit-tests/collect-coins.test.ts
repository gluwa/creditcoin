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

  test("should return false with missing '0x' prefix", () => {
    expect(
      isTxHashValid(
        "2446f1fd773fbb9f080e674b60c6a033c7ed7427b8b9413cf28a2a4a6da9b56c",
      ),
    ).toBe(false);
  });

  test("should return false when tx hash length < 64 characters", () => {
    expect(
      isTxHashValid(
        "0x2446f1fd773fbb9f080e674b60c6a033c7ed7427b8b9413cf28a2a4a6da9b56",
      ),
    ).toBe(false);
  });

  test("should return false when tx hash length > 64 characters", () => {
    expect(
      isTxHashValid(
        "0x2446f1fd773fbb9f080e674b60c6a033c7ed7427b8b9413cf28a2a4a6da9b56ceeeeee",
      ),
    ).toBe(false);
  });

  test("should return false when called with empty string", () => {
    expect(isTxHashValid("")).toBe(false);
  });

  test("Should return false when input has non hexadecimal character(s)", () => {
    expect(
      isTxHashValid(
        "0x2446f1fd773fbb9f080e674b60-INVALID-d7427b8b9413cf28a2a4a6da9b56c",
      ),
    ).toBe(false);
  });
});

describe(isExternalAddressValid, () => {
  test("should return true with valid address, prefixed with '0x'", () => {
    expect(
      isExternalAddressValid("0x71C7656EC7ab88b098defB751B7401B5f6d8976F"),
    ).toBe(true);
  });

  test("should return true with valid address missing the '0x' prefix", () => {
    expect(
      isExternalAddressValid("71C7656EC7ab88b098defB751B7401B5f6d8976F"),
    ).toBe(true);
  });

  test("Should return false when called with empty string", () => {
    expect(isExternalAddressValid("")).toBe(false);
  });

  test("should return false when argument length < 42, including the '0x' prefix", () => {
    expect(
      isExternalAddressValid("0x71C7656EC7ab88b098defB751B7401B5f6d8976"),
    ).toBe(false);
  });

  test("should return false when input has non hexadecimal characters", () => {
    expect(
      isExternalAddressValid("0x71C7656EC7ab88b098-INVALID-401B5f6d8976F"),
    ).toBe(false);
  });

  test("should return false when argument length > 42, including the '0x' prefix", () => {
    expect(
      isExternalAddressValid("0x71C7656EC7ab88b098defB751B7401B5f6d8976FF"),
    ).toBe(false);
  });

  test("should return false when argument length < 40, without the '0x' prefix", () => {
    expect(
      isExternalAddressValid("71C7656EC7ab88b098defB751B7401B5f6d8976"),
    ).toBe(false);
  });

  test("should return false when argument length > 40, without the '0x' prefix", () => {
    expect(
      isExternalAddressValid("71C7656EC7ab88b098defB751B7401B5f6d8976FF"),
    ).toBe(false);
  });
});
