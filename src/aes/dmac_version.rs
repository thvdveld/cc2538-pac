#[doc = "Register `DMAC_VERSION` reader"]
pub type R = crate::R<DmacVersionSpec>;
#[doc = "Field `EIP_NUMBER` reader - Binary encoding of the EIP-number of this DMA controller (209)"]
pub type EipNumberR = crate::FieldReader;
#[doc = "Field `EIP_NUMBER_COMPL` reader - Bit-by-bit complement of the EIP_NUMBER field bits."]
pub type EipNumberComplR = crate::FieldReader;
#[doc = "Field `HW_PATCH_LEVEL` reader - Patch level Starts at 0 at first delivery of this version"]
pub type HwPatchLevelR = crate::FieldReader;
#[doc = "Field `HW_MINOR_VERSION` reader - Minor version number"]
pub type HwMinorVersionR = crate::FieldReader;
#[doc = "Field `HW_MAJOR_VERSION` reader - Major version number"]
pub type HwMajorVersionR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Binary encoding of the EIP-number of this DMA controller (209)"]
    #[inline(always)]
    pub fn eip_number(&self) -> EipNumberR {
        EipNumberR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Bit-by-bit complement of the EIP_NUMBER field bits."]
    #[inline(always)]
    pub fn eip_number_compl(&self) -> EipNumberComplR {
        EipNumberComplR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Patch level Starts at 0 at first delivery of this version"]
    #[inline(always)]
    pub fn hw_patch_level(&self) -> HwPatchLevelR {
        HwPatchLevelR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Minor version number"]
    #[inline(always)]
    pub fn hw_minor_version(&self) -> HwMinorVersionR {
        HwMinorVersionR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Major version number"]
    #[inline(always)]
    pub fn hw_major_version(&self) -> HwMajorVersionR {
        HwMajorVersionR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[doc = "DMAC version register This register contains an indication (or signature) of the EIP type of this DMAC, as well as the hardware version/patch numbers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_version::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacVersionSpec;
impl crate::RegisterSpec for DmacVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_version::R`](R) reader structure"]
impl crate::Readable for DmacVersionSpec {}
#[doc = "`reset()` method sets DMAC_VERSION to value 0"]
impl crate::Resettable for DmacVersionSpec {
    const RESET_VALUE: u32 = 0;
}
