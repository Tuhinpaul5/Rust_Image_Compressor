# Rust Image Compressor

### Overview
This project is a simple image compression tool built using the Rust programming language and the [image_compressor] crate. The goal of this tool is to provide an easy-to-use solution for compressing images while maintaining acceptable quality.

## How to Run

Follow these steps to run the Rust Image Compressor:

### Prerequisites

Before you begin, make sure you have Rust installed on your machine. If not, you can install Rust by following the instructions on the official website : [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).


You'll need to install MinGW (*If you don't already have it installed see below*)

### Clone the Repository

Clone this repository to your local machine:
```
git clone https://github.com/Tuhinpaul5/rust_image_compressor.git
```
```
cd rust_image_compressor
```
Copy your images into the **images** folder and then run the program.

```
cargo run
```

This will add a **comps** folder with your compressed images.

hange the value of 
```
use std::path::PathBuf;
use image_compressor::FolderCompressor;
use image_compressor::Factor;

fn main() {
    let origin = PathBuf::from("images"); // original directory path
    let dest = PathBuf::from("comps"); // destination directory path
    let mut comp = FolderCompressor::new(origin, dest);
    comp.set_cal_func(|_, _, _| {   
        return Factor::new(1.0, 0.7);   // 1.0 is the quality, 0.7 is the size_ratio
    });
    comp.set_thread_count(4);   // number of threads
    match comp.compress() {   // compress the folder
        Ok(_) => {},    
        Err(e) => println!("Cannot compress the folder!: {}", e),   // if there is an error, print it
    }
}

```

## Acknowledgments
Special thanks to the creators of the **[image_compressor]** crate for providing a powerful image compression library in Rust.

## For Windows Systems
If you have mingw previously installed on the system and there is an error when running the program 

**Error:Failed to find tool. Is gcc.exe installed?**


This can be solved by adding the "mingw\bin" directory to your PATH variables

#### If the problem continues
Solve this by:

- Going to MSYS2.org https://www.msys2.org

- Download and install `msys2-x86_64-20220603.exe` (or whichever version is current). The installation is pretty straightforward, just take a note on the installation path as you will need to add this to the environment PATH later on.
  
A new program called **"MSYS2"** should've been installed. Open it, and you'll see a terminal window in it. You will run ALL of the commands mentioned below in this terminal. If you close it, open it again.

- Run
```
pacman -Syu
```
  This will update the packages and databases.

- Run (Again - This step may not be needed, but is recommended by MSYS2 in case packages need an additional updates.)
```
pacman -Syu
```
- Run
```
pacman -S --needed base-devel mingw-w64-x86_64-toolchain
```
  You will now see a selection of all the packages. If you don't know which to choose, just press enter on your keyboard and you shall get all of them.

  After that, gcc.exe will be installed in the bin folder of mingw64.

- Add theC:\msys64\mingw64\bin file path to your path environment variable (or Wherever you decided to install it).
  In case you need assistance on adding to the path, check out this link.

  Remember to restart your CMD/Windows Terminal/Power Shell for the environment path to take place.

  Then again [run] the program.

[image_compressor]: https://crates.io/crates/image_compressor
[run]:(#how-to-run)
[check out this link]: https://www.computerhope.com/issues/ch000549.htm
[mingw]: (#for-windows-systems)
