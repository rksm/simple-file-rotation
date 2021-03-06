# simple-file-rotation

This is an implementation of simple [FileRotation](FileRotation) mechanism using only std.
Given a file like `my.log`, it will copy that file to `my.1.log`, renaming a
potentially pre-existing `my.1.log` to `my.2.log`. It accepts an optional
number of max filesto keep. It will only rotate files when invoked, it will
/not/ watch any files or do any kind of background processing.

```rust
use simple_file_rotation::{FileRotation};
FileRotation::new("my.log")
    .max_old_files(2)
    .rotate()?;
```

Why yet another file rotation library?
- No additional dependencies.
- No features I don't need.
