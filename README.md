# mathml-tokens
Tokenization strategies for Presentation MathML,
as plain-text training data.

### Status 

This is an early stage experiment in finding a compact token dialect for MathML which makes it
attractive for plain-text training data use in LLMs.

So far using [quick-xml](https://crates.io/crates/quick-xml) to also allow for a WASM build target.
Ideally the repository grows to be more portable, e.g. supporting a native browser DOM walk, 
or a libxml backend, or ...

Equally importantly, a specification on the transformation algorithm should get added once 
the choices made solidify.

### Examples

See the HTML/TXT pairs in [tests/samples](tests/samples/).

### MathML Tokens (Proposal, v0.1.0)

There are [30 elements](https://w3c.github.io/mathml-core/#mathml-elements-and-attributes) in MathML Core.
They are tokenized in 4 categories

1. `Tokens`: recurse into elements, do not wrap with start/end outer tokens. Skip over text content.
    - Includes `mrow`, `mstyle`, `mi`, `mn`, `mo`, `ms`, `mtext`, `mpadded`
2. `Skip`: skip entire node and all of its descendants
    - Includes `annotation`, `annotation-xml`, `maction`, `mphantom`
3. `WrappedTokens`: recurse into elements, outer-wrapping a dedicated start/end token pair. Keep text content.
    - Includes `math`,`mroot`,`msqrt`,`merror`,`mtable`,`mtr`
`mtd`
4. `WrappedArgs`: recurse into elements, outer-wrapping a dedicated start/end token pair, ALSO wrapping 
      each argument with the `[arg]` -- `[end_arg]` token pair. Keep text content.
    - Needed when positional argument information influences layout (e.g. which arg is baseline, which arg is script-level)
    - Simplified: if an argument is a single token, omit the wrappers. For example, avoid `[arg] x [end_arg]` and simply use `x`.
    - Incldues `mfrac`, `mover`, `munder`, `munderover`, `msub`, `msup`, `msubsup`, `mmultiscripts`, `mprescripts`
5. `Literal`: replace with literal token, ignoring descendants (if any)
    - `mglyph` is replaced with the literal `glyph`
    - `mspace` replaced with a literal space ` `
    - text content in in `WrappedTokens` and `WrappedArgs` modes is preserved as-is

### Token Map

element | category | outer tokens | note
:-- | :-- | :-- | :-- 
`math` |wrapped tokens| `[math]` -- `[end_math]` | can config on/off
`mroot` |wrapped tokens| `[root]` -- `[end_root]`  |
`msqrt` |wrapped tokens| `[sqrt]` -- `[end_sqrt]` |
`merror` |wrapped tokens| `[error]` -- `[end_error]` |
`mtable` |wrapped tokens| `[table]` -- `[end_table]` |
`mtr` |wrapped tokens| `[tr]` -- `[end_tr]` | 
`mtd` |wrapped tokens| `[td]` -- `[end_td]` |
`mfrac` | wrapped args | `[frac]` -- `[end_frac]` |
`mover` | wrapped args | `[over]` -- `[end_over]` |
`munder` | wrapped args | `[under]` -- `[end_under]` |
`munderover` | wrapped args | `[underover]` -- `[end_underover]` |
`msub` | wrapped args | `[sub]` -- `[end_sub]` |
`msup` | wrapped args | `[sup]` -- `[end_sup]` |
`msubsup` | wrapped args | `[subsup]` -- `[end_subsup]` |
`mmultiscripts` | wrapped args | `[multiscripts]` -- `[end_multiscripts]` |
`mprescripts` | wrapped args | `[prescripts]` -- `[end_prescripts]` |
`semantics` | tokens | |
`mrow` | tokens | |
`mstyle` | tokens | |
`mi` | tokens | |
`mn` | tokens | |
`mo` | tokens | |
`ms` | tokens | |
`mtext` | tokens | |
`mpadded` | tokens | |
`mspace` | literal | ` ` |
`annotation` | skip | |
`annotation-xml` | skip | |
`maction` | skip | | 
`mphantom` | skip | |

### Tokenization details

The above map introduces 34 special tokens for MathML (16 tag starts, 16 tag ends and 2 inner argument start/end).

- When emitting tokens, they are separated by a whitespace, as usual.
- Unicode support is crucial. Full UTF-8 support is relied on, so that we can preserve the differences between `N`, `ℕ` or `𝒩`.
- Trimming whitespace in text nodes is optional, but should be safe. Text nodes are otherwise copied as-is.
- The outer `[math]--[end_math]` wrap is more useful when working within a mixed text+math document context. For single formula tokenization, they should be considered optional.
- The reference implementation in the repository unwraps (or skips) the wider vocabulary of elements in HTML+SVG, in order to test against entire existing documents. The `Unwrap` mode is specific to that use.

### Open Questions

 - Are there attributes worht preserving under tokenizations? Which ones and in what tokenized form?
