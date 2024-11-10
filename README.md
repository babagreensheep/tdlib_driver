# TDLib driver
Rust based driver for [td](https://github.com/tdlib/td)

## Dependancies
The static objects built by the build script still relies on certain dynamically linked dependancies. Current attempts to alter the build script for completely static objects have failed.
- openssh
- zlib
- libc++ (?)

# Goals
- (As far as possible) build [td](https://github.com/tdlib/td) as a static object without installing build dependancies (using docker, but see [Dependancies](#Dependancies)).
- Parse [td's schema](https://github.com/tdlib/td/blob/master/td/generate/scheme/td_api.tl) and wrap methods, objects and classes in rust equivalents (see [Polymorphism](#Polymorphism). 
- Asynchronous RAII wrapper around [td](https://github.com/tdlib/td), including loop to listen for responses from the interfacea. 

# Polymorphism
[td](https://github.com/tdlib/td) uses polymorphism extensively (classes and inheritance) in a manner that cannot be implemented in rust. However it is possible to use rust's `enum` to implement a certain level of polymorphism.
