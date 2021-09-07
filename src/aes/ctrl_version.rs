#[doc = "Register `CTRL_VERSION` reader"]
pub struct R(crate::R<CTRL_VERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_VERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_VERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_VERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MAJOR_VERSION` reader - Major version number"]
pub struct MAJOR_VERSION_R(crate::FieldReader<u8, u8>);
impl MAJOR_VERSION_R {
    pub(crate) fn new(bits: u8) -> Self {
        MAJOR_VERSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAJOR_VERSION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MINOR_VERSION` reader - Minor version number"]
pub struct MINOR_VERSION_R(crate::FieldReader<u8, u8>);
impl MINOR_VERSION_R {
    pub(crate) fn new(bits: u8) -> Self {
        MINOR_VERSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINOR_VERSION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PATCH_LEVEL` reader - Patch level Starts at 0 at first delivery of this version"]
pub struct PATCH_LEVEL_R(crate::FieldReader<u8, u8>);
impl PATCH_LEVEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PATCH_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PATCH_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EIP_NUMBER_COMPL` reader - These bits simply contain the complement of bits \\[7:0\\]
(0x87), used by a driver to ascertain that the EIP-120t register is indeed read."]
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
#[doc = "Field `EIP_NUMBER` reader - These bits encode the EIP number for the EIP-120t, this field contains the value 120 (decimal) or 0x78."]
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
    pub fn major_version(&self) -> MAJOR_VERSION_R {
        MAJOR_VERSION_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Minor version number"]
    #[inline(always)]
    pub fn minor_version(&self) -> MINOR_VERSION_R {
        MINOR_VERSION_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Patch level Starts at 0 at first delivery of this version"]
    #[inline(always)]
    pub fn patch_level(&self) -> PATCH_LEVEL_R {
        PATCH_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - These bits simply contain the complement of bits \\[7:0\\]
(0x87), used by a driver to ascertain that the EIP-120t register is indeed read."]
    #[inline(always)]
    pub fn eip_number_compl(&self) -> EIP_NUMBER_COMPL_R {
        EIP_NUMBER_COMPL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - These bits encode the EIP number for the EIP-120t, this field contains the value 120 (decimal) or 0x78."]
    #[inline(always)]
    pub fn eip_number(&self) -> EIP_NUMBER_R {
        EIP_NUMBER_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_version](index.html) module"]
pub struct CTRL_VERSION_SPEC;
impl crate::RegisterSpec for CTRL_VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_version::R](R) reader structure"]
impl crate::Readable for CTRL_VERSION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CTRL_VERSION to value 0"]
impl crate::Resettable for CTRL_VERSION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
