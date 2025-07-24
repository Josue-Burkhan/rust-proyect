# rust-proyect

## Overview

As a software engineer, I'm constantly working with different tools and services, from local Node.js APIs to rendering programs like Blender. Each tool generates logs in its own format, making debugging a fragmented and time-consuming process. This project is an initiative to deepen my understanding of systems programming and create a practical solution to this problem. My goal is to build a high-performance, universal log analysis tool in Rust that can centralize, parse, and provide insights from various log sources in a unified way.

This project serves as a practical exercise in learning Rust, focusing on its performance, safety features, and powerful ecosystem for building command-line applications. The aim is to develop a modular and extensible application that can be easily adapted to new log formats.

My purpose for creating this software is to build a robust, real-world utility that streamlines my development workflow. It's a challenge to handle data parsing, real-time processing, and user-friendly CLI design, and a perfect opportunity to apply and expand my software engineering skills.

[Software Demo Video](https://youtu.be/XIPhGC-cGDU)

## Development Environment

This software was developed in a Windows environment using the following tools:

* **Code Editor:** Visual Studio Code
* **Build Tool & Package Manager:** Cargo (the official Rust package manager)
* **Compiler Toolchain:** The project relies on the MSVC (Microsoft Visual C++) toolchain, installed via the **Build Tools for Visual Studio 2022**, which provides the necessary linker (`link.exe`).

The primary programming language used is **Rust**. Rust was chosen for its strong performance, memory safety guarantees, and excellent ecosystem for building efficient and reliable command-line tools. Key libraries (crates) used include `clap` for parsing command-line arguments, `colored` for terminal output styling, and `regex` for pattern matching in log entries.

## Useful Websites

Here is a list of websites that were helpful during the development of this project:

* [The Rust Programming Language Book](https://doc.rust-lang.org/book/ch01-02-hello-world.html)
* [Clap (CLI Argument Parser) Crate Documentation](https://docs.rs/clap/latest/clap/)
* [Regex Crate Documentation](https://docs.rs/regex/latest/regex/)
* [Colored Crate Documentation](https://docs.rs/colored/latest/colored/)
