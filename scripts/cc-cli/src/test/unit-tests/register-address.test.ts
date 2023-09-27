import { Wallet } from "creditcoin-js";
import {
  isMnemonicValid,
  isValidPrivateKey,
  newWalletFromMnemonic,
  newWalletFromPrivateKey,
} from "../../commands/registerAddress";
import { utils } from "ethers";

describe(isValidPrivateKey, () => {
  test("should return true when called with valid private key", () => {
    expect(
      isValidPrivateKey(
        "0x8da4ef21b864d2cc526dbdb2a120bd2874c36c9d0a1fb7f8c63d7f7a8b41de8f",
      ),
    ).toBe(true);
  });

  test("should return false when input is missing the '0x' prefix", () => {
    expect(
      isValidPrivateKey(
        "8da4ef21b864d2cc526dbdb2a120bd2874c36c9d0a1fb7f8c63d7f7a8b41de8f",
      ),
    ).toBe(false);
  });

  test("should return false when key length < 64", () => {
    expect(
      isValidPrivateKey(
        "0x8da4ef21b864d2cc526dbdb2a120bd2874c36c9d0a1fb7f8c63d7f7a8b41de8",
      ),
    ).toBe(false);
  });

  test("should return false when key length > 64", () => {
    expect(
      isValidPrivateKey(
        "0x8da4ef21b864d2cc526dbdb2a120bd2874c36c9d0a1fb7f8c63d7f7a8b41de8feeee",
      ),
    ).toBeFalsy();
  });

  test("should return false when argument is empty string", () => {
    expect(isValidPrivateKey("")).toBe(false);
  });

  test("should return false when argument has non hexadecimal characters", () => {
    expect(
      isValidPrivateKey(
        "0x8da4ef21b864d2cc526dbdb2-INVALID-4c36c9d0a1fb7f8c63d7f7a8b41de8f",
      ),
    ).toBe(false);
  });
});

describe(newWalletFromPrivateKey, () => {
  test("should return new wallet with valid private key", () => {
    const goodWallet = Wallet.createRandom();

    const testWallet = newWalletFromPrivateKey(goodWallet.privateKey);
    expect(testWallet).toBeTruthy();
    expect(testWallet.privateKey).toEqual(goodWallet.privateKey);
    expect(testWallet).toBeInstanceOf(Wallet);
  });

  test("should throw error with invalid private key", () => {
    const goodWallet = Wallet.createRandom();
    const badPk = goodWallet.privateKey.substring(
      0,
      goodWallet.privateKey.length - 1,
    );
    expect(() => {
      newWalletFromPrivateKey(badPk);
    }).toThrow("Error: Could not create wallet from private key:");
  });

  test("should thow error when called with empty string", () => {
    expect(() => {
      newWalletFromPrivateKey("");
    }).toThrow("Error: Could not create wallet from private key:");
  });
});

describe(newWalletFromMnemonic, () => {
  test("should return new wallet with valid mnemonic", () => {
    const mnemonic = utils.entropyToMnemonic(utils.randomBytes(32));
    const testWallet = newWalletFromMnemonic(mnemonic);
    expect(testWallet).toBeTruthy();
    expect(testWallet).toBeInstanceOf(Wallet);
  });

  test("should throw error when called with bad mnemonic", () => {
    // construct a bad mnemonic by taking a good one and dropping the last word
    const mnemonic = utils.entropyToMnemonic(utils.randomBytes(32)).split(" ");
    mnemonic.pop();
    const badMnemonic = mnemonic.join(" ");
    expect(() => {
      newWalletFromMnemonic(badMnemonic);
    }).toThrow("Error: Could not create wallet from mnemonic:");
  });

  test("should throw error when called with empty string", () => {
    expect(() => {
      newWalletFromMnemonic("");
    }).toThrow("Error: Could not create wallet from mnemonic:");
  });
});

describe(isMnemonicValid, () => {
  test("should return true when called with valid mnemonic", () => {
    expect(
      isMnemonicValid(utils.entropyToMnemonic(utils.randomBytes(32))),
    ).toBe(true);
  });

  test("should return false when called with empty string", () => {
    expect(isMnemonicValid("")).toBe(false);
  });

  test("should return false when called with odd length mnemonic", () => {
    // construct a bad mnemonic by taking a good one and dropping the last word
    const mnemonic = utils.entropyToMnemonic(utils.randomBytes(32)).split(" ");
    mnemonic.pop();
    const badMnemonic = mnemonic.join(" ");

    expect(isMnemonicValid(badMnemonic)).toBe(false);
  });
});
