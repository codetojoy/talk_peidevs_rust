
export class Foo {
  id: number;

  repeatString(s: string, n: number): string {
    let result: string = "";

    for(let i = 1; i <= n; i++) {
        result += s;
    }

    return result;
  }
}

