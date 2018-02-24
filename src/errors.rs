error_chain! {
    foreign_links {
        IO(::std::io::Error);
        WS(::reqwest::Error);
        Json(::serde_json::Error);
    }
}
