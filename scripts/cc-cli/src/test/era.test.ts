import { eraIsInHistory } from "../utils/era";

describe("eraIsInHistory", () => {
  it("should return false when checking the current era", () => {
    expect(eraIsInHistory(1, 84, 1)).toBe(false);
  });

  it("should return true when checking the previous era", () => {
    expect(eraIsInHistory(1, 84, 2)).toBe(true);
  });

  it("should return false when checking anera past the history depth", () => {
    expect(eraIsInHistory(1, 84, 86)).toBe(false);
  });

  it("should return false when checking an era in the future", () => {
    expect(eraIsInHistory(2, 84, 1)).toBe(false);
  });
});
