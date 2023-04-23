# A Rust wrapper around the OpenTriviaQA database
## [https://github.com/uberspot/OpenTriviaQA](https://github.com/uberspot/OpenTriviaQA)

#### This is a wrapper around the OpenTriviaQA database. This library has the questions built into the binary (they are ~8mb), so it's useful where other trivia libraries cannot function (e.g. WASM).

## Code Example
```rust
// Create the context, loading all trivia questions. Here is also where you specify whether or not you would like questions to repeat themselves
let mut ctx = otqa::TriviaContext::new(false);

// Pick a question at random of a specified category. There are twenty to choose from.
let question = ctx.get_question(otqa::Category::Animals, otqa::Seeding::Random);

// If you wanted to pick a specific question (e.g. if you wanted a custom seeding algorithm), the syntax would look like this:
// let question = ctx.get_question(otqa::Category::Animals, otqa::Seeding::Specific(0));
// It will pick the question of that category at that index, applying a modulo should it overflow the amount of questions

// Here are all fields of questions
println!("{}", question.question);
println!("Category: {:?}", question.category);
for answer in question.answers {
    println!("{}", answer);
}
println!("Correct answer: {}", question.correct_answer);
```