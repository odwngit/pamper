# pamper
A super simple rust library to write and read image data to/from `.pam` (Portable Arbitrary Map) files.  
This is very rudimentary, with no support for some features of `.pam` such as black and white tuple types (and even maxval is always set to 255).  
But, this is suitable for my purposes.
# Usage
Example usage:  
```rust
use pamper::save_pam;

fn main() {
    let data: Vec<u8> = vec![
        255, 0, 0,
        0, 255, 0,
        0, 0, 255
    ];

    save_pam("output.pam", 3, 1, 3, false, data);
}
```