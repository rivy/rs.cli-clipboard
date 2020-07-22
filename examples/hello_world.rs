fn main() {
    let mut ctx = cli_clipboard::get_clipboard().unwrap();
    let the_string = "Hello, world!";
    ctx.set_contents(the_string.to_owned()).unwrap();
}
