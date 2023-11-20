mod emulator;
mod frontend;

fn main() {
    dioxus_web::launch(frontend::App);
}
