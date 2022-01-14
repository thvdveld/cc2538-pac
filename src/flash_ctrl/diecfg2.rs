#[doc = "Register `DIECFG2` reader"]
pub struct R(crate::R<DIECFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIECFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIECFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIECFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIE_MAJOR_REVISION` reader - Indicates the major revision (all layer change) number for the cc2538 0x0 - PG1.0 0x2 - PG2.0"]
pub struct DIE_MAJOR_REVISION_R(crate::FieldReader<u8, u8>);
impl DIE_MAJOR_REVISION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIE_MAJOR_REVISION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIE_MAJOR_REVISION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIE_MINOR_REVISION` reader - Indicates the minor revision (metla layer only) number for the cc2538 0x0 - PG1.0 or PG2.0"]
pub struct DIE_MINOR_REVISION_R(crate::FieldReader<u8, u8>);
impl DIE_MINOR_REVISION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIE_MINOR_REVISION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIE_MINOR_REVISION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF_CORE_EN` reader - 1: RF_CORE is enabled. 0: RF_CORE is permanently disabled."]
pub struct RF_CORE_EN_R(crate::FieldReader<bool, bool>);
impl RF_CORE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF_CORE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_CORE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AES_EN` reader - 1: AES is enabled. 0: AES is permanently disabled."]
pub struct AES_EN_R(crate::FieldReader<bool, bool>);
impl AES_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AES_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AES_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKA_EN` reader - 1: PKA is enabled. 0: PKA is permanently disabled."]
pub struct PKA_EN_R(crate::FieldReader<bool, bool>);
impl PKA_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PKA_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKA_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 12:15 - Indicates the major revision (all layer change) number for the cc2538 0x0 - PG1.0 0x2 - PG2.0"]
    #[inline(always)]
    pub fn die_major_revision(&self) -> DIE_MAJOR_REVISION_R {
        DIE_MAJOR_REVISION_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Indicates the minor revision (metla layer only) number for the cc2538 0x0 - PG1.0 or PG2.0"]
    #[inline(always)]
    pub fn die_minor_revision(&self) -> DIE_MINOR_REVISION_R {
        DIE_MINOR_REVISION_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 2 - 1: RF_CORE is enabled. 0: RF_CORE is permanently disabled."]
    #[inline(always)]
    pub fn rf_core_en(&self) -> RF_CORE_EN_R {
        RF_CORE_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1: AES is enabled. 0: AES is permanently disabled."]
    #[inline(always)]
    pub fn aes_en(&self) -> AES_EN_R {
        AES_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 1: PKA is enabled. 0: PKA is permanently disabled."]
    #[inline(always)]
    pub fn pka_en(&self) -> PKA_EN_R {
        PKA_EN_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538. The DIE_*_REVISION registers are an exeception to this, as they are hardwired and are not part of the FLASH information page.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diecfg2](index.html) module"]
pub struct DIECFG2_SPEC;
impl crate::RegisterSpec for DIECFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diecfg2::R](R) reader structure"]
impl crate::Readable for DIECFG2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIECFG2 to value 0"]
impl crate::Resettable for DIECFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
