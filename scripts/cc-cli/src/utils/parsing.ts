import { validateAddress } from "@polkadot/util-crypto/address";
import { BN, parseUnits } from "creditcoin-js";

// Parse valid or exit with error
export const parseAddresOrExit = parseOrExit(parseAddress);
export const parseAmountOrExit = parseOrExit(parseAmount);
export const parseHexStringOrExit = parseOrExit(parseHexString);
export const parseIntegerOrExit = parseOrExit(parseInteger);
export const parsePercentAsPerbillOrExit = parseOrExit(parsePercentAsPerbill);
export const parseChoiceOrExit = parseChoiceOrExitFn;

// A function that takes a parsing function and returns a new function that does tries to parse or prints the error and exits
function parseOrExit<T>(parse: (input: string) => T): (input: string) => T {
  return (input: string) => {
    try {
      return parse(input);
    } catch (e: any) {
      console.error(`Unable to parse input. ${e.message as string}`);
      process.exit(1);
    }
  };
}

function parseChoiceOrExitFn(input: string, choices: string[]): string | never {
  try {
    return parseChoice(input, choices);
  } catch (e: any) {
    console.error(`Unable to parse input. ${e.message as string}`);
    process.exit(1);
  }
}

export function parseAddress(input: string): string {
  try {
    validateAddress(input);
  } catch (e: any) {
    throw new Error(`Invalid address: ${e.message as string}`);
  }
  return input;
}

export function parseAmount(input: string): BN {
  try {
    const parsed = positiveBigNumberFromString(input);
    return new BN(parsed.toString());
  } catch (e: any) {
    throw new Error(`Invalid amount: ${e.message as string}`);
  }
}

// Choices must be in Capitalized form: ['Staked', 'Stash', 'Controller']
export function parseChoice(input: string, choices: string[]): string {
  const styled = input.charAt(0).toUpperCase() + input.slice(1).toLowerCase();
  if (!choices.includes(styled)) {
    throw new Error(
      `Invalid choice: ${input}, must be one of ${choices.toString()}`
    );
  }
  return styled;
}

export function parseBoolean(input: true | undefined): boolean {
  return input ? true : false;
}

export function parseInteger(input: string): number {
  const float = Number.parseFloat(input);
  if (float % 1 !== 0) {
    throw new Error("Must be an integer");
  }
  const int = Number.parseInt(input, 10);
  return int;
}

export function parseHexString(input: string): string {
  if (!input.match(/^0x[\da-f]+$/i)) {
    throw new Error("Must be a valid hexadecimal number");
  }
  return input;
}

export function parsePercentAsPerbill(input: string): number {
  if (input.match(/[^0-9.]/)) {
    throw new Error("Percent value must be a number");
  }
  const value = Number.parseFloat(input);
  if (value < 0 || value > 100) {
    throw new Error("Percent value must be between 0 and 100");
  }
  return Math.floor(value * 10_000_000);
}

function positiveBigNumberFromString(amount: string) {
  const parsedValue = parseUnits(amount, 18);

  if (parsedValue.isZero()) {
    throw new Error("Must be greater than 0");
  }

  if (parsedValue.isNegative()) {
    throw new Error("Must be a positive number");
  }

  return parsedValue;
}

export function inputOrDefault(
  input: string | undefined,
  defaultValue: string
): string {
  if (input === undefined) {
    return defaultValue;
  }
  return input;
}

export function requiredInput(
  input: string | undefined,
  message: string
): string {
  if (input === undefined) {
    console.error(message);
    process.exit(1);
  }
  return input;
}
