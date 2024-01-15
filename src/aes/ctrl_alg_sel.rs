#[doc = "Register `CTRL_ALG_SEL` reader"]
pub type R = crate::R<CTRL_ALG_SEL_SPEC>;
#[doc = "Register `CTRL_ALG_SEL` writer"]
pub type W = crate::W<CTRL_ALG_SEL_SPEC>;
#[doc = "Field `KEYSTORE` reader - If set to one, selects the Key Store as destination for the DMA The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)"]
pub type KEYSTORE_R = crate::BitReader;
#[doc = "Field `KEYSTORE` writer - If set to one, selects the Key Store as destination for the DMA The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)"]
pub type KEYSTORE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES` reader - If set to one, selects the AES engine as source/destination for the DMA The read and write maximum transfer size to the DMA engine is set to 16 bytes."]
pub type AES_R = crate::BitReader;
#[doc = "Field `AES` writer - If set to one, selects the AES engine as source/destination for the DMA The read and write maximum transfer size to the DMA engine is set to 16 bytes."]
pub type AES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASH` reader - If set to one, selects the hash engine as destination for the DMA The maximum transfer size to DMA engine is set to 64 bytes for reading and 32 bytes for writing (the latter is only applicable if the hash result is written out through the DMA)."]
pub type HASH_R = crate::BitReader;
#[doc = "Field `HASH` writer - If set to one, selects the hash engine as destination for the DMA The maximum transfer size to DMA engine is set to 64 bytes for reading and 32 bytes for writing (the latter is only applicable if the hash result is written out through the DMA)."]
pub type HASH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAG` reader - If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest). For SHA-256 operation, a DMA must be set up for both input data and TAG. For any other selected module, setting this bit only allows a DMA that reads the TAG. No data allowed to be transferred to or from the selected module via the DMA."]
pub type TAG_R = crate::BitReader;
#[doc = "Field `TAG` writer - If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest). For SHA-256 operation, a DMA must be set up for both input data and TAG. For any other selected module, setting this bit only allows a DMA that reads the TAG. No data allowed to be transferred to or from the selected module via the DMA."]
pub type TAG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - If set to one, selects the Key Store as destination for the DMA The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)"]
    #[inline(always)]
    pub fn keystore(&self) -> KEYSTORE_R {
        KEYSTORE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If set to one, selects the AES engine as source/destination for the DMA The read and write maximum transfer size to the DMA engine is set to 16 bytes."]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If set to one, selects the hash engine as destination for the DMA The maximum transfer size to DMA engine is set to 64 bytes for reading and 32 bytes for writing (the latter is only applicable if the hash result is written out through the DMA)."]
    #[inline(always)]
    pub fn hash(&self) -> HASH_R {
        HASH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 31 - If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest). For SHA-256 operation, a DMA must be set up for both input data and TAG. For any other selected module, setting this bit only allows a DMA that reads the TAG. No data allowed to be transferred to or from the selected module via the DMA."]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If set to one, selects the Key Store as destination for the DMA The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)"]
    #[inline(always)]
    #[must_use]
    pub fn keystore(&mut self) -> KEYSTORE_W<CTRL_ALG_SEL_SPEC> {
        KEYSTORE_W::new(self, 0)
    }
    #[doc = "Bit 1 - If set to one, selects the AES engine as source/destination for the DMA The read and write maximum transfer size to the DMA engine is set to 16 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn aes(&mut self) -> AES_W<CTRL_ALG_SEL_SPEC> {
        AES_W::new(self, 1)
    }
    #[doc = "Bit 2 - If set to one, selects the hash engine as destination for the DMA The maximum transfer size to DMA engine is set to 64 bytes for reading and 32 bytes for writing (the latter is only applicable if the hash result is written out through the DMA)."]
    #[inline(always)]
    #[must_use]
    pub fn hash(&mut self) -> HASH_W<CTRL_ALG_SEL_SPEC> {
        HASH_W::new(self, 2)
    }
    #[doc = "Bit 31 - If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest). For SHA-256 operation, a DMA must be set up for both input data and TAG. For any other selected module, setting this bit only allows a DMA that reads the TAG. No data allowed to be transferred to or from the selected module via the DMA."]
    #[inline(always)]
    #[must_use]
    pub fn tag(&mut self) -> TAG_W<CTRL_ALG_SEL_SPEC> {
        TAG_W::new(self, 31)
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
#[doc = "Algorithm select This algorithm selection register configures the internal destination of the DMA controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_alg_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_alg_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_ALG_SEL_SPEC;
impl crate::RegisterSpec for CTRL_ALG_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_alg_sel::R`](R) reader structure"]
impl crate::Readable for CTRL_ALG_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl_alg_sel::W`](W) writer structure"]
impl crate::Writable for CTRL_ALG_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL_ALG_SEL to value 0"]
impl crate::Resettable for CTRL_ALG_SEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
