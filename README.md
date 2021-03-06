## Yarrow Differential Privacy
Yarrow is built around a data representation for a statistical analysis. There are three types of projects:
- Validator: validates that an analysis is differentially private
- Runtime: execute analysis
- Bindings: helpers to create analysis

The runtime and bindings may be written in any language. The core data representation is in protobuf, and the validator is written in Rust. All projects implement protobuf code generation, protobuf serialization/deserialization, communication over FFI, handle distributable packaging, and have at some point compiled cross-platform (more testing needed). All projects communicate via proto definitions from the `prototypes` directory.  

Alternative C++ and Haskell runtimes and validator stubs have been moved to the `architecture-exploration` branch.  


#### Validator
The rust validator compiles to binaries that expose C foreign function interfaces and read/automatically generate code for protobuf. A validator C FFI is described in the wiki.  

#### Runtimes
The Rust runtime uses a package called ndarray, which feels somewhat like writing numpy in Rust.  

#### Bindings
There are two language bindings, one in Python, one in R. Both support building binaries into an installable package.  

The Python package is more developed, with helper classes, syntax sugar for building analyses, and visualizations.  

The R package uses a shim library in C to interface with compiled binaries. There isn't a programmer interface like in Python yet, but there is a pattern for exposing the C FFI in R code, as well as protobuf generation.  

The steps for adding bindings in a new language are essentially:  
1. set up package management  
2. set up dependency management  
3. pack binaries with the given language's tools  
4. protobuf code generation  
5. FFI implementation and protobuf encoding/decoding  
6. write programmer interface  


### Install
1. clone the repository  


    git clone $REPOSITORY_URI

2. Install protobuf compiler from source  
    Mac:  
        - install xcode `sudo xcode-select --install`  
        - install macports https://www.macports.org/install.php  
        - install unix make tools `sudo /opt/local/bin/port install autoconf automake libtool`  
        - continue on with the Ubuntu install directions  
    Ubuntu:  
        - download the "all" Github release. Use version 3.9.x (because of conan)  
          `https://github.com/protocolbuffers/protobuf/releases/download/v3.9.1/protobuf-all-3.9.1.zip`  
        - Follow the Protobuf instructions, starting from `./configure`  
          `https://github.com/protocolbuffers/protobuf/blob/master/src/README.md`  
          NOTE: move to a directory without spaces in the path  
