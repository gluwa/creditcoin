import {
  parseAddressInternal,
  parseAmountInternal,
  parseBoolean,
  parseChoiceInternal,
  parseHexStringInternal,
  parseIntegerInternal,
  parsePercentAsPerbillInternal,
} from "../../utils/parsing";

describe("parseAddress", () => {
  test("with valid argument returns the same address", () => {
    const substrateAddress = "5EACfEfYjfg5ZHpzp9uoMCR68UNGBUAu5AYjvZdM5aLYaojx";
    const parsedAddress = parseAddressInternal(substrateAddress);
    expect(parsedAddress).toBe(substrateAddress);
  });

  test("with invalid argument throws an error", () => {
    // address is too short
    const substrateAddress = "5EACfEfYjfg5ZHpzp9uoMCZdM5aLYaojx";
    const parsedInvalid = () => parseAddressInternal(substrateAddress);
    expect(parsedInvalid).toThrowError(Error);
  });
});

describe("parseAmount", () => {
  test("with valid integer argument returns the same amount * 10^^18", () => {
    const amount = "1";
    const parsedAmount = parseAmountInternal(amount);
    expect(parsedAmount.toString()).toBe("1000000000000000000");
  });

  test("with valid float argument returns the same amount * 10^^18", () => {
    const amount = "0.4";
    const parsedAmount = parseAmountInternal(amount);
    expect(parsedAmount.toString()).toBe("400000000000000000");
  });

  test("with negative argument throws an error", () => {
    const amount = "-1";
    const parsedInvalid = () => parseAmountInternal(amount);
    expect(parsedInvalid).toThrowError(Error);
  });

  test("with argument containing decimal comma throws an error", () => {
    const amount = "100,1";
    const parsedInvalid = () => parseAmountInternal(amount);
    expect(parsedInvalid).toThrowError(Error);
  });

  test("with 0 as argument throws an error", () => {
    const amount = "0";
    const parsedInvalid = () => parseAmountInternal(amount);
    expect(parsedInvalid).toThrowError(Error);
  });

  test("with string argument throws an error", () => {
    const amount = "abcdef";
    const parsedInvalid = () => parseAmountInternal(amount);
    expect(parsedInvalid).toThrowError(Error);
  });
});

describe("parseChoice", () => {
  test("with valid argument returns the same choice", () => {
    const choice = "Staked";
    const choices = ["Staked", "Stash", "Controller"];
    const parsedChoice = parseChoiceInternal(choice, choices);
    expect(parsedChoice).toBe(choice);
  });

  test("with valid mixed case argument returns choice in canonical format", () => {
    const choice = "stAKed";
    const choices = ["Staked", "Stash", "Controller"];
    const parsedChoice = parseChoiceInternal(choice, choices);
    expect(parsedChoice).toBe("Staked");
  });

  test("with invalid argument throws an error", () => {
    const choice = "Bonded";
    const choices = ["Staked", "Stash", "Controller"];
    const parsedInvalid = () => parseChoiceInternal(choice, choices);
    expect(parsedInvalid).toThrowError(Error);
  });
});

describe("parseBoolean", () => {
  test("with 'true' argument returns true", () => {
    const bool = true;
    const parsedBool = parseBoolean(bool);
    expect(parsedBool).toBe(bool);
  });

  test("with 'undefined' argument returns false", () => {
    const bool = undefined;
    const parsedBool = parseBoolean(bool);
    expect(parsedBool).toBe(false);
  });
});

describe("parseInteger", () => {
  test("with valid argument, 0, returns the same integer", () => {
    const parsedInteger = parseIntegerInternal("0");
    expect(parsedInteger).toBe(0);
  });

  test("with valid argument, > 0, returns the same integer", () => {
    const parsedInteger = parseIntegerInternal("1");
    expect(parsedInteger).toBe(1);
  });

  test("with valid argument, < 0, returns the same integer", () => {
    const parsedInteger = parseIntegerInternal("-1");
    expect(parsedInteger).toBe(-1);
  });

  test("with float argument throws an error", () => {
    const parsedInvalid = () => parseIntegerInternal("0.1");
    expect(parsedInvalid).toThrowError(Error);
  });

  test("with string argument throws an error", () => {
    const integer = "abcdef";
    const parsedInvalid = () => parseIntegerInternal(integer);
    expect(parsedInvalid).toThrowError(Error);
  });
});

describe("parseHexString", () => {
  test("with valid argument, lower case, returns the same hex string", () => {
    const hexString = "0x1234567890abcdef";
    const parsedHexString = parseHexStringInternal(hexString);
    expect(parsedHexString).toBe(hexString);
  });

  test("with valid argument, mixed case, returns the same hex string", () => {
    const hexString = "0x1234567890AbCdeF";
    const parsedHexString = parseHexStringInternal(hexString);
    expect(parsedHexString).toBe(hexString);
  });

  test("with invalid argument, missing 0x prefix, throws an error", () => {
    const hexString = "1234567890abcdef";
    const parsedInvalid = () => parseHexStringInternal(hexString);
    expect(parsedInvalid).toThrowError(Error);
  });

  test("with invalid argument, contains invalid hex digits, throws an error", () => {
    const parsedInvalid = () => parseHexStringInternal("0x123x==xZZZ");
    expect(parsedInvalid).toThrowError(Error);
  });
});

describe("parsePercentAsPerbill", () => {
  test("with valid argument, lower boundary, returns correct perbill", () => {
    const parsedPerbill = parsePercentAsPerbillInternal("0");
    expect(parsedPerbill).toBe(0);
  });

  test("with valid argument, upper boundary, returns correct perbill", () => {
    const parsedPerbill = parsePercentAsPerbillInternal("100");
    expect(parsedPerbill).toBe(1_000_000_000);
  });

  test("with valid argument, float value inside boundaries, returns correct perbill", () => {
    const parsedPerbill = parsePercentAsPerbillInternal("50.5");
    expect(parsedPerbill).toBe(505_000_000);
  });

  test("with invalid argument, float < lower boundary, throws an error", () => {
    const parsedInvalid = () => parsePercentAsPerbillInternal("-0.1");
    expect(parsedInvalid).toThrowError(Error);
  });

  test("with invalid argument, int < lower boundary, throws an error", () => {
    const parsedInvalid = () => parsePercentAsPerbillInternal("-1");
    expect(parsedInvalid).toThrowError(Error);
  });

  test("with invalid argument, float > upper boundary, throws an error", () => {
    const perbill = "100.1";
    const parsedInvalid = () => parsePercentAsPerbillInternal(perbill);
    expect(parsedInvalid).toThrowError(Error);
  });

  test("with invalid argument, int > upper boundary, throws an error", () => {
    const parsedInvalid = () => parsePercentAsPerbillInternal("101");
    expect(parsedInvalid).toThrowError(Error);
  });

  test("with invalid argument, a string, throws an error", () => {
    const perbill = "abcdef";
    const parsedInvalid = () => parsePercentAsPerbillInternal(perbill);
    expect(parsedInvalid).toThrowError(Error);
  });
});
