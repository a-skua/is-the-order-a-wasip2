#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::cli::run::Guest;
use bindings::wasi::cli::stdout::get_stdout;
use bindings::wasi::random::random::get_random_u64;

struct Component;

impl Guest for Component {
    fn run() -> Result<(), ()> {
        let random = get_random_u64().to_string() + "\n";

        let stdout = get_stdout();
        stdout.blocking_write_and_flush(random.as_bytes()).unwrap();
        Ok(())
    }
}

bindings::export!(Component with_types_in bindings);
