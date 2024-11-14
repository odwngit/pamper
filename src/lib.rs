// Reference: https://manpages.ubuntu.com/manpages/trusty/man5/pam.5.html

use std::fs::OpenOptions;
use std::io::Write;
use std::process;

fn depth_to_tupletype(depth: u8, alpha: bool, maxval: u32) -> String {
    let mut inferred_tupletype: String = "".to_owned();
    let mut ref_depth = depth.clone();
    if alpha {
        ref_depth -= 1;
    }
    inferred_tupletype.push_str(match ref_depth {
        1 => match maxval {
            1 => "BLACKANDWHITE",
            _ => "GRAYSCALE"
        },
        3 => "RGB",
        _ => panic!("Tupletype was not inferrable.")
    });
    if alpha {
        inferred_tupletype.push_str("_ALPHA");
    }
    return inferred_tupletype;
}

/*
Saves data in the form of a Vec<u8> to a file specified by path.
Arguments:
- path &str - path where the file is saved e.g. "output.pam"
- width u32 - width of the image
- height u32 - height of the image
- depth u8 - number of bytes per pixel e.g. 4 for RGBA
- alpha bool - true if the image has an alpha channel
- data Vec<u8> - image data
*/
pub fn save_pam(path: &str, width: u32, height: u32, depth: u8, alpha: bool, data: Vec<u8>) {
    let file_open = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path);

    let mut file = match file_open {
        Ok(f) => f,
        Err(_) => {
            process::exit(1);
        }
    };

    let mut header: String = "P7\n".to_owned();

    header.push_str(format!(
        "WIDTH {}\nHEIGHT {}\nDEPTH {}\nMAXVAL {}\nTUPLETYPE {}\nENDHDR\n", 
        width,
        height,
        depth,
        255,
        depth_to_tupletype(depth, alpha, 255)
        ).as_str()
    );

    let mut buffer: Vec<u8> = vec![];
    buffer.append(&mut 
        header
        .as_bytes()
        .to_vec());
    buffer.append(&mut data.clone());

    file.write_all(&buffer).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_data: Vec<u8> = vec![
            255, 0, 0, 255, 255, 0, 0, 255, 0,
            0, 0, 0, 100, 100, 100, 255, 255, 255,
            0, 255, 255, 255, 0, 255, 0, 0, 0
        ];
        save_pam("output.pam", 3, 3, 3, false, test_data);
    }
}