use sonic_channel::*;

fn main() -> result::Result<()> {
    let push_channel = IngestChannel::start("localhost:1491", "SecretPassword")?;

    let a = push_channel.push("collection", "bucket", "object:{ind:1}", "my best recipe")?;
    dbg!(a);
    let _ = push_channel.push("collection", "bucket", "object:{ind:2}", "my best records")?;
    let _ = push_channel.push("collection", "bucket", "object:{ind:3}", "my best home")?;

    let channel = SearchChannel::start("localhost:1491", "SecretPassword")?;
    let objects = channel.query_with_limit("collection", "bucket", "rec", 5)?;

    dbg!(&objects);

    for item in objects {
        println!("{}", item)
    }


    Ok(())
}
