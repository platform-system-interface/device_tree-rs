/// Convenience alias for the [`Result`](core::result::Result) type.
pub type Result<T> = core::result::Result<T, DeviceTreeError>;

pub type SliceReadResult<T> = core::result::Result<T, SliceReadError>;

pub type VecWriteResult = core::result::Result<(), VecWriteError>;

/// An error describe parsing problems when creating device trees.
#[derive(Debug)]
pub enum DeviceTreeError {
    /// The magic number `MAGIC_NUMBER` was not found at the start of the
    /// structure.
    InvalidMagicNumber,

    /// An offset or size found inside the device tree is outside of what was
    /// supplied to `load()`.
    SizeMismatch,

    /// Failed to read data from slice.
    SliceReadError(SliceReadError),

    /// The data format was not as expected at the given position
    ParseError(usize),

    /// While trying to convert a string that was supposed to be ASCII, invalid
    /// utf8 sequences were encounted
    Utf8Error,

    /// The device tree version is not supported by this library.
    VersionNotSupported,

    /// The device tree structure could not be serialized to DTB
    VecWriteError(VecWriteError),

    /// Property could not be parsed
    PropError(PropError),
}

impl From<SliceReadError> for DeviceTreeError {
    fn from(e: SliceReadError) -> DeviceTreeError {
        DeviceTreeError::SliceReadError(e)
    }
}

impl From<VecWriteError> for DeviceTreeError {
    fn from(e: VecWriteError) -> DeviceTreeError {
        DeviceTreeError::VecWriteError(e)
    }
}

impl From<PropError> for DeviceTreeError {
    fn from(e: PropError) -> Self {
        Self::PropError(e)
    }
}

impl From<core::str::Utf8Error> for DeviceTreeError {
    fn from(_: core::str::Utf8Error) -> DeviceTreeError {
        DeviceTreeError::Utf8Error
    }
}

/// Represents property errors.
#[derive(Debug)]
pub enum PropError {
    NotFound,
    Utf8Error,
    Missing0,
    SliceReadError(SliceReadError),
}

impl From<core::str::Utf8Error> for PropError {
    fn from(_: core::str::Utf8Error) -> PropError {
        PropError::Utf8Error
    }
}

impl From<SliceReadError> for PropError {
    fn from(e: SliceReadError) -> PropError {
        PropError::SliceReadError(e)
    }
}

#[derive(Debug)]
pub enum SliceReadError {
    UnexpectedEndOfInput,
}

#[derive(Debug)]
pub enum VecWriteError {
    NonContiguousWrite,
    UnalignedWrite,
}
