#[tokio::main]
async fn main() -> () {
    let args: Vec<String> = std::env::args().collect();
    let out = match args[1].as_ref() {
        "get" => get_posts().await,
        "show" => show_posts().await,
        "create" => create_posts().await,
        _ => "unknown command.",
    };
    println!("{}", out);
}

async fn get_posts() -> &'static str {
    ""
}

async fn show_posts() -> &'static str {
    ""
}

async fn create_posts() -> &'static str {
    ""
}
