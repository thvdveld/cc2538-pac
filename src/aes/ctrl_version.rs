#[doc = "Register `CTRL_VERSION` reader"]
pub type R = crate::R<CtrlVersionSpec>;
#[doc = "Field `EIP_NUMBER` reader - These bits encode the EIP number for the EIP-120t, this field contains the value 120 (decimal) or 0x78."]
pub type EipNumberR = crate::FieldReader;
#[doc = "Field `EIP_NUMBER_COMPL` reader - These bits simply contain the complement of bits \\[7:0\\]
(0x87), used by a driver to ascertain that the EIP-120t register is indeed read."]
pub type EipNumberComplR = crate::FieldReader;
#[doc = "Field `PATCH_LEVEL` reader - Patch level Starts at 0 at first delivery of this version"]
pub type PatchLevelR = crate::FieldReader;
#[doc = "Field `MINOR_VERSION` reader - Minor version number"]
pub type MinorVersionR = crate::FieldReader;
#[doc = "Field `MAJOR_VERSION` reader - Major version number"]
pub type MajorVersionR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits encode the EIP number for the EIP-120t, this field contains the value 120 (decimal) or 0x78."]
    #[inline(always)]
    pub fn eip_number(&self) -> EipNumberR {
        EipNumberR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - These bits simply contain the complement of bits \\[7:0\\]
(0x87), used by a driver to ascertain that the EIP-120t register is indeed read."]
    #[inline(always)]
    pub fn eip_number_compl(&self) -> EipNumberComplR {
        EipNumberComplR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Patch level Starts at 0 at first delivery of this version"]
    #[inline(always)]
    pub fn patch_level(&self) -> PatchLevelR {
        PatchLevelR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Minor version number"]
    #[inline(always)]
    pub fn minor_version(&self) -> MinorVersionR {
        MinorVersionR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Major version number"]
    #[inline(always)]
    pub fn major_version(&self) -> MajorVersionR {
        MajorVersionR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[doc = "Version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_version::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlVersionSpec;
impl crate::RegisterSpec for CtrlVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_version::R`](R) reader structure"]
impl crate::Readable for CtrlVersionSpec {}
#[doc = "`reset()` method sets CTRL_VERSION to value 0"]
impl crate::Resettable for CtrlVersionSpec {
    const RESET_VALUE: u32 = 0;
}
