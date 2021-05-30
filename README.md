# SUC
## (Simple) Secure User Credentials

**SUC** offers one thing: A quick way to save and hash user credentials.

### Who is SUC made for?
If your project..
- Has few users
- Can afford loss of data (file corruption)
- Is a hobby project

..then SUC is the right thing for you!

## Features

- Easily open/create a new _SucFile_
- Save and hash a username and password with one method
- Automatically generate salt
- Check if user exists and check if password is correct with one method

### Why use SUC?
- Simple
- Quick to implement
- No database etc. needed
- Comparatively small file size
- No need to think about hashing and salting

### Why NOT to use SUC?
- Developer is a hobbyist
- **No guarantee on actual security**
- One wrong bit in the file has the potential to destroy all of the data and probably panics your program
- File gets loaded into memory completely, therefore no large amounts of users recommended

### Hashing Details
Hashing is done using the **argon2** Rust crate with the default options. The hash is saved to the file
in the [PHC] format. Salt is generated using [OsRng].

### Example
```rust
use suc::sucfile::SucFile;

fn main() {
    let mut sf = SucFile::open("test.suc").unwrap();
    sf.add("Alexander", "4312541").unwrap();
    println!("{}", sf.check("Alexander", "4312541").unwrap());
    sf.remove("Alexander").unwrap();
}
```

   [PHC]: <https://github.com/P-H-C/phc-string-format/blob/master/phc-sf-spec.md#specification>
   [OsRng]: <https://docs.rs/rand/0.5.0/rand/rngs/struct.OsRng.html>
