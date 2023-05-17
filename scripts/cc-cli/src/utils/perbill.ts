export function perbillFromPercent(value: number) {
  if (value < 0 || value > 100) {
    throw new Error("Percent value must be between 0 and 100");
  }
  return Math.floor(value * 10_000_000);
}

export function percentFromPerbill(value: number) {
  if (value < 0 || value > 1_000_000_000) {
    throw new Error("Perbill value must be between 0 and 1,000,000,000");
  }
  return value / 10_000_000;
}
