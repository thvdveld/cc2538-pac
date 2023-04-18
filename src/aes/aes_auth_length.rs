#[doc = "Register `AES_AUTH_LENGTH` writer"]
pub struct W(crate::W<AES_AUTH_LENGTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AES_AUTH_LENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<AES_AUTH_LENGTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AES_AUTH_LENGTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUTH_LENGTH` writer - Bits \\[31:0\\]
of the authentication length register store the authentication data length in bytes for combined modes only (GCM or CCM). Supported AAD-lengths for CCM are from 0 to (2^16 - 2^8) bytes. For GCM any value up to (2^32 - 1) bytes can be used. Once processing with this context is started, this length decrements to 0. A write to this register triggers the engine to start using this context for GCM and CCM. For a host read operation, these registers return all-0s."]
pub type AUTH_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AES_AUTH_LENGTH_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Bits \\[31:0\\]
of the authentication length register store the authentication data length in bytes for combined modes only (GCM or CCM). Supported AAD-lengths for CCM are from 0 to (2^16 - 2^8) bytes. For GCM any value up to (2^32 - 1) bytes can be used. Once processing with this context is started, this length decrements to 0. A write to this register triggers the engine to start using this context for GCM and CCM. For a host read operation, these registers return all-0s."]
    #[inline(always)]
    #[must_use]
    pub fn auth_length(&mut self) -> AUTH_LENGTH_W<0> {
        AUTH_LENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Authentication length register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_auth_length](index.html) module"]
pub struct AES_AUTH_LENGTH_SPEC;
impl crate::RegisterSpec for AES_AUTH_LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [aes_auth_length::W](W) writer structure"]
impl crate::Writable for AES_AUTH_LENGTH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AES_AUTH_LENGTH to value 0"]
impl crate::Resettable for AES_AUTH_LENGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
