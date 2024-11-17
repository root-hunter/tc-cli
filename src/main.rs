fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let matches = clap::Command::new("tc")
        .author("Antonio Ricciardi <dev.roothunter@gmail.com>")
        .about("Text compressor/decompressor tool")
        .version("0.1.0")
        .subcommands([
            clap::Command::new("compress")
                .short_flag('c')
                .arg(clap::Arg::new("input").short('i'))
                .arg(clap::Arg::new("output").short('o')),
            clap::Command::new("decompress")
                .short_flag('d')
                .arg(clap::Arg::new("input").short('i'))
                .arg(clap::Arg::new("output").short('o')),
        ])
        .arg_required_else_help(true)
        .get_matches();

    match matches.subcommand() {
        Some(("compress", command)) => {
            if command.args_present() {
                let original_path: &String = command.get_one("input").unwrap();
                let compressed_path: &String = command.get_one("output").unwrap();

                if let Ok(_) = tc::compress_file(original_path, compressed_path) {
                    println!("FILE COMPRESSED: {}", compressed_path);
                } else {
                    println!("ERROR TO COMPRESS: {}", original_path);
                }
            }
        }
        Some(("decompress", command)) => {
            if command.args_present() {
                let compressed_path: &String = command.get_one("input").unwrap();
                let decompressed_path: &String = command.get_one("output").unwrap();

                if let Ok(_) = tc::decompress_file(compressed_path, decompressed_path) {
                    println!("FILE DECOMPRESSED: {}", decompressed_path);
                } else {
                    println!("ERROR TO DECOMPRESS: {}", compressed_path);
                }
            }
        }
        _ => {
            println!("NOT VALID PARAM 1");
        }
    }
}
