pub mod dom;
pub mod parser;

use parser::Parser;

fn main() {
    let input = "<html>\
                    <head>\
                        <title>Document</title>\
                    </head>\
                    <body>\
                        <h1 id='h1' class='title'>Hello, World!</h1>\
                    </body>\
                </html>";
    let root = Parser::parse(input.to_string());
    dbg!(root);
}
