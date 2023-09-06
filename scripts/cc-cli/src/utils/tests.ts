export const describeIf = (condition: boolean, name: string, fn: any) =>
    condition ? describe(name, fn) : describe.skip(name, fn);
