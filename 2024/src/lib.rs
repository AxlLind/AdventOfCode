use proc_macro::TokenStream;
use syn::{parse_macro_input, Ident, LitInt, ItemFn};

#[proc_macro_attribute]
pub fn main(args: TokenStream, input: TokenStream) -> TokenStream {
    let day = parse_macro_input!(args as LitInt);
    let input_path = format!("../../inputs/{}.in", day.token());

    let mut aoc_solution = parse_macro_input!(input as ItemFn);
    aoc_solution.sig.ident = Ident::new("aoc_solution", aoc_solution.sig.ident.span());

    let tokens = quote::quote! {
        const INPUT: &str = include_str!(#input_path);
        #aoc_solution
        fn main() {
            let now = ::std::time::Instant::now();
            let (p1, p2) = aoc_solution(INPUT.trim_end());
            let elapsed = now.elapsed();
            println!("Part one: {}", p1);
            println!("Part two: {}", p2);
            if elapsed.as_millis() > 0 {
                println!("Time: {}ms", elapsed.as_millis());
            } else {
                println!("Time: {}μs", elapsed.as_micros());
            }
        }
    };
    TokenStream::from(tokens)
}
