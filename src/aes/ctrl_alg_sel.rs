#[doc = "Register `CTRL_ALG_SEL` reader"]
pub type R = crate::R<CtrlAlgSelSpec>;
#[doc = "Register `CTRL_ALG_SEL` writer"]
pub type W = crate::W<CtrlAlgSelSpec>;
#[doc = "Field `KEYSTORE` reader - If set to one, selects the Key Store as destination for the DMA The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)"]
pub type KeystoreR = crate::BitReader;
#[doc = "Field `KEYSTORE` writer - If set to one, selects the Key Store as destination for the DMA The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)"]
pub type KeystoreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES` reader - If set to one, selects the AES engine as source/destination for the DMA The read and write maximum transfer size to the DMA engine is set to 16 bytes."]
pub type AesR = crate::BitReader;
#[doc = "Field `AES` writer - If set to one, selects the AES engine as source/destination for the DMA The read and write maximum transfer size to the DMA engine is set to 16 bytes."]
pub type AesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASH` reader - If set to one, selects the hash engine as destination for the DMA The maximum transfer size to DMA engine is set to 64 bytes for reading and 32 bytes for writing (the latter is only applicable if the hash result is written out through the DMA)."]
pub type HashR = crate::BitReader;
#[doc = "Field `HASH` writer - If set to one, selects the hash engine as destination for the DMA The maximum transfer size to DMA engine is set to 64 bytes for reading and 32 bytes for writing (the latter is only applicable if the hash result is written out through the DMA)."]
pub type HashW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAG` reader - If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest). For SHA-256 operation, a DMA must be set up for both input data and TAG. For any other selected module, setting this bit only allows a DMA that reads the TAG. No data allowed to be transferred to or from the selected module via the DMA."]
pub type TagR = crate::BitReader;
#[doc = "Field `TAG` writer - If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest). For SHA-256 operation, a DMA must be set up for both input data and TAG. For any other selected module, setting this bit only allows a DMA that reads the TAG. No data allowed to be transferred to or from the selected module via the DMA."]
pub type TagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - If set to one, selects the Key Store as destination for the DMA The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)"]
    #[inline(always)]
    pub fn keystore(&self) -> KeystoreR {
        KeystoreR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If set to one, selects the AES engine as source/destination for the DMA The read and write maximum transfer size to the DMA engine is set to 16 bytes."]
    #[inline(always)]
    pub fn aes(&self) -> AesR {
        AesR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If set to one, selects the hash engine as destination for the DMA The maximum transfer size to DMA engine is set to 64 bytes for reading and 32 bytes for writing (the latter is only applicable if the hash result is written out through the DMA)."]
    #[inline(always)]
    pub fn hash(&self) -> HashR {
        HashR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 31 - If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest). For SHA-256 operation, a DMA must be set up for both input data and TAG. For any other selected module, setting this bit only allows a DMA that reads the TAG. No data allowed to be transferred to or from the selected module via the DMA."]
    #[inline(always)]
    pub fn tag(&self) -> TagR {
        TagR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If set to one, selects the Key Store as destination for the DMA The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)"]
    #[inline(always)]
    pub fn keystore(&mut self) -> KeystoreW<CtrlAlgSelSpec> {
        KeystoreW::new(self, 0)
    }
    #[doc = "Bit 1 - If set to one, selects the AES engine as source/destination for the DMA The read and write maximum transfer size to the DMA engine is set to 16 bytes."]
    #[inline(always)]
    pub fn aes(&mut self) -> AesW<CtrlAlgSelSpec> {
        AesW::new(self, 1)
    }
    #[doc = "Bit 2 - If set to one, selects the hash engine as destination for the DMA The maximum transfer size to DMA engine is set to 64 bytes for reading and 32 bytes for writing (the latter is only applicable if the hash result is written out through the DMA)."]
    #[inline(always)]
    pub fn hash(&mut self) -> HashW<CtrlAlgSelSpec> {
        HashW::new(self, 2)
    }
    #[doc = "Bit 31 - If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest). For SHA-256 operation, a DMA must be set up for both input data and TAG. For any other selected module, setting this bit only allows a DMA that reads the TAG. No data allowed to be transferred to or from the selected module via the DMA."]
    #[inline(always)]
    pub fn tag(&mut self) -> TagW<CtrlAlgSelSpec> {
        TagW::new(self, 31)
    }
}
#[doc = "Algorithm select This algorithm selection register configures the internal destination of the DMA controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_alg_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_alg_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlAlgSelSpec;
impl crate::RegisterSpec for CtrlAlgSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_alg_sel::R`](R) reader structure"]
impl crate::Readable for CtrlAlgSelSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl_alg_sel::W`](W) writer structure"]
impl crate::Writable for CtrlAlgSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL_ALG_SEL to value 0"]
impl crate::Resettable for CtrlAlgSelSpec {
    const RESET_VALUE: u32 = 0;
}
