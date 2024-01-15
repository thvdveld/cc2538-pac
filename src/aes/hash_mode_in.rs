#[doc = "Register `HASH_MODE_IN` writer"]
pub type W = crate::W<HASH_MODE_IN_SPEC>;
#[doc = "Field `NEW_HASH` writer - When set to 1, it indicates that the hash engine must start processing a new hash session. The HASH_DIGEST_n registers will automatically be loaded with the initial hash algorithm constants of the selected hash algorithm. When this bit is 0 while the hash processing is started, the initial hash algorithm constants are not loaded in the HASH_DIGEST_n registers. The hash engine will start processing with the digest that is currently in its internal HASH_DIGEST_n registers. This bit is automatically cleared when hash processing is started."]
pub type NEW_HASH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHA256_MODE` writer - The host must write this bit with 1 before processing a hash session."]
pub type SHA256_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - When set to 1, it indicates that the hash engine must start processing a new hash session. The HASH_DIGEST_n registers will automatically be loaded with the initial hash algorithm constants of the selected hash algorithm. When this bit is 0 while the hash processing is started, the initial hash algorithm constants are not loaded in the HASH_DIGEST_n registers. The hash engine will start processing with the digest that is currently in its internal HASH_DIGEST_n registers. This bit is automatically cleared when hash processing is started."]
    #[inline(always)]
    #[must_use]
    pub fn new_hash(&mut self) -> NEW_HASH_W<HASH_MODE_IN_SPEC> {
        NEW_HASH_W::new(self, 0)
    }
    #[doc = "Bit 3 - The host must write this bit with 1 before processing a hash session."]
    #[inline(always)]
    #[must_use]
    pub fn sha256_mode(&mut self) -> SHA256_MODE_W<HASH_MODE_IN_SPEC> {
        SHA256_MODE_W::new(self, 3)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Hash mode register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_mode_in::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_MODE_IN_SPEC;
impl crate::RegisterSpec for HASH_MODE_IN_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hash_mode_in::W`](W) writer structure"]
impl crate::Writable for HASH_MODE_IN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_MODE_IN to value 0"]
impl crate::Resettable for HASH_MODE_IN_SPEC {
    const RESET_VALUE: u32 = 0;
}
