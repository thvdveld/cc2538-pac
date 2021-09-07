#[doc = "Register `SW_REV` reader"]
pub struct R(crate::R<SW_REV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_REV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_REV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_REV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FW_CAPABILITIES` reader - 4-bit binary encoding for the functionality implemented in the firmware. Value 0 indicates basic ModExp with/without CRT. Value 1 adds Modular Inversion, value 2 adds Modular Inversion and ECC operations. Values 3-15 are reserved."]
pub struct FW_CAPABILITIES_R(crate::FieldReader<u8, u8>);
impl FW_CAPABILITIES_R {
    pub(crate) fn new(bits: u8) -> Self {
        FW_CAPABILITIES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FW_CAPABILITIES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAJOR_FW_REVISION` reader - 4-bit binary encoding of the major firmware revision number"]
pub struct MAJOR_FW_REVISION_R(crate::FieldReader<u8, u8>);
impl MAJOR_FW_REVISION_R {
    pub(crate) fn new(bits: u8) -> Self {
        MAJOR_FW_REVISION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAJOR_FW_REVISION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MINOR_FW_REVISION` reader - 4-bit binary encoding of the minor firmware revision number"]
pub struct MINOR_FW_REVISION_R(crate::FieldReader<u8, u8>);
impl MINOR_FW_REVISION_R {
    pub(crate) fn new(bits: u8) -> Self {
        MINOR_FW_REVISION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINOR_FW_REVISION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FW_PATCH_LEVEL` reader - 4-bit binary encoding of the firmware patch level, initial release will carry value zero Patches are used to remove bugs without changing the functionality or interface of a module."]
pub struct FW_PATCH_LEVEL_R(crate::FieldReader<u8, u8>);
impl FW_PATCH_LEVEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FW_PATCH_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FW_PATCH_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 28:31 - 4-bit binary encoding for the functionality implemented in the firmware. Value 0 indicates basic ModExp with/without CRT. Value 1 adds Modular Inversion, value 2 adds Modular Inversion and ECC operations. Values 3-15 are reserved."]
    #[inline(always)]
    pub fn fw_capabilities(&self) -> FW_CAPABILITIES_R {
        FW_CAPABILITIES_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 4-bit binary encoding of the major firmware revision number"]
    #[inline(always)]
    pub fn major_fw_revision(&self) -> MAJOR_FW_REVISION_R {
        MAJOR_FW_REVISION_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 4-bit binary encoding of the minor firmware revision number"]
    #[inline(always)]
    pub fn minor_fw_revision(&self) -> MINOR_FW_REVISION_R {
        MINOR_FW_REVISION_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 4-bit binary encoding of the firmware patch level, initial release will carry value zero Patches are used to remove bugs without changing the functionality or interface of a module."]
    #[inline(always)]
    pub fn fw_patch_level(&self) -> FW_PATCH_LEVEL_R {
        FW_PATCH_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "PKA firmware revision and capabilities register This register allows the host access to the internal firmware revision number of the PKA Engine for software driver matching and diagnostic purposes. This register also contains a field that encodes the capabilities of the embedded firmware. The PKA_SW_REV register is written by the firmware within a few clock cycles after starting up that firmware. The hardware reset value is zero, indicating that the information has not been written yet.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_rev](index.html) module"]
pub struct SW_REV_SPEC;
impl crate::RegisterSpec for SW_REV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_rev::R](R) reader structure"]
impl crate::Readable for SW_REV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SW_REV to value 0"]
impl crate::Resettable for SW_REV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
