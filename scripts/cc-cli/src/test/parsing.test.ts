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
    let substrateAddress = "5EACfEfYjfg5ZHpzp9uoMCR68UNGBUAu5AYjvZdM5aLYaojx";
    let parsedAddress = parseAddress(substrateAddress);
    expect(parsedAddress).toBe(substrateAddress);
  });

  test("parsed invalid address throws error", () => {
    let substrateAddress = "5EACfEfYjfg5ZHpzp9uoMCZdM5aLYaojx";
    let parsedInvalid = () => parseAddress(substrateAddress);
    expect(parsedInvalid).toThrowError(Error);
  });
});

describe("parseAmount", () => {
  test("parsed valid amount returns same amount * 10^^18", () => {
    let amount = "100";
    let parsedAmount = parseAmount(amount);
    expect(parsedAmount.toString()).toBe("100000000000000000000");
  });

  test("parsed negative amount throws error", () => {
    let amount = "-100";
    let parsedInvalid = () => parseAmount(amount);
    expect(parsedInvalid).toThrowError(Error);
  });

  test("parsed invalid decimal char throws error", () => {
    let amount = "100,1";
    let parsedInvalid = () => parseAmount(amount);
    expect(parsedInvalid).toThrowError(Error);
  });

  test("parsed zero amount throws error", () => {
    let amount = "0";
    let parsedInvalid = () => parseAmount(amount);
    expect(parsedInvalid).toThrowError(Error);
  });

  test("parsed string throws error", () => {
    let amount = "abcdef";
    let parsedInvalid = () => parseAmount(amount);
    expect(parsedInvalid).toThrowError(Error);
  });
});

describe("parseChoice", () => {
  test("parsed valid choice returns same choice", () => {
    let choice = "Staked";
    let choices = ["Staked", "Stash", "Controller"];
    let parsedChoice = parseChoice(choice, choices);
    expect(parsedChoice).toBe(choice);
  });

  test("parsed different choice returns formatted choice", () => {
    let choice = "staked";
    let choices = ["Staked", "Stash", "Controller"];
    let parsedChoice = parseChoice(choice, choices);
    expect(parsedChoice).toBe("Staked");
  });

  test("parsed invalid choice throws error", () => {
    let choice = "Bonded";
    let choices = ["Staked", "Stash", "Controller"];
    let parsedInvalid = () => parseChoice(choice, choices);
    expect(parsedInvalid).toThrowError(Error);
  });
});

describe("parseBoolean", () => {
  test("parsed true returns true", () => {
    let bool = true;
    let parsedBool = parseBoolean(bool);
    expect(parsedBool).toBe(bool);
  });

  test("parsed undefined returns false", () => {
    let bool = undefined;
    let parsedBool = parseBoolean(bool);
    expect(parsedBool).toBe(false);
  });
});

describe("parseInteger", () => {
  test("parsed valid integer returns same integer", () => {
    let integer = "100";
    let parsedInteger = parseInteger(integer);
    expect(parsedInteger).toBe(100);
  });

  test("parsed float throws error", () => {
    let integer = "100.1";
    let parsedInvalid = () => parseInteger(integer);
    expect(parsedInvalid).toThrowError(Error);
  });

  test("parsed string throws error", () => {
    let integer = "abcdef";
    let parsedInvalid = () => parseInteger(integer);
    expect(parsedInvalid).toThrowError(Error);
  });
});

describe("parseHexString", () => {
  test("parsed valid hex string returns same hex string", () => {
    let hexString = "0x1234567890abcdef";
    let parsedHexString = parseHexString(hexString);
    expect(parsedHexString).toBe(hexString);
  });

  test("parsed invalid hex string throws error", () => {
    let hexString = "1234567890abcdef";
    let parsedInvalid = () => parseHexString(hexString);
    expect(parsedInvalid).toThrowError(Error);
  });
});

describe("parsePercentAsPerbill", () => {
  test("parsed valid percent returns correct perbill", () => {
    let percent = "100";
    let parsedPerbill = parsePercentAsPerbill(percent);
    expect(parsedPerbill).toBe(100 * 10_000_000);
  });

  test("parsed invalid perbill throws error", () => {
    let perbill = "100.1";
    let parsedInvalid = () => parsePercentAsPerbill(perbill);
    expect(parsedInvalid).toThrowError(Error);
  });

  test("parsed not number throws error", () => {
    let perbill = "abcdef";
    let parsedInvalid = () => parsePercentAsPerbill(perbill);
    expect(parsedInvalid).toThrowError(Error);
  });
});
