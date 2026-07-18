# Fragment ambiguity

Plain heading fragment [[Page#Head]] and block fragment [[Page#^blk]]

Multi-hash subpath [[Page#A#B]] and double hash [[Page##H]]

Mixed heading then block [[Page#Head#^blk]]

Empty pieces: [[Page#]] and [[Page#^]] and [[#Head]] and [[#^blk]]

Aliased: [[Page#Head|alias one]] and [[Page#^blk|alias two]]

Caret without hash [[Page^glued]] keeps the caret in the target

Spaces [[Page#head with spaces]] and slash [[Page#h/with/slash]]

Unicode fragment [[Page#标题]] and unicode target [[笔记#Head]]

Case variants [[page#HEAD]] and block case [[Page#^UPPER-1]]

Degenerates: [[]] and [[|alias-only]] and [[#]] and [[#^]]

Embedded forms: ![[Page#A#B]] and ![[Page#^blk|al]]
