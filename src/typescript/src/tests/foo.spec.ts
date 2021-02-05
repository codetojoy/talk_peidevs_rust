
import { Foo } from "../foo";

describe("Foo", () => {
  test("repeat string", () => {
    let foo: Foo = new Foo();
    let s: string = "abc";
    let n: number = 3;

    // test
    let result: string = foo.repeatString(s, n);

    expect(result).toBe("abcabcabc");
  });
});
