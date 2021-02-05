export interface Animal {
  getNoise(): string;
  getAge(): number;
}

export function oldest<T extends Animal>(x: T, y: T): T {
  console.log(`TRACER x: ${x.getNoise()}`);
  console.log(`TRACER y: ${y.getNoise()}`);
  let result: T = x.getAge() > y.getAge() ? x : y;
  return result;
}
