use std::fs;
use std::path::Path;

fn list_files(dir: &str) {
    let paths = fs::read_dir(Path::new(dir)).unwrap(); // read_dir returns a Result<ReadDir> which contains an iterator of DirEntry, the DirEntry contains the metadata of the file which can be used to get the file name
    //this is the example of output of read_dir:
    //Ok(ReadDir { path: "C:\\Users\\user\\Desktop\\Rust\\file_system\\src", inner: Inner { inner: 0x0000000000000000 } })
    for path in paths {
        println!("{:?}", path.unwrap().path().display()); //unwrap the result of the iterator and get the path of the file
    }
}

fn test() {
    let path = ".";
    let path = Path::new(path);
    println!("{:?}", path.parent().unwrap());
}

fn main() {
    let dir = "."; //directory path
    // list_files(dir);
    test();
}
