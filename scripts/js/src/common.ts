export const u8aToHex = (bytes: Uint8Array | Buffer): string => {
    const byteArray = Uint8Array.from(bytes);
    return byteArray.reduce((str, byte) => str + byte.toString(16).padStart(2, '0'), '0x');
};
