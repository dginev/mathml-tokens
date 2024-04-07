# mathml-tokens
Tokenization strategies for Presentation MathML, as plain text training data.

### Status 

This is an early stage experiment in finding a compact token dialect for MathML which makes it
attractive for plain-text training data use in LLMs.

So far using [quick-xml](https://crates.io/crates/quick-xml) to also allow for a WASM build target.

Ideally the repository grows to be more portable, e.g. supporting a native browser DOM walk, 
or a libxml backend, or ...

Equally importantly, a specification on the transformation algorithm should get added once 
the choices made solidify.

### Examples

See the HTML/TXT pairs in [tests/samples/](tests/samples/).