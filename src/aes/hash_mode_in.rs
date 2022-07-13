#[doc = "Register `HASH_MODE_IN` writer"]
pub struct W(crate::W<HASH_MODE_IN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_MODE_IN_SPEC>;
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
impl From<crate::W<HASH_MODE_IN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_MODE_IN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHA256_MODE` writer - The host must write this bit with 1 before processing a hash session."]
pub type SHA256_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASH_MODE_IN_SPEC, bool, O>;
#[doc = "Field `NEW_HASH` writer - When set to 1, it indicates that the hash engine must start processing a new hash session. The HASH_DIGEST_n registers will automatically be loaded with the initial hash algorithm constants of the selected hash algorithm. When this bit is 0 while the hash processing is started, the initial hash algorithm constants are not loaded in the HASH_DIGEST_n registers. The hash engine will start processing with the digest that is currently in its internal HASH_DIGEST_n registers. This bit is automatically cleared when hash processing is started."]
pub type NEW_HASH_W<'a, const O: u8> = crate::BitWriter<'a, u32, HASH_MODE_IN_SPEC, bool, O>;
impl W {
    #[doc = "Bit 3 - The host must write this bit with 1 before processing a hash session."]
    #[inline(always)]
    pub fn sha256_mode(&mut self) -> SHA256_MODE_W<3> {
        SHA256_MODE_W::new(self)
    }
    #[doc = "Bit 0 - When set to 1, it indicates that the hash engine must start processing a new hash session. The HASH_DIGEST_n registers will automatically be loaded with the initial hash algorithm constants of the selected hash algorithm. When this bit is 0 while the hash processing is started, the initial hash algorithm constants are not loaded in the HASH_DIGEST_n registers. The hash engine will start processing with the digest that is currently in its internal HASH_DIGEST_n registers. This bit is automatically cleared when hash processing is started."]
    #[inline(always)]
    pub fn new_hash(&mut self) -> NEW_HASH_W<0> {
        NEW_HASH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hash mode register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_mode_in](index.html) module"]
pub struct HASH_MODE_IN_SPEC;
impl crate::RegisterSpec for HASH_MODE_IN_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hash_mode_in::W](W) writer structure"]
impl crate::Writable for HASH_MODE_IN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HASH_MODE_IN to value 0"]
impl crate::Resettable for HASH_MODE_IN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
