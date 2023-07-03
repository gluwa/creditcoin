import {
  parseAddress,
  parseAmount,
  parseBoolean,
  parseChoice,
  parseHexString,
  parseInteger,
  parsePercentAsPerbill,
} from "../utils/parsing";

describe("parseAddress", () => {
  test("parsed valid address returns same address", () => {
    const substrateAddress = "5EACfEfYjfg5ZHpzp9uoMCR68UNGBUAu5AYjvZdM5aLYaojx";
    const parsedAddress = parseAddress(substrateAddress);
    expect(parsedAddress).toBe(substrateAddress);
  });

  test("parsed invalid address throws error", () => {
    const substrateAddress = "5EACfEfYjfg5ZHpzp9uoMCZdM5aLYaojx";
    const parsedInvalid = () => parseAddress(substrateAddress);
    expect(parsedInvalid).toThrowError(Error);
  });
});

describe("parseAmount", () => {
  test("parsed valid amount returns same amount * 10^^18", () => {
    const amount = "100";
    const parsedAmount = parseAmount(amount);
    expect(parsedAmount.toString()).toBe("100000000000000000000");
  });

  test("parsed negative amount throws error", () => {
    const amount = "-100";
    const parsedInvalid = () => parseAmount(amount);
    expect(parsedInvalid).toThrowError(Error);
  });

  test("parsed invalid decimal char throws error", () => {
    const amount = "100,1";
    const parsedInvalid = () => parseAmount(amount);
    expect(parsedInvalid).toThrowError(Error);
  });

  test("parsed zero amount throws error", () => {
    const amount = "0";
    const parsedInvalid = () => parseAmount(amount);
    expect(parsedInvalid).toThrowError(Error);
  });

  test("parsed string throws error", () => {
    const amount = "abcdef";
    const parsedInvalid = () => parseAmount(amount);
    expect(parsedInvalid).toThrowError(Error);
  });
});

describe("parseChoice", () => {
  test("parsed valid choice returns same choice", () => {
    const choice = "Staked";
    const choices = ["Staked", "Stash", "Controller"];
    const parsedChoice = parseChoice(choice, choices);
    expect(parsedChoice).toBe(choice);
  });

  test("parsed different choice returns formatted choice", () => {
    const choice = "staked";
    const choices = ["Staked", "Stash", "Controller"];
    const parsedChoice = parseChoice(choice, choices);
    expect(parsedChoice).toBe("Staked");
  });

  test("parsed invalid choice throws error", () => {
    const choice = "Bonded";
    const choices = ["Staked", "Stash", "Controller"];
    const parsedInvalid = () => parseChoice(choice, choices);
    expect(parsedInvalid).toThrowError(Error);
  });
});

describe("parseBoolean", () => {
  test("parsed true returns true", () => {
    const bool = true;
    const parsedBool = parseBoolean(bool);
    expect(parsedBool).toBe(bool);
  });

  test("parsed undefined returns false", () => {
    const bool = undefined;
    const parsedBool = parseBoolean(bool);
    expect(parsedBool).toBe(false);
  });
});

describe("parseInteger", () => {
  test("parsed valid integer returns same integer", () => {
    const integer = "100";
    const parsedInteger = parseInteger(integer);
    expect(parsedInteger).toBe(100);
  });

  test("parsed float throws error", () => {
    const integer = "100.1";
    const parsedInvalid = () => parseInteger(integer);
    expect(parsedInvalid).toThrowError(Error);
  });

  test("parsed string throws error", () => {
    const integer = "abcdef";
    const parsedInvalid = () => parseInteger(integer);
    expect(parsedInvalid).toThrowError(Error);
  });
});

describe("parseHexString", () => {
  test("parsed valid hex string returns same hex string", () => {
    const hexString = "0x1234567890abcdef";
    const parsedHexString = parseHexString(hexString);
    expect(parsedHexString).toBe(hexString);
  });

  test("parsed invalid hex string throws error", () => {
    const hexString = "1234567890abcdef";
    const parsedInvalid = () => parseHexString(hexString);
    expect(parsedInvalid).toThrowError(Error);
  });
});

describe("parsePercentAsPerbill", () => {
  test("parsed valid percent returns correct perbill", () => {
    const percent = "100";
    const parsedPerbill = parsePercentAsPerbill(percent);
    expect(parsedPerbill).toBe(100 * 10_000_000);
  });

  test("parsed invalid perbill throws error", () => {
    const perbill = "100.1";
    const parsedInvalid = () => parsePercentAsPerbill(perbill);
    expect(parsedInvalid).toThrowError(Error);
  });

  test("parsed not number throws error", () => {
    const perbill = "abcdef";
    const parsedInvalid = () => parsePercentAsPerbill(perbill);
    expect(parsedInvalid).toThrowError(Error);
  });
});
