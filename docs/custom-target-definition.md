# How to Define a Custom Target

Copy a similar target from:

```
[insert command here]
```

The LLVM docs have useful information on memoery layout
https://llvm.org/docs/LangRef.html#data-layout

The GNU linker was giving me a nasty error, but LLD (the LLVM linker, a drop-in replacement for the GNU one) seems to be working fine
https://lld.llvm.org