#[doc = "Register `SW_REV` reader"]
pub type R = crate::R<SW_REV_SPEC>;
#[doc = "Field `FW_PATCH_LEVEL` reader - 4-bit binary encoding of the firmware patch level, initial release will carry value zero Patches are used to remove bugs without changing the functionality or interface of a module."]
pub type FW_PATCH_LEVEL_R = crate::FieldReader;
#[doc = "Field `MINOR_FW_REVISION` reader - 4-bit binary encoding of the minor firmware revision number"]
pub type MINOR_FW_REVISION_R = crate::FieldReader;
#[doc = "Field `MAJOR_FW_REVISION` reader - 4-bit binary encoding of the major firmware revision number"]
pub type MAJOR_FW_REVISION_R = crate::FieldReader;
#[doc = "Field `FW_CAPABILITIES` reader - 4-bit binary encoding for the functionality implemented in the firmware. Value 0 indicates basic ModExp with/without CRT. Value 1 adds Modular Inversion, value 2 adds Modular Inversion and ECC operations. Values 3-15 are reserved."]
pub type FW_CAPABILITIES_R = crate::FieldReader;
impl R {
    #[doc = "Bits 16:19 - 4-bit binary encoding of the firmware patch level, initial release will carry value zero Patches are used to remove bugs without changing the functionality or interface of a module."]
    #[inline(always)]
    pub fn fw_patch_level(&self) -> FW_PATCH_LEVEL_R {
        FW_PATCH_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 4-bit binary encoding of the minor firmware revision number"]
    #[inline(always)]
    pub fn minor_fw_revision(&self) -> MINOR_FW_REVISION_R {
        MINOR_FW_REVISION_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 4-bit binary encoding of the major firmware revision number"]
    #[inline(always)]
    pub fn major_fw_revision(&self) -> MAJOR_FW_REVISION_R {
        MAJOR_FW_REVISION_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 4-bit binary encoding for the functionality implemented in the firmware. Value 0 indicates basic ModExp with/without CRT. Value 1 adds Modular Inversion, value 2 adds Modular Inversion and ECC operations. Values 3-15 are reserved."]
    #[inline(always)]
    pub fn fw_capabilities(&self) -> FW_CAPABILITIES_R {
        FW_CAPABILITIES_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "PKA firmware revision and capabilities register This register allows the host access to the internal firmware revision number of the PKA Engine for software driver matching and diagnostic purposes. This register also contains a field that encodes the capabilities of the embedded firmware. The PKA_SW_REV register is written by the firmware within a few clock cycles after starting up that firmware. The hardware reset value is zero, indicating that the information has not been written yet.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_rev::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SW_REV_SPEC;
impl crate::RegisterSpec for SW_REV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_rev::R`](R) reader structure"]
impl crate::Readable for SW_REV_SPEC {}
#[doc = "`reset()` method sets SW_REV to value 0"]
impl crate::Resettable for SW_REV_SPEC {
    const RESET_VALUE: u32 = 0;
}
