use mdbook_chm::{context_to_chm, get_context};

fn main() {
    let Some(ctx) = get_context() else {
        eprintln!("Could not get context from stdin. Is this a valid mdbook build?");
        std::process::exit(1);
    };

    let builder = context_to_chm(ctx);
    if let Err(e) = builder.compile() {
        eprintln!("Could not compile CHM: {e}");
        std::process::exit(1);
    }

    println!("CHM compiled successfully.");
    std::process::exit(0);
}
