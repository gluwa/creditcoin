export function perbillFromPercent(value: number) {
  return Math.floor(value * 10_000_000);
}

export function percentFromPerbill(value: number) {
  return value / 10_000_000;
}
