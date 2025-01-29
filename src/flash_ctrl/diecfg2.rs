#[doc = "Register `DIECFG2` reader"]
pub type R = crate::R<Diecfg2Spec>;
#[doc = "Field `PKA_EN` reader - 1: PKA is enabled. 0: PKA is permanently disabled."]
pub type PkaEnR = crate::BitReader;
#[doc = "Field `AES_EN` reader - 1: AES is enabled. 0: AES is permanently disabled."]
pub type AesEnR = crate::BitReader;
#[doc = "Field `RF_CORE_EN` reader - 1: RF_CORE is enabled. 0: RF_CORE is permanently disabled."]
pub type RfCoreEnR = crate::BitReader;
#[doc = "Field `DIE_MINOR_REVISION` reader - Indicates the minor revision (metla layer only) number for the cc2538 0x0 - PG1.0 or PG2.0"]
pub type DieMinorRevisionR = crate::FieldReader;
#[doc = "Field `DIE_MAJOR_REVISION` reader - Indicates the major revision (all layer change) number for the cc2538 0x0 - PG1.0 0x2 - PG2.0"]
pub type DieMajorRevisionR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 1: PKA is enabled. 0: PKA is permanently disabled."]
    #[inline(always)]
    pub fn pka_en(&self) -> PkaEnR {
        PkaEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: AES is enabled. 0: AES is permanently disabled."]
    #[inline(always)]
    pub fn aes_en(&self) -> AesEnR {
        AesEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: RF_CORE is enabled. 0: RF_CORE is permanently disabled."]
    #[inline(always)]
    pub fn rf_core_en(&self) -> RfCoreEnR {
        RfCoreEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Indicates the minor revision (metla layer only) number for the cc2538 0x0 - PG1.0 or PG2.0"]
    #[inline(always)]
    pub fn die_minor_revision(&self) -> DieMinorRevisionR {
        DieMinorRevisionR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Indicates the major revision (all layer change) number for the cc2538 0x0 - PG1.0 0x2 - PG2.0"]
    #[inline(always)]
    pub fn die_major_revision(&self) -> DieMajorRevisionR {
        DieMajorRevisionR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538. The DIE_*_REVISION registers are an exeception to this, as they are hardwired and are not part of the FLASH information page.\n\nYou can [`read`](crate::Reg::read) this register and get [`diecfg2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diecfg2Spec;
impl crate::RegisterSpec for Diecfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diecfg2::R`](R) reader structure"]
impl crate::Readable for Diecfg2Spec {}
#[doc = "`reset()` method sets DIECFG2 to value 0"]
impl crate::Resettable for Diecfg2Spec {
    const RESET_VALUE: u32 = 0;
}
