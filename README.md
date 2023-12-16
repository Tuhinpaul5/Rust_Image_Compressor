# Rust Image Compressor

### Overview
This project is a simple image compression tool built using the Rust programming language and the [image_compressor] crate. The goal of this tool is to provide an easy-to-use solution for compressing images while maintaining acceptable quality.

## How to Run

Follow these steps to run the Rust Image Compressor:

### Prerequisites

Before you begin, make sure you have Rust installed on your machine. If not, you can install Rust by following the instructions on the official website : [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

## For Windows Systems
You'll need to install mingw (See the below for steps)

### Clone the Repository

Clone this repository to your local machine:

git clone https://github.com/Tuhinpaul5/rust_image_compressor.git

```
cd rust_image_compressor
```
Copy your images into the **images** folder and then run the program.

```
cargo run
```

This will add a **comps** folder with your compressed images.

## Acknowledgments
Special thanks to the creators of the **[image_compressor]** crate for providing a powerful image compression library in Rust.

## For Windows Systems

### There might be an error running the program 
**Error:Failed to find tool. Is gcc.exe installed?**

Solve this by:

- Going to MSYS2.org https://www.msys2.org

- Download and install msys2-x86_64-20220603.exe (or whichever version is current). The installation is pretty straightforward, just take a note on the installation path as you will need to add this to the environment PATH later on.
  
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
