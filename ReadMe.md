Author
Hodik Maksym

Project: Game Quest Parser (Course Project)
Goal: To create a parser for a custom domain-specific language (DSL) for RPG games.

# Game Quest Parser (GQDL)

**Game Quest Parser** is a robust command-line tool and library written in Rust for parsing the **Game Quest Definition Language (GQDL)**.

It allows game developers to define RPG quests in a simple, human-readable text format and converts them into structured Rust objects. This project demonstrates the implementation of a **Recursive Descent Parser** from scratch, ensuring strict type safety and detailed error reporting.

---

## Features
* **Custom Grammar:** Parses a structured language with nested blocks and key-value properties.
* **Type Safety:** Distinguishes between integers, booleans, strings, and lists.
* **Error Handling:** Provides descriptive errors using `thiserror` (library) and `anyhow` (CLI).
* **CLI Tool:** Includes a command-line interface built with `clap`.

---

## Technical Description

The parsing process transforms raw text into a usable data structure through two main stages:

### 1. Lexical Analysis (Lexer)
The `Lexer` scans the source code character by character and groups them into **Tokens**.
* **Input:** `active: true`
* **Output:** `[Token::Identifier("active"), Token::Colon, Token::True]`

### 2. Syntactic Analysis (Parser)
The `Parser` iterates through the tokens and builds an **Abstract Syntax Tree (AST)** represented by the `Quest` struct. It validates the order of tokens against the defined grammar rules.

**Data Flow Diagram:**
```text
[ Source File ] -> [ Lexer ] -> [ Token Stream ] -> [ Parser ] -> [ Quest Struct ]
                                                        |
                                               (Validates Grammar)
```
Grammar Rules
The parser follows the EBNF (Extended Backus-Naur Form) grammar rules below:

EBNF
```
QUEST_DEF  ::= "quest" (IDENTIFIER | STRING) "{" BODY "}"
BODY       ::= PROPERTY (","? PROPERTY)*
PROPERTY   ::= KEY ":" VALUE
KEY        ::= "reward" | "active" | "step"
VALUE      ::= INTEGER | BOOLEAN | STRING
```
Example Input (test_quest.txt)
```
quest "The Lost Sword" {
    active: true,
    reward: 500,
    step: "Talk to the blacksmith",
    step: "Find the cave entrance",
    step: "Defeat the skeleton king"
}
```
Installation & Usage
Prerequisites
Rust and Cargo (latest stable version)
CLI Commands
1. Parse a Quest File
   Reads the specified file and prints the parsed Rust structure.
   cargo run -- parse --file test_quest.txt
3. Show Credits
   Displays project information and author.
	cargo run -- credits
4. Help
   Displays all available commands and options.
   cargo run -- --help
