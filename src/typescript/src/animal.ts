export interface Animal {
  getNoise(): string;
  getAge(): number;
  getName(): string;
}

export function oldest(x: Animal, y: Animal): Animal {
  console.log(`TRACER x: ${x.getNoise()}`);
  console.log(`TRACER y: ${y.getNoise()}`);
  let result: Animal = x.getAge() > y.getAge() ? x : y;
  return result;
}
