#[doc = "Register `AES_AUTH_LENGTH` writer"]
pub type W = crate::W<AesAuthLengthSpec>;
#[doc = "Field `AUTH_LENGTH` writer - Bits \\[31:0\\]
of the authentication length register store the authentication data length in bytes for combined modes only (GCM or CCM). Supported AAD-lengths for CCM are from 0 to (2^16 - 2^8) bytes. For GCM any value up to (2^32 - 1) bytes can be used. Once processing with this context is started, this length decrements to 0. A write to this register triggers the engine to start using this context for GCM and CCM. For a host read operation, these registers return all-0s."]
pub type AuthLengthW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Bits \\[31:0\\]
of the authentication length register store the authentication data length in bytes for combined modes only (GCM or CCM). Supported AAD-lengths for CCM are from 0 to (2^16 - 2^8) bytes. For GCM any value up to (2^32 - 1) bytes can be used. Once processing with this context is started, this length decrements to 0. A write to this register triggers the engine to start using this context for GCM and CCM. For a host read operation, these registers return all-0s."]
    #[inline(always)]
    pub fn auth_length(&mut self) -> AuthLengthW<AesAuthLengthSpec> {
        AuthLengthW::new(self, 0)
    }
}
#[doc = "Authentication length register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_auth_length::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesAuthLengthSpec;
impl crate::RegisterSpec for AesAuthLengthSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aes_auth_length::W`](W) writer structure"]
impl crate::Writable for AesAuthLengthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_AUTH_LENGTH to value 0"]
impl crate::Resettable for AesAuthLengthSpec {
    const RESET_VALUE: u32 = 0;
}
