use fizzbuzzlib::*;

fn main() -> std::io::Result<()> {
    write_range(std::io::stdout().lock(), 1..101)
}