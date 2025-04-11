use mdbook_chm::mdbook::{MdBookChm, context};

fn main() {
    let Some(ctx) = context() else {
        eprintln!("Could not get context from stdin. Is this a valid mdbook build?");
        std::process::exit(1);
    };

    let builder = match ctx.as_chm() {
        Ok(builder) => builder,
        Err(e) => {
            eprintln!("Could not process book: {e}");
            std::process::exit(1);
        }
    };

    if let Err(e) = builder.compile() {
        eprintln!("Error compiling CHM: {e}");
        std::process::exit(1);
    }

    println!("CHM compiled successfully.");
    std::process::exit(0);
}
