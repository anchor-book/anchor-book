# A tiny independent specification

```anchorbook-requirement
schema-version = 1
id = "preserve-input"
title = "Preserve input"
status = "accepted"
statement = "Reading leaves the supplied source unchanged."
source = "The reader is observational."
```

```anchorbook-decision
schema-version = 1
id = "pure-reader"
title = "Use an in-memory reader"
status = "accepted"
choice = "Accept supplied chapters."
requirements = ["preserve-input"]
rationale = "Separate acquisition from validation."
alternatives = ["Read paths inside the validator."]
reconsider-when = "A new adapter needs to acquire input differently."
```

```anchorbook-anchor
schema-version = 1
id = "immutable-input"
title = "Inputs do not change"
status = "accepted"
property = "Parsing observes the input without changing it."
authorities = [
  { kind = "requirement", id = "preserve-input" },
  { kind = "decision", id = "pure-reader" },
]

[[criteria]]
id = "source-comparison"
verification = ["test"]
statement = "Compare input before and after validation."
```
