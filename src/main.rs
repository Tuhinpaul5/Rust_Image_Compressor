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
