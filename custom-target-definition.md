# How to Define a Custom Target

The rust only includes support for certain chips/instruction sets. Run `rustup target list`—if you don't see your target, do not fear! You can define a custom one as explained briefly here.

First, find a similar target and copy its configuration. The following will print this to your shell.

```sh
rustc +nightly -Z unstable-options --print target-spec-json --target $SOME_SIMILAR_TARGET
```

Edit all the fields that are different, and add more as necessary.

The LLVM docs have useful information on memoery layout
https://llvm.org/docs/LangRef.html#data-layout

**Note**
_For this project, we could not get the GNU linker to work, but LLD (the LLVM linker which is a drop-in replacement for the GNU one) works fine._
