peg::parser!{
    pub grammar vector_parser() for str {
      rule number() -> u32
        = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }

      pub rule vector() -> Vec<u32>
        = "[" l:(number() ** ",") "]" { l }
    }
}