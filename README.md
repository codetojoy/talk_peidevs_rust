### Summary

* Notes/examples for a talk on Rust, for [PEI Developers](http://peidevs.github.io/), February 2021.
* slides [here](./doc)
    - Note that I don't create slide-decks; I aim to tell a story during the talk. So the slides may not be useful, post-hoc.

### Attribution

* see [Attribution.md](./Attribution.md)

### Cutting-room floor

Here are some thoughts that didn't make it to the presentation:

* The Rust community has a [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct#:~:text=We%20are%20committed%20to%20providing,nationality%2C%20or%20other%20similar%20characteristic.). This isn't just for a conference: it covers all aspects of the community.
* Analogy:
    - Dealing with memory management (e.g. C/C++) is like driving a stick-shift in a car. It is extra cognitive load, orthogonal to the goal (i.e. navigating to a destination).
    - Garbage-collected languages (e.g. Java/C#/Go) are like driving with an automatic transmission: easy and we don't give it much thought.
    - Rust is like a semi-automatic paddle-shifter on a sports-car's steering wheel. No clutch: just flick the paddles.
        - Stick-shift drivers immediately "get it" and appreciate that it is less work.
        - Automatic-trans drivers might be aghast: "what do you mean, you need to pay attention to what gear you're in?". Rather than seeing the benefits, the additional cognitive load is heavy and jarring.
* Scott Hanselman commented that `drop()` is reminiscent of AOP. [here](https://hanselminutes.com/713/rust-a-language-for-the-next-40-years-with-carol-nichols) on Hanselminutes
* I've observed a phenomenon where, after being sufficiently 'beaten up' by the Borrow Checker, that I disengage thought and am just hammering out code to try and placate it.
    - aka "Borrow Checker, take the wheel"
* More notes on "minimal runtime":
    - The minimal runtime implies ultra-fast startup time. Compare with both JVM and Java frameworks, which battle the issue of startup time.
    - No tuning for garbage collection! 
    - It isn't a surprise, but simple Rust console programs are startlingly fast. Rust reminds us just how powerful today's computers are.

### Examples from presentation

* see [src](./src) for code examples
    - ownership e.g. 1 [here](./src/rust/cats_3_ownership_1)
    - ownership e.g. 2 [here](./src/rust/cats_3_ownership_2)
    - borrow e.g. 1 [here](./src/rust/cats_4_borrow_1))
    - borrow e.g. 2 [here](./src/rust/cats_4_borrow_2))
    - C++ dangling pointer [here](./src/c%2B%2B/cats_5_dangling_pointer)
        - compared to Rust [here](./src/rust/cats_5_compared_to_cpp)
    - TypeScript interface [here](./src/typescript)
        - compared to Rust [here](./src/rust/cats_6_lifetimes)
    - Rust parallel with Rayon [here](./src/rust/cats_7_rayon)

### Usage in production

* [here](https://serokell.io/blog/rust-companies)
* adoption notes on [Wikipedia](https://en.wikipedia.org/wiki/Rust_(programming_language)#Adoption)
* [here](https://changelog.com/podcast/takeover-codeish-34) near 18m55s mark
    - Mozilla Firefox
    - Amazon Firecracker Micro VM
    - [Google Fuschia OS](https://en.wikipedia.org/wiki/Google_Fuchsia) 
    - Facebook Mercurial server
    - Facebook blockchain for Libra
    - research for Ember framework (diff-algorithm in engine)

### Modest projects (as examples)

* [Factorial](https://github.com/codetojoy/Factorial_Rust)
* [WarO](https://github.com/codetojoy/WarO_Rust) 

### Resources

* free book: [The Rust Programming Language](https://doc.rust-lang.org/book/)
* [Rust Playground](https://play.rust-lang.org/)
* [Rust Crate Registry](https://crates.io/) (like `npm` or Maven Central)
* [Rust Foundation](https://foundation.rust-lang.org/posts/2021-02-08-hello-world/) announced
* [Rust Foundation concept](https://blog.rust-lang.org/2020/08/18/laying-the-foundation-for-rusts-future.html) (addresses Mozilla layoffs)
* video
    - ["Rust: A Language for the Next 40 Years"](https://www.youtube.com/watch?v=A3AdN7U24iU) by [Carol Nichols](https://twitter.com/Carols10cents)
    - ["Rust, WebAssembly, and the future of Serverless"](https://www.youtube.com/watch?v=CMB6AlE1QuI) by [Steve Klabnik](https://twitter.com/steveklabnik)
    - ["A Case for Oxidation: The Rust Programming Language"](https://www.youtube.com/watch?v=cDFSrVhnZKo) by [Sergio Benitez](https://github.com/SergioBenitez)
* blog post: ["What is Rust and why is it so popular?"](https://stackoverflow.blog/2020/01/20/what-is-rust-and-why-is-it-so-popular/), Stack Overflow 
    - ["Consider Rust"](https://www.youtube.com/watch?v=DnT-LUQgc7s) by [Jon Gjengset](https://www.youtube.com/channel/UC_iD0xppBwwsrM9DegC5cQQ)
* podcast
    - episode: [Intro to Rust programming](https://changelog.com/podcast/takeover-codeish-34) on The Changelog
    - episode: [Rust: A language for the next 40 years](https://hanselminutes.com/713/rust-a-language-for-the-next-40-years-with-carol-nichols) on Hanselminutes
    - series (inactive): [New Rustacean](https://newrustacean.com/)
