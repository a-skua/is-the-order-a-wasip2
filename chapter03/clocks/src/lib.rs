#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::cli::run::Guest;
use bindings::wasi::cli::stdout::get_stdout;
use bindings::wasi::clocks::wall_clock::now;

struct Component;

impl Guest for Component {
    fn run() -> Result<(), ()> {
        let current_time = now().seconds.to_string() + "\n";

        let stdout = get_stdout();
        stdout
            .blocking_write_and_flush(current_time.as_bytes())
            .unwrap();
        Ok(())
    }
}

bindings::export!(Component with_types_in bindings);
