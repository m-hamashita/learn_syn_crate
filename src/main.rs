use syn::{self, parse_str, visit::Visit};
use std::fs::read_to_string;

struct DependencyVisitor;

impl<'ast> Visit<'ast> for DependencyVisitor {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        println!("Found struct: {}", i.ident);
        // Check dependencies in struct fields
        for field in i.fields.iter() {
            println!("  Field: {}", field.ident.as_ref().unwrap());
            println!("  Type: {}", field.ty);
        }
    }

    fn visit_item_trait(&mut self, i: &'ast syn::ItemTrait) {
        println!("Found trait: {}", i.ident);
        // Check dependencies in supertraits
        for path in i.supertraits.iter() {
            println!("  Inherits from: {}", path.into_token_stream());
        }
    }
}

fn main() {
    let file_content = read_to_string("path/to/your/file.rs").expect("Unable to read the file");
    let ast: syn::File = parse_str(&file_content).expect("Unable to parse the file");

    let mut visitor = DependencyVisitor;
    syn::visit::visit_file(&mut visitor, &ast);
}
