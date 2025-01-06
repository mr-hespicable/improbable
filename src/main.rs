mod app;

use app::app;

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(app);
}

