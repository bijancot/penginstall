use std::fs::File;
use std::io::Write;
use tera::Tera;
use tera::Context;
use std::string;
use clap::{Arg, Command};

fn main() -> std::io::Result<()> {
    // fn main(){
    //get input from user
    // 1. the input consist off:
    // - domain name
    // - directory of code
    // - what framework
    // - where to write the config file (would make default value for this one)
    // 2. once input handled by variables, do the magic
    // 3. read template using Tera, write new file using fs std
    // 4. Done

    let matches = Command::new("Penginstall; nginx config generator based on rust")
        .version("0.0.5b")
        .author("Panji Iman Baskoro <panjidia995@gmail.com>")
        .about("A simple ")
        .help_template("\
        {before-help}{name} {version}
    {author-with-newline}{about-with-newline}
        {usage-heading} {usage}
        
        {all-args}{after-help}
        ")
        .arg(Arg::new("target_dir")
                 .short('t')
                 .long("target-dir")
                 .help("Target directory of the config file"))
        .arg(Arg::new("domain")
                 .short('d')
                 .long("domain")
                 .help("domain name that want to be pointed"))
        .arg(Arg::new("backend")
                 .short('b')
                 .long("backend")
                 .help("the backend name"))
        .arg(Arg::new("filename")
                 .short('f')
                 .long("filename")
                 .help("source template defaulted to default.conf"))
        .get_matches();

    // let target_dir: Option<&String> = matches.get_one("file").unwrap().to_string();
    let target_dir_flag: Option<&String> = matches.get_one("target_dir");
    let target_dir = &target_dir_flag.unwrap().to_string();
    let domain_flag: Option<&String> = matches.get_one("domain");
    let domain = &domain_flag.unwrap().to_string();
    let backend_flag: Option<&String> = matches.get_one("backend");
    let backend = &backend_flag.unwrap().to_string();
    let filename_flag: Option<&String> = matches.get_one("filename");
    let file_name = &filename_flag.unwrap().to_string();
    
    let tera = match Tera::new("templates/**") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let mut context = Context::new();
    context.insert("domain",&domain);
    context.insert("backend", &backend);
    let panji = tera.render("nginx.conf.tpl", &context).unwrap();

    // println!("{}", tera.render("hello", &context).unwrap());
    // let panji = tera.render("hello", &context).unwrap();
    // write result to file
    let new_file = format!("{} {}", &target_dir, &file_name);
    let mut f = File::create(new_file)?;
    f.write_all(&panji.as_bytes())?;
    Ok(())
}