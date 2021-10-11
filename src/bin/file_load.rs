use fern_ndarray_learn::setup_logger;
use fern_ndarray_learn::file_util::*;
 
fn main() {
    let filepath: String = "data.txt".into();
    let logpath = "output.log";
    setup_logger(logpath);
   
    let res = load_file_2d_vec(filepath.clone(), 2, 3);
    println!("{:#?}", res);

    let res2 = load_file_2d_array(filepath.clone(), 2, 3);
    println!("{:#?}", res2);
}