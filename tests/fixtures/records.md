# A tiny independent specification

> **`R-preserve-input`** (accepted): **Preserve input**
> Statement: Reading leaves the supplied source unchanged.
> Source: The reader is observational.

> **`D-pure-reader`** (accepted): **Use an in-memory reader**
> Choice: Accept supplied chapters.
> Requirements: `R-preserve-input`
> Rationale: Separate acquisition from validation.
> Alternatives:
> - Read paths inside the validator.
> Reconsider when: A new adapter needs to acquire input differently.

> **`A-immutable-input`** (accepted): **Inputs do not change**
> Property: Parsing observes the input without changing it.
> Authority: `R-preserve-input`; `D-pure-reader`
> - Criterion `source-comparison` [test]: Compare input before and after validation.
