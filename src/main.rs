mod html;
mod stencil;

use html::html::ToHtml;
use std::{fs, io::Result, path::PathBuf};
use stencil::Stencil;
use warp::Filter;

const PORT: u16 = 3000;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("./www/index.html"));

    let assets = warp::path("static").and(warp::fs::dir("./www/static"));

    let posts = warp::path!(String).map(|post_title| {
        let stencils = stencil::list_available_stencils().unwrap();
        let routelist: Vec<(PathBuf, String, String)> = stencils
            .iter()
            .map(|entry| {
                let (route, mut stencil) = Stencil::first_line(entry.path());
                stencil.pop();
                (entry.path(), route, stencil)
            })
            .collect();
        log::info!("Routelist: {:?}", routelist);

        match routelist.iter().find(|(_, route, _)| route == &post_title) {
            Some((path, _, stencil)) => {
                let index = fs::read_to_string(format!("./www/stencils/{}", stencil)).unwrap();
                let content = fs::read_to_string(path).unwrap();
                warp::reply::html(index.replace("{{content}}", &Stencil::from(content).to_html()))
            }
            None => warp::reply::html(String::from("404 Page not found")),
        }
    });
    let routes = warp::get().and(index.or(assets).or(posts));
    log::info!("Starting server on port {}...", PORT);
    warp::serve(routes).run(([0, 0, 0, 0], PORT)).await;

    Ok(())
}
