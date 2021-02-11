import { Animal } from "./animal";

export class Cat implements Animal {
  public name: string;
  public age: number;

  public getNoise(): string {
    return `${this.name} says MEOW`;
  }

  public getAge(): number {
    return this.age;
  }

  public getName(): string {
    return this.name;
  }
}

export class Dog implements Animal {
  public name: string;
  public age: number;

  public getNoise(): string {
    return `${this.name} says WOOF`;
  }

  public getAge(): number {
    return this.age;
  }

  public getName(): string {
    return this.name;
  }
}
