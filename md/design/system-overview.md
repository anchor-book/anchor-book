# System Overview

The repository will contain both the anchor-book engine and its own Anchor Book.
mdBook provides the Book's structure and rendering. The engine will supply the
capabilities for maintaining core content and checking anchor relationships and
verification evidence.

The engine must compile independently of a successful Book build. Once compiled,
it can manage and check this Book through the same interfaces it provides to
other projects. Engine development can continue even when the Book needs repair.

The current Book establishes the initial structure. The engine and its detailed
interfaces have not yet been implemented.
