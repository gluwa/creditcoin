import { isValidPrivateKey } from "../../commands/registerAddress";

describe(isValidPrivateKey, () => {
  test("should return true when called with valid private key", () => {
    expect(
      isValidPrivateKey(
        "0x8da4ef21b864d2cc526dbdb2a120bd2874c36c9d0a1fb7f8c63d7f7a8b41de8f",
      ),
    ).toBe(true);
  });

  test("should return false when input is missing prefix", () => {
    expect(
      isValidPrivateKey(
        "8da4ef21b864d2cc526dbdb2a120bd2874c36c9d0a1fb7f8c63d7f7a8b41de8f",
      ),
    ).toBe(false);
  });

  test("should return false when input is missing final character", () => {
    expect(
      isValidPrivateKey(
        "0x8da4ef21b864d2cc526dbdb2a120bd2874c36c9d0a1fb7f8c63d7f7a8b41de8",
      ),
    ).toBe(false);
  });

  test("should return false when argument is empty string", () => {
    expect(isValidPrivateKey("")).toBe(false);
  });

  test("should return false when argument has non hexadecimal characters", () => {
    expect(
      isValidPrivateKey(
        "0x8da4ef21b864d2cc526dbdb2a120bd2874c36c9d0a1fb7f8c63d7f7a8b41de8fZ",
      ),
    ).toBe(false);
  });

  test("should return false when argument non hexadecimal characters", () => {
    expect(
      isValidPrivateKey(
        "0x8da4ef21b864d2cc526dbdb2a120bd2874c36c9d0a1fb7f8c63d7f7a8b41de8ff",
      ),
    ).toBe(false);
  });
});
