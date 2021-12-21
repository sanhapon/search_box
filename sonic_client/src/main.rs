use sonic_channel::*;
use std::{fs, io};

fn main() -> result::Result<()> {

    let push_channel = IngestChannel::start("localhost:1491", "SecretPassword")?;
    let _ = read_and_push(push_channel, ".\\pos").unwrap();


    let channel = SearchChannel::start("localhost:1491", "SecretPassword")?;
    let _ = search(channel, "horrific");

    Ok(())
}

fn search(search_channel: SearchChannel, key_word: &str) -> io::Result<()> {

    let objs = search_channel.query("collection", "bucket", key_word).unwrap();

    for item in objs {
        let v: Vec<&str>  = item.split(":").collect();
        let file_name : &str = v.get(1).unwrap();
        let flie_path = format!(".\\{}.txt", file_name);
        let content = fs::read_to_string(flie_path).unwrap();
        println!("{}", content);
        println!();
    }

    Ok(())
}


fn read_and_push(ingest_channel: IngestChannel, the_path: &str) -> io::Result<()> {

    for entry in fs::read_dir(the_path)? {
        let path = entry?.path();
        let file_path = path.to_str().unwrap();
        let v: Vec<&str> = file_path.split(".").collect();
        let file_name : &str = v.get(1).unwrap();

        let content = fs::read_to_string(file_path).unwrap();

        let a = ingest_channel.push("collection", "bucket", &format!("id:{}", file_name), &content);
        println!("{} {:?}", file_name, a);
    }

    Ok(())
}
