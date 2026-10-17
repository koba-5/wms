use api::controller::todo::todo_controller;
use api::util::app::App;

#[tokio::main]
async fn main() {
    let app = App::new();

    app.execute(async |r| r.route("/todo", todo_controller()))
        .await;
}
