//! Native foundation status and bounded CSV import; domain behavior stays in core.

mod command;
mod config;
mod output;

fn main() -> std::process::ExitCode {
    let outcome = command::run(std::env::args_os().skip(1).collect());
    let code = output::emit(
        &mut std::io::stdout().lock(),
        &mut std::io::stderr().lock(),
        outcome,
    );
    std::process::ExitCode::from(code)
}
