use serde_json::{Result, Value, to_string};

fn fetch_data() -> String {
    String::from(
        r#"
            {
                "id": 1,
                "title": "Hello, Rust"
            }
        "#,
    )
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct BlogPost {
    id: u32,
    title: String,
}

fn main() -> anyhow::Result<()> {
    let post: BlogPost = {
        let data = fetch_data();
        serde_json::from_str(data.as_str())?
    };
    println!("deserialized = {:?}", post);

    let post_json: String = serde_json::to_string(&post)?;
    println!("serialized = {:?}", post_json);

    Ok(())
}