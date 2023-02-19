mod css;

use css::*;

fn main() {
    println!("{}", C![C::typ::text_white, M![M::lg, C::typ::text_lg]]);
}
