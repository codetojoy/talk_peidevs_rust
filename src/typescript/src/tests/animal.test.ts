import { Animal, oldest } from "../animal";
import { Cat, Dog } from "../pets";

describe("oldest", () => {
  test("sanity check", () => {
    let cat: Cat = new Cat();
    cat.name = "Mozart";
    cat.age = 4;

    let dog: Dog = new Dog();
    dog.name = "Beethoven";
    dog.age = 3;

    // test
    let result: Animal = oldest(cat, dog);

    expect(result.getName()).toBe("Mozart");
  });
});
