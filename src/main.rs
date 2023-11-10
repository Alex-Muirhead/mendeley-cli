use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;

use anyhow;
use httparse::Status;
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use reqwest::blocking as reqwest;
use url::Url;
use webbrowser;

fn main() -> anyhow::Result<()> {
    // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
    // token URL.
    let client = BasicClient::new(
        ClientId::new("16771".to_string()),
        // Don't store the secret here but whatever
        Some(ClientSecret::new("GXyW78A2mGGBsNU6".to_string())),
        AuthUrl::new("https://api.mendeley.com/oauth/authorize".to_string())?,
        Some(TokenUrl::new(
            "https://api.mendeley.com/oauth/token".to_string(),
        )?),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new("http://localhost:5000".to_string())?);

    // Generate a PKCE challenge.
    // let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("all".to_string()))
        .url();

    // This is the URL you should redirect the user to, in order to trigger the
    // authorization process.
    match webbrowser::open(&auth_url.to_string()) {
        Ok(_) => (),
        Err(_) => println!("Could not open new tab, browse to {}", auth_url),
    };

    // Once the user has been redirected to the redirect URL, you'll have access to the
    // authorization code. For security reasons, your code should verify that the `state`
    // parameter returned by the server matches `csrf_state`.

    // A very naive implementation of the redirect server.
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();

    for stream in listener.incoming() {
        // Keep going until we find something that's valid
        let Ok(mut stream) = stream else { continue };

        // let code;
        // let state;

        {
            // let mut request_line = String::new();
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();

            // let buf = BufReader::new(&stream);
            // buf.read_line(&mut request_line)
            //     .expect("Failed to read response");

            let mut headers = [httparse::EMPTY_HEADER; 20];
            let mut req = httparse::Request::new(&mut headers);
            // let res = req.parse(&request_line).unwrap();
            let res = req.parse(&buffer).unwrap().unwrap();
            println!("Response: {:?}", res);
            match req.path {
                Some(ref path) => {
                    println!("Path: {:?}", path);
                }
                None => {}
            }
        }

        // println!("Response line: {}", &request_line);

        //         let redirect_url = request_line.split_whitespace().nth(1).unwrap();
        //         let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

        //         let code_pair = url
        //             .query_pairs()
        //             .find(|&(ref key, _)| key == "code")
        //             .unwrap();

        //         let (_, value) = code_pair;
        //         code = AuthorizationCode::new(value.into_owned());

        //         let state_pair = url
        //             .query_pairs()
        //             .find(|pair| {
        //                 let &(ref key, _) = pair;
        //                 key == "state"
        //             })
        //             .unwrap();

        //         let (_, value) = state_pair;
        //         state = CsrfToken::new(value.into_owned());
        //     }

        //     // Create a crappy web-page for the server response...
        //     let message = "Go back to your terminal :)";
        //     let response = format!(
        //         "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
        //         message.len(),
        //         message
        //     );
        //     stream.write_all(response.as_bytes()).unwrap();

        //     println!("Mendeley returned the following code:\n{}\n", code.secret());
        //     println!(
        //         "Mendeley returned the following state:\n{} (expected `{}`)\n",
        //         state.secret(),
        //         csrf_token.secret()
        //     );

        //     // Exchange the code with a token.
        //     let token_res = client.exchange_code(code).request(http_client);

        //     println!("Mendeley returned the following token:\n{:?}\n", token_res);

        //     if let Ok(token) = &token_res {
        //         let scopes = if let Some(scopes_vec) = token.scopes() {
        //             scopes_vec
        //                 .iter()
        //                 .map(|comma_separated| comma_separated.split(','))
        //                 .flatten()
        //                 .collect::<Vec<_>>()
        //         } else {
        //             Vec::new()
        //         };
        //         println!("Mendeley returned the following scopes:\n{:?}\n", scopes);
        //     }

        //     let client = reqwest::Client::new();
        //     let mut res = client
        //         .get("https://api.mendeley.com/documents")
        //         .bearer_auth(token_res.as_ref().unwrap().access_token().secret())
        //         .send()?;

        //     println!("Result: {:#?}", &res);
        //     let mut body = String::new();
        //     let _ = res.read_to_string(&mut body);

        //     // println!("Body:\n{}", body);

        //     // The server will terminate itself after collecting the first code.
        //     break;
    }

    // Unwrapping token_result will either produce a Token or a RequestTokenError.
    Ok(())
}
