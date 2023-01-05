use crate::error::{IntoResult, Result};
use mbedtls_sys::*;

define!(
    #[c_ty(sha1_context)]
    struct Sha1;
    const drop: fn(&mut Self) = sha1_free;
    impl<'a> Into<ptr> {}
);

const SHA1_DIGEST_LEN: usize = 20;
type Digest = [u8; SHA1_DIGEST_LEN];

impl Sha1 {
    pub fn new() -> Self {
        let mut inner = sha1_context::default();
        inner.mode = esp_idf_sys::SHA_TYPE_SHA1;
        Self { inner }
    }

    pub fn update(&mut self, data: &[u8]) -> Result<&mut Self> {
        unsafe { sha1_update(self.into(), data.as_ptr(), data.len() as _) }.into_result()?;
        Ok(self)
    }

    pub fn finish_into(&mut self, out: &mut Digest) -> Result<()> {
        unsafe { sha1_finish(self.into(), out.as_mut_ptr()) }.into_result_discard()
    }

    pub fn finish(&mut self) -> Result<Digest> {
        let mut digest = Digest::default();
        self.finish_into(&mut digest)?;
        Ok(digest)
    }
}
