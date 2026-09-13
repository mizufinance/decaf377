# Fq and Fr native arithmetic

Fq and Fr use one generated 32-bit implementation each. Their `u32/generated.rs`
files are unmodified outputs of the shared `shieldd-formal/decaf_generate.py`
recipe, using Fiat commit `e0a0a97d201ec1709d9ac11f1dce47c19468bf9e` and its
pinned array-index printer patch. Regenerate in that formal workspace and copy
the corresponding `.work/decaf-fields/rust/{fq,fr}.rs` outputs here unchanged.
The command and field parameters are recorded in each generated file's header.

`u32/fiat.rs` retains the earlier typed arithmetic API as a compatibility
adapter. It contains no separate arithmetic implementation. The high-level
wrappers call the generated implementation directly. The Arkworks-backed
public types retain their existing stored representation and convert their
canonical Montgomery limbs to and from Rust32 without an additional Montgomery
reduction. Both representations use `R = 2^256`.

Raw 32-byte inputs are reduced with a fixed 256-step Horner loop, so every
generated arithmetic operand is reduced. Expert Montgomery constructors still
require canonical residues for arithmetic; the Fq sentinel supports equality
only. Inversion retains its public `Option` behavior and branches on zero.
Constant-time coverage must account for that domain restriction separately.

These implementation and regression checks are not a complete formal
certificate. Native execution correspondence, all reachable API refinements,
compiled traces and consumer/protocol closure remain obligations in the formal
workspace. Fp is a separate field and has its own implementation and coverage.
