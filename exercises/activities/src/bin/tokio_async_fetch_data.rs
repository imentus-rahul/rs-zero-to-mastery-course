// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let url = "https://en.wikipedia.org/";
//     let text = reqwest::get(url).await?.text().await?;
//     println!("{}", text);

//     Ok(())
// }

// use std::collections::HashMap;
// use reqwest;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let resp = reqwest::get("https://httpbin.org/ip")
//         .await?
//         .json::<HashMap<String, String>>()
//         .await?;
//     println!("{:#?}", resp);
//     Ok(())
// }

// use reqwest::Error;
// use serde::Deserialize;

// #[derive(Deserialize, Debug)]
// struct User {
//     login: String,
//     id: u32,
// }

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     let request_url = format!(
//         "https://www.api.github.com/repos/{owner}/{repo}/stargazers",
//         owner = "rust-lang-nursery",
//         repo = "rust-cookbook"
//     );

//     async fn run() -> Result<(), reqwest::Error> {
//         let body = reqwest::get("https://www.rust-lang.org")
//             .await?
//             .text()
//             .await?;

//         println!("body = {:?}", body);
//         Ok(())
//     }
//     // run().await;

//     println!("{}", request_url);
//     let request_url = "https://www.api.github.com/repos/rust-lang-nursery/rust-cookbook/stargazers".to_owned();
//     let response = reqwest::get(&request_url).await?.text().await?;
//     println!(
//         "🚀 ~ file: tokio_async_fetch_data.rs ~ line 40 ~ fnmain ~ response {:?}",
//         response
//     );

//     // let users: Vec<User> = response.json().await?;
//     // println!("{:?}", users);
//     Ok(())
// }

// use error_chain::error_chain;

// error_chain! {
//     foreign_links {
//         Io(std::io::Error);
//         HttpRequest(reqwest::Error);
//     }
// }

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // this request url is not text, it's json, therefore issue
    let request_url = format!(
        "https://www.api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );
    // let request_url =
    //     "https://www.api.github.com/repos/rust-lang-nursery/rust-cookbook/stargazers".to_owned();

    // let res = reqwest::get(request_url).await?;
    // let res = reqwest::get("https://www.api.github.com/repos/rust-lang-nursery/rust-cookbook/stargazers").await?;
    let res = reqwest::get("http://httpbin.org/get").await?;
    // println!(
    //     "🚀 ~ file: tokio_async_fetch_data.rs ~ line 79 ~ fnmain ~ res {:?}",
    //     res
    // );
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{}", body);
    Ok(())
}
