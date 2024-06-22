use std::fs;



pub fn create_file(){
    let path = "./data/file01.txt";
    let path2 = "./data/file02.txt";
    let path3 = "./data/file03.txt";
    let text = "Something here";
    let text2 = "I hate you";
    let text3 = "Cohomology";
    _ = std::fs::write(path, text);
    _ = std::fs::write(path2, text2);
    _ = std::fs::write(path3, text3);

}


pub fn test_create_dir(){
    let path = "./db";
    let my_path = std::path::Path::new(path);
    if my_path.exists(){
        println!("Directory already exists");
        return;
    }
    let result = fs::create_dir(path);
    if result.is_ok(){
        println!("Fine");
    }
    else{
        println!("Some problem. {:?}",result.err());
    }
}