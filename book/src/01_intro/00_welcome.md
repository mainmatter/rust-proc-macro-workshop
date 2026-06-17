# Welcome

Welcome to our **Rust procedural macros course**!\
You'll learn how to **extend the Rust compiler** by writing your own procedural macros.

> Procedural macros allow you to run code at compile time that operates over Rust syntax,
> both consuming and producing Rust syntax.
> You can sort of think of procedural macros as functions from an AST to another AST.
>
> [The Rust Reference](https://doc.rust-lang.org/reference/procedural-macros.html)

I'll take you on a journey through the Rust macro system, exploring derive macros, attribute macros,
and function-like macros, learning how to parse and generate Rust code using the `syn` and `quote` crates.

## Methodology

This course is based on the "learn by doing" principle.\
It has been designed to be interactive and hands-on.

[Mainmatter](https://mainmatter.com/rust-consulting/) developed this course
to be delivered in a classroom setting, in a single day: each attendee advances
through the lessons at their own pace, with an experienced instructor providing
guidance, answering questions and diving deeper into the topics as needed.\
If you're interested in attending one of our training sessions, or if you'd like to
bring this course to your company, please [get in touch](https://mainmatter.com/contact/).

You can also follow the course on your own, but we recommend you find a friend or
a mentor to help you along the way should you get stuck. You can
also find solutions to all exercises in the
[`solutions` branch of the GitHub repository](https://github.com/mainmatter/rust-proc-macro-workshop/tree/solutions).

## Structure

On the left side of the screen, you can see that the course is divided into sections.
Each section introduces a new concept or feature.\
To verify your understanding, each section is paired with an exercise that you need to solve.

You can find the exercises in the
[companion GitHub repository](https://github.com/mainmatter/rust-proc-macro-workshop).\
Before starting the course, make sure to clone the repository to your local machine:

```bash
# If you have an SSH key set up with GitHub
git clone git@github.com:mainmatter/rust-proc-macro-workshop.git
# Otherwise, use the HTTPS URL:
#
#   git clone https://github.com/mainmatter/rust-proc-macro-workshop.git
```

We also recommend you work on a branch, so you can easily track your progress and pull
in updates from the main repository, if needed:

```bash
cd rust-proc-macro-workshop
git checkout -b my-solutions
```

All exercises are located in the `exercises` folder.
Each exercise is a Rust package, and most are split into two crates: a thin consumer crate and a
`macros` sub-crate that holds the procedural macro you'll be working on.
The instructions on what to do, along with a test suite to automatically verify your solution, live
next to the code you need to edit: usually in `macros/src/lib.rs`, or in `src/lib.rs` for the
simpler single-crate exercises.

### `wr`, the workshop runner

To verify your solutions, we've also provided a tool to guide you through the course: the `wr` CLI, short for "workshop runner".
Install `wr` by following the instructions on [its website](https://mainmatter.github.io/rust-workshop-runner/).

Once you have `wr` installed, open a new terminal and navigate to the top-level folder of the repository.
Run the `wr` command to start the course:

```bash
wr
```

`wr` will verify the solution to the current exercise.\
Don't move on to the next section until you've solved the exercise for the current one.

You can also check a specific exercise without advancing through the course: navigate into its
folder and run `wr check`.

```bash
cd exercises/01_intro/00_welcome/
wr check
```

> We recommend committing your solutions to Git as you progress through the course,
> so you can easily track your progress and "restart" from a known point if needed.

### A note on AI assistants

We strongly recommend that you **disable AI coding assistants** (agents, copilots, autocomplete suggestions)
while working through the exercises.\
The goal of this course is to learn by doing: struggling with the code, reading the documentation, and
building your own mental model of how procedural macros work. Letting an AI fill in the blanks
for you would short-circuit the learning process.

Enjoy the course!

## Author

Authored by [Guillaume Desmottes](mailto:guillaume.desmottes@mainmatter.com) for
[Mainmatter](https://mainmatter.com/rust-consulting/).

Guillaume is part of the Mainmatter Rust team. He maintains various crates, such as
[`system-deps`](https://crates.io/crates/system-deps), and is the creator of
[Karapulse](https://gitlab.freedesktop.org/gdesmott/karapulse), a Rust karaoke application.
