use std::fs::File;

use crate::hashtree::HashTree;

pub fn parse(file: File, mut writer: impl std::io::Write) {
    let mut tree = HashTree::<String>::new();

    write!(writer, "{:#?}", file).unwrap();

    k
}

///////////////////////////////////////////////////////////////////////////////
//                                 Unit Tests                                //
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod parser {
    use super::*;

    #[ignore = "not now"]
    #[test]
    fn test_name() {
        // parse(file, ve);
        assert_eq!(ve, b"whatever");
    }
}
