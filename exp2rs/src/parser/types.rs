use std::num::NonZeroUsize;

/// 7.1 Simple data types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SimpleDataType {
    /// 8.1.1
    Number,
    /// 8.1.2
    Real,
    /// 8.1.3
    Integer,
    /// 8.1.4
    Logical,
    /// 8.1.5
    Boolen,
    /// 8.1.6
    String_ {
        width: Option<NonZeroUsize>,
        fixed: bool,
    },
    /// 8.1.7
    Binary { width: Option<NonZeroUsize> },
}
