
fn main() {
    println!("Format conversion between *.brep & *.step files.");
    println!("Synopsis:");
    println!("\tbrepconf [-f (step | brep)] <source> [-o <dest>]");
    println!("");
    println!("Options:");
    println!("\t-f (step | brep)");
    println!("\t\tSpecifiy the input format. Only required if file extension isnt '.step' or '.brep'");
    println!("\t-o <dest>");
    println!("\t\tOutput the resulting file in path <dest>.");
    println!("\t\tIf omitted, brepconv will append the proper file extension.");
}
