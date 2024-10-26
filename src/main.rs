use ivan_markhaichuk_parser::vector_parser;

fn main() {
    let vector = vector_parser::vector("[1,2,3,4]");
    assert_eq!(&vector, &Ok(vec![1,2,3,4]));
}