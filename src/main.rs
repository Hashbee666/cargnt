use colourizer::StyledString;
use std::{env, fs, fs::File, io::Write};

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        
        let add1x = env::current_dir()?;
        let add1 = add1x.to_string_lossy();

        let add2 = args[1].to_string().to_owned();

        let mut file_path = "".to_owned();

        file_path.push_str(&add1);
        file_path.push_str("/");
        file_path.push_str(&add2);

        println!("Making folder with name {} ", args[1]);

        //Creates Main Directory
        fs::create_dir(&file_path)?;

        //Creates Cargo.toml file in Main Directory
        let mut new_path = "".to_owned();

        new_path.push_str(&file_path);
        new_path.push_str("/Cargo.toml");

        let mut file = File::create(&new_path)?;

        let mut cargo_text = "[package]\nname = \"".to_owned();

        cargo_text.push_str(&add2);
        cargo_text.push_str("\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n# Thanks for using my program <3\n\n[dependencies]");

        file.write_all(cargo_text.as_bytes())?;

        //Creates src folder in Main Directory

        let mut new_path = "".to_owned();

        new_path.push_str(&file_path);
        new_path.push_str("/src");
        fs::create_dir(&new_path)?;

        //Creates main.rs file in /src

        new_path.push_str("/main.rs");

        let mut file = File::create(&new_path)?;
        file.write_all("fn main() {\n    println!(\"Hello world!\");\n}".as_bytes())?;
    }
    else {
        println!("{}The following required arguments were not provided:\n{}\nUSAGE:\n    cargnt <path>", "error: ".rgb("255;0;0"), "    <path>".rgb("0;255;0"));
    }
    Ok(())
}
