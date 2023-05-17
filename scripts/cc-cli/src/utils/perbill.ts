export function perbillFromPercent(value: number) {
  if (value < 0 || value > 100) {
    throw new Error("Percent value must be between 0 and 100");
  }
  return Math.floor(value * 10_000_000);
}

export function percentFromPerbill(value: number) {
  return value / 10_000_000;
}
