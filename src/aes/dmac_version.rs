#[doc = "Register `DMAC_VERSION` reader"]
pub struct R(crate::R<DMAC_VERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_VERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_VERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_VERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HW_MAJOR_VERSION` reader - Major version number"]
pub struct HW_MAJOR_VERSION_R(crate::FieldReader<u8, u8>);
impl HW_MAJOR_VERSION_R {
    pub(crate) fn new(bits: u8) -> Self {
        HW_MAJOR_VERSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HW_MAJOR_VERSION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HW_MINOR_VERSION` reader - Minor version number"]
pub struct HW_MINOR_VERSION_R(crate::FieldReader<u8, u8>);
impl HW_MINOR_VERSION_R {
    pub(crate) fn new(bits: u8) -> Self {
        HW_MINOR_VERSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HW_MINOR_VERSION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HW_PATCH_LEVEL` reader - Patch level Starts at 0 at first delivery of this version"]
pub struct HW_PATCH_LEVEL_R(crate::FieldReader<u8, u8>);
impl HW_PATCH_LEVEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HW_PATCH_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HW_PATCH_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EIP_NUMBER_COMPL` reader - Bit-by-bit complement of the EIP_NUMBER field bits."]
pub struct EIP_NUMBER_COMPL_R(crate::FieldReader<u8, u8>);
impl EIP_NUMBER_COMPL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EIP_NUMBER_COMPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EIP_NUMBER_COMPL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EIP_NUMBER` reader - Binary encoding of the EIP-number of this DMA controller (209)"]
pub struct EIP_NUMBER_R(crate::FieldReader<u8, u8>);
impl EIP_NUMBER_R {
    pub(crate) fn new(bits: u8) -> Self {
        EIP_NUMBER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EIP_NUMBER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 24:27 - Major version number"]
    #[inline(always)]
    pub fn hw_major_version(&self) -> HW_MAJOR_VERSION_R {
        HW_MAJOR_VERSION_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Minor version number"]
    #[inline(always)]
    pub fn hw_minor_version(&self) -> HW_MINOR_VERSION_R {
        HW_MINOR_VERSION_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Patch level Starts at 0 at first delivery of this version"]
    #[inline(always)]
    pub fn hw_patch_level(&self) -> HW_PATCH_LEVEL_R {
        HW_PATCH_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Bit-by-bit complement of the EIP_NUMBER field bits."]
    #[inline(always)]
    pub fn eip_number_compl(&self) -> EIP_NUMBER_COMPL_R {
        EIP_NUMBER_COMPL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Binary encoding of the EIP-number of this DMA controller (209)"]
    #[inline(always)]
    pub fn eip_number(&self) -> EIP_NUMBER_R {
        EIP_NUMBER_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DMAC version register This register contains an indication (or signature) of the EIP type of this DMAC, as well as the hardware version/patch numbers.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_version](index.html) module"]
pub struct DMAC_VERSION_SPEC;
impl crate::RegisterSpec for DMAC_VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_version::R](R) reader structure"]
impl crate::Readable for DMAC_VERSION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMAC_VERSION to value 0"]
impl crate::Resettable for DMAC_VERSION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
