use syn::{Expr, Result};

fn run() -> Result<()> {
    let code = "assert_eq!(u8::max_value(), 255)";
    let expr = syn::parse_str::<Expr>(code)?;
    let tokens = quote::quote! { #expr };
    println!("{}", tokens);
    Ok(())
}

fn main() {
    run().unwrap();
}
