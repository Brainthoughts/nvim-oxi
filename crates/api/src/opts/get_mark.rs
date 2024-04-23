/// Options passed to [`get_mark()`](crate::get_mark). Currently unused.
#[derive(Clone, Debug, Default, macros::OptsBuilder)]
#[repr(C)]
pub struct GetMarkOpts {
    #[cfg(feature = "neovim-nightly")]
    #[builder(mask)]
    mask: u64,
}

#[cfg(not(feature = "neovim-nightly"))]
impl From<&GetMarkOpts> for types::Dictionary {
    fn from(_: &GetMarkOpts) -> Self {
        Self::new()
    }
}
