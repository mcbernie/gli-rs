
pub use self::sampler1d::FSampler1D;
pub use self::sampler1d_array::FSampler1DArray;

mod sampler1d;
mod sampler1d_array;

use crate::ffi::root::gli;

/// Texture coordinate wrapping mode.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Wrap(pub(crate) gli::wrap);

impl Wrap {
    pub const CLAMP_TO_EDGE          : Wrap = Wrap(gli::wrap_WRAP_CLAMP_TO_EDGE         );
    pub const FIRST                  : Wrap = Wrap(gli::wrap_WRAP_FIRST                 );
    pub const CLAMP_TO_BORDER        : Wrap = Wrap(gli::wrap_WRAP_CLAMP_TO_BORDER       );
    pub const REPEAT                 : Wrap = Wrap(gli::wrap_WRAP_REPEAT                );
    pub const MIRROR_REPEAT          : Wrap = Wrap(gli::wrap_WRAP_MIRROR_REPEAT         );
    pub const MIRROR_CLAMP_TO_EDGE   : Wrap = Wrap(gli::wrap_WRAP_MIRROR_CLAMP_TO_EDGE  );
    pub const MIRROR_CLAMP_TO_BORDER : Wrap = Wrap(gli::wrap_WRAP_MIRROR_CLAMP_TO_BORDER);
    pub const LAST                   : Wrap = Wrap(gli::wrap_WRAP_LAST                  );
}

/// Texture filtering mode.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Filter(pub(crate) gli::filter);

impl Filter {
    pub const NONE    : Filter = Filter(gli::filter_FILTER_NONE   );
    pub const NEAREST : Filter = Filter(gli::filter_FILTER_NEAREST);
    pub const FIRST   : Filter = Filter(gli::filter_FILTER_FIRST  );
    pub const LINEAR  : Filter = Filter(gli::filter_FILTER_LINEAR );
    pub const LAST    : Filter = Filter(gli::filter_FILTER_LAST   );
}
