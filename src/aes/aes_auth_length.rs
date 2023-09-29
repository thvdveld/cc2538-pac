#[doc = "Register `AES_AUTH_LENGTH` writer"]
pub type W = crate::W<AES_AUTH_LENGTH_SPEC>;
#[doc = "Field `AUTH_LENGTH` writer - Bits \\[31:0\\]
of the authentication length register store the authentication data length in bytes for combined modes only (GCM or CCM). Supported AAD-lengths for CCM are from 0 to (2^16 - 2^8) bytes. For GCM any value up to (2^32 - 1) bytes can be used. Once processing with this context is started, this length decrements to 0. A write to this register triggers the engine to start using this context for GCM and CCM. For a host read operation, these registers return all-0s."]
pub type AUTH_LENGTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Bits \\[31:0\\]
of the authentication length register store the authentication data length in bytes for combined modes only (GCM or CCM). Supported AAD-lengths for CCM are from 0 to (2^16 - 2^8) bytes. For GCM any value up to (2^32 - 1) bytes can be used. Once processing with this context is started, this length decrements to 0. A write to this register triggers the engine to start using this context for GCM and CCM. For a host read operation, these registers return all-0s."]
    #[inline(always)]
    #[must_use]
    pub fn auth_length(&mut self) -> AUTH_LENGTH_W<AES_AUTH_LENGTH_SPEC, 0> {
        AUTH_LENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Authentication length register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_auth_length::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AES_AUTH_LENGTH_SPEC;
impl crate::RegisterSpec for AES_AUTH_LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aes_auth_length::W`](W) writer structure"]
impl crate::Writable for AES_AUTH_LENGTH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AES_AUTH_LENGTH to value 0"]
impl crate::Resettable for AES_AUTH_LENGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
