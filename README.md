# Words Learner

Word translation game where the user is presented with words in one
language and are asked to provide translation in another language. The
application reads word pairs from a JSON file, presents a random word to
the user, and asks for the translation.

# Prerequisites

-   Cargo. I suggest installing [rustup](https://rustup.rs/) and
    installing stable branch of rust but you can also install plain
    [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

# Quick Start

1.Clone this repo

``` {.bash org-language="sh"}
git clone https://github.com/antonsakhanovych/Words-Learner.git
```

2.Create your words.json file in the format:

``` json
[
    {
        "from": "Hello",
        "to": "Merhaba"
    },
    {
        "from": "How are you?",
        "to": "Nasılsın?"
    }
]
```

3.Compile

``` bash
cargo build --release
```

4.Start the game

``` bash
./target/release/words_learner words.json
```
