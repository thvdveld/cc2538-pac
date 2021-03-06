#[doc = "Register `REVISION` reader"]
pub struct R(crate::R<REVISION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REVISION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REVISION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REVISION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MAJOR_HW_REVISION` reader - 4-bit binary encoding of the major hardware revision number"]
pub type MAJOR_HW_REVISION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MINOR_HW_REVISION` reader - 4-bit binary encoding of the minor hardware revision number"]
pub type MINOR_HW_REVISION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HW_PATCH_LEVEL` reader - 4-bit binary encoding of the hardware patch level, initial release will carry value zero Patches are used to remove bugs without changing the functionality or interface of a module."]
pub type HW_PATCH_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPLEMENT_OF_BASIC_EIP_NUMBER` reader - Bit-by-bit logic complement of bits \\[7:0\\], EIP-28 gives 0xE3"]
pub type COMPLEMENT_OF_BASIC_EIP_NUMBER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BASIC_EIP_NUMBER` reader - 8-bit binary encoding of the EIP number, EIP-28 gives 0x1C"]
pub type BASIC_EIP_NUMBER_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 24:27 - 4-bit binary encoding of the major hardware revision number"]
    #[inline(always)]
    pub fn major_hw_revision(&self) -> MAJOR_HW_REVISION_R {
        MAJOR_HW_REVISION_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 4-bit binary encoding of the minor hardware revision number"]
    #[inline(always)]
    pub fn minor_hw_revision(&self) -> MINOR_HW_REVISION_R {
        MINOR_HW_REVISION_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 4-bit binary encoding of the hardware patch level, initial release will carry value zero Patches are used to remove bugs without changing the functionality or interface of a module."]
    #[inline(always)]
    pub fn hw_patch_level(&self) -> HW_PATCH_LEVEL_R {
        HW_PATCH_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Bit-by-bit logic complement of bits \\[7:0\\], EIP-28 gives 0xE3"]
    #[inline(always)]
    pub fn complement_of_basic_eip_number(&self) -> COMPLEMENT_OF_BASIC_EIP_NUMBER_R {
        COMPLEMENT_OF_BASIC_EIP_NUMBER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 8-bit binary encoding of the EIP number, EIP-28 gives 0x1C"]
    #[inline(always)]
    pub fn basic_eip_number(&self) -> BASIC_EIP_NUMBER_R {
        BASIC_EIP_NUMBER_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "PKA hardware revision register This register allows the host access to the hardware revision number of the PKA engine for software driver matching and diagnostic purposes. It is always located at the highest address in the access space of the module and contains an encoding of the EIP number (with its complement as signature) for recognition of the hardware module.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [revision](index.html) module"]
pub struct REVISION_SPEC;
impl crate::RegisterSpec for REVISION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [revision::R](R) reader structure"]
impl crate::Readable for REVISION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets REVISION to value 0"]
impl crate::Resettable for REVISION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
