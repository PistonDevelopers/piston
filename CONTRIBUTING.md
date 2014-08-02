Git commands and the pull request process: https://github.com/bvssvni/rust-empty/issues/31

Post a message [here](https://github.com/PistonDevelopers/piston/issues/70) "I want to contribute to Rust-Graphics" and @Denommus or @bvssvni will add you as a collaborator. When you are a collaborator you can assign issues to yourself.

This project uses the issue tracker heavily. We try to plan as much as possible up front and break the tasks down into 'Easy' tasks. You do not need to know a lot about Rust to work on the project. We use [Rust-Empty](https://github.com/pistondevelopers/rust-empty) which contains a lot of information in the issue tracker for those who knows little about Rust/Terminal/Vim/Github.

Here is how you get started:

1. Fork the repository.
2. Clone your repository to the local hard drive.
3. Open up the Terminal and navigate to the project directory.
4. If you have not installed Rust yet, type `make nightly-install`.
5. Type `make && make test-internal` to build and test the library.

You work against the fork as you would do with personal projects. When you want review of your code, you open up a pull request on Github.

Feel free to open issues, post comments about progress, questions, new ideas, brain storming etc. The label "Information" is used to collect relevant information. You can remove and edit comments as a way of refining ideas in the issue tracker. This is very helpful because many concerns in this project are very complex. Many issues needs to be broken down into new issues before they can be implemented. Please add comments like "can copy code from X" that tells other people how you do it efficiently.

At this stage, algorithms do not have to cover all edge cases. Feel free to write unit tests, but they are not required. Open issues for edge cases or concerns about performance.

If you have an idea how to make collaboration better, open an issue.
