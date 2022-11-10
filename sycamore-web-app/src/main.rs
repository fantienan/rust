use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| view! { cx,
        p { "Hello, World!iii" }
        button(class="btn", on:click=|_| {println!("tttt")}) {"btn"}
    });
}