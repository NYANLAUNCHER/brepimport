pub static HELP_TEXT: &str = r#"
Format conversion between *.brep & *.step files.
Synopsis:
    brepconf [-f (step | brep)] <source> [-o <dest>]

Options:
    -f (step | brep)
        Specifiy the input format. Only required if file extension isn't '.step' or '.brep'
    -o <dest>
        Output the resulting file in path <dest>.
        If omitted, brepconv will append the proper file extension.
"#;

fn main() {
    print!("{}", HELP_TEXT);
}
