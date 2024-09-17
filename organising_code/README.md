# Managing growing projects
So far our programs have been in one module in one file. As a project grows, code should be organised into multiple modules and then multiple files.

A package can contain multiple binary crates and optionally one library crate

# The module system
- Packages: A bundle of one or more crates that lets you build, test and share crates
- Crates: A tree of modules that produces either a library or executable
- Modules and use: Let you control the organisation, scope and privacy of paths
- Paths: A way of naming an item, such as a struct/function/module

## Packages and crates
A crate is the smallest amount of code the rust compiler will consider at a time. When you run rustc file, the compiler considers that file to be a crate. Crates can contain modules, and modules can be defined in other files.

A crate is either:
- A binary crate: programs which compile to an executable
- A library crate: these don't have a main function and don't compile to an executable. They define functionality intended to be shared with multiple projects.

Crate is used interchangeably with library crate, which itself is used interchangeably with a general library.

The crate root is the source file that makes up the root module of your crate. The compiler starts here.

Note running cargo new organising_code creates a package!
We know it's a package because it has a Cargo.toml

Cargo knows that src/main.rs is the crate root of a binary crate with the same name as the package. Cargo also knowns that src/lib.rs is the crate root of a library crate with the same name as the package.

To have multiple binary crates, put their crate roots in src/bin

## Modules
```
cargo new restaurant --lib
```
