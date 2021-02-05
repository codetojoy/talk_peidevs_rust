import { from, fromEvent } from "rxjs";
import { take, tap, map, toArray, mergeMap, reduce } from "rxjs/operators";
import { of, range } from "rxjs";

const now = new Date().toLocaleTimeString([], {
  hour: "2-digit",
  minute: "2-digit",
  second: "2-digit",
});

let displayTextArea = <HTMLElement>document.querySelector("#display");
let goButton = document.querySelector("#go");

let goClick$ = fromEvent(goButton, "click");

const wordList = [
  "Johann Sebastian Bach",
  "Ludwig van Beethoven",
  "Wolfgang Amadeus Mozart",
  "Franz Schubert",
  "Richard Wagner",
  "Antonio Vivaldi",
  "Johannes Brahms",
  "Giuseppe Verdi",
  "Robert Schumann",
  "Giacomo Puccini",
  "Antonín Leopold Dvorák",
  "George Frideric Handel",
  "Franz Liszt",
  "Franz Joseph Haydn",
  "Frédéric Chopin",
  "Igor Stravinsky",
  "Gustav Mahler",
  "Richard Strauss",
  "Dmitri Dmitriyevich Shostakovich",
  "Hector Berlioz",
];

function f(a, b) {
  return a == b ? 0 : a > b ? 1 : -1;
}

class Container {
  private _word: string;

  constructor(value: string) {
    this._word = value;
  }

  get word(): string {
    return this._word;
  }

  public toString() {
    return this._word;
  }
}

class MyCompare {
  public sort(list: Container[]) {
    list.sort((a, b) => {
      // return f(a[0].toLowerCase(), b[0].toLowerCase());
      return this.foo(a.word[0].toLowerCase(), b.word[0].toLowerCase());
    });
  }

  foo(a, b) {
    return a == b ? 0 : a > b ? -1 : 1;
  }
}

goClick$.subscribe(() => {
  displayTextArea.innerHTML = "";
  let containerList = wordList.map((word) => new Container(word));
  new MyCompare().sort(containerList);
  from(containerList).subscribe((word) => {
    displayTextArea.innerHTML += `${word}\n`;
  });
});
