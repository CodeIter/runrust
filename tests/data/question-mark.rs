use std::fs::File;

File::open("__runrust-this-file-does-not-exist.txt")?;
