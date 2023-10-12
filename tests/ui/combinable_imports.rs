#![feature(register_tool)]
#![register_tool(marker)]
#![allow(unused_imports)]

mod example_1 {
    use std::fs::File;
    use std::vec::Vec;

    use std::collections::BTreeMap;
    #[allow(marker::combinable_imports)]
    use std::collections::BTreeSet;
}

mod example_2 {
    use std::{collections::BTreeMap, collections::BTreeSet};
}

mod example_3 {
    use std::{
        collections::{BTreeMap, BTreeSet},
        fs::File,
        vec::Vec,
    };
}

/// False positive, waiting on <https://github.com/rust-marker/marker/issues/26>
mod example_4 {
    pub use std::fs::File;
    use std::vec::Vec;
}

fn main() {
    let _x = 18;
}
