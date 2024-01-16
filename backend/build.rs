fn main(){}

// use std::fs;
// use std::env;
// use std::path::Path;

// fn main()  ->  Result<(), std::io::Error> {

//     let files = ["animals.csv", "adjectives.csv"];

//     match env::var("OUT_DIR") {
//         Ok(out_dir) => {
//             println!("Output is: {:?}", out_dir);
//             if out_dir.contains("debug"){
//                 println!("Targetting release!");
//                 create_target_dir(String::from("debug"))?;
//                 for file in files {
//                     copy_file(String::from("debug"), String::from(file))?;
//                 }
//             }
//             else if out_dir.contains("release"){
//                 create_target_dir(String::from("release"))?;
//                 for file in files {
//                     copy_file(String::from("debug"), String::from(file))?;
//                 }            }
        
//         }
//         Err(e) => {
//             panic!("Failed to get OUT_DIR! {:?}", e);
//         }
//     }

//     Ok(())


// }


// fn create_target_dir(target: String) -> Result<(), std::io::Error>{
//     fs::create_dir_all(
//         Path::new("target")
//                 .join(target)
//                 .join("resources"))

// }

// fn copy_file(target: String, file: String) -> Result<u64, std::io::Error>{
//     fs::copy(
//         Path::new("resources")
//                 .join(&file), 
//                 Path::new("target")
//                     .join(target)
//                     .join("resources")
//                     .join(&file))
// }