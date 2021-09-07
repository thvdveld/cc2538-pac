#[doc = "Register `PP` reader"]
pub struct R(crate::R<PP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NB` reader - 9-bit support 1: The UART module provides support for the transmission of 9-bit data for RS-485 support. 0: The UART module does not provide support for the transmission of 9-bit data for RS-485 support."]
pub struct NB_R(crate::FieldReader<bool, bool>);
impl NB_R {
    pub(crate) fn new(bits: bool) -> Self {
        NB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SC` reader - Smart card support 1: The UART module provides smart card support. 0: The UART module does not provide smart card support."]
pub struct SC_R(crate::FieldReader<bool, bool>);
impl SC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - 9-bit support 1: The UART module provides support for the transmission of 9-bit data for RS-485 support. 0: The UART module does not provide support for the transmission of 9-bit data for RS-485 support."]
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Smart card support 1: The UART module provides smart card support. 0: The UART module does not provide smart card support."]
    #[inline(always)]
    pub fn sc(&self) -> SC_R {
        SC_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "UART peripheral properties The PP register provides information regarding the properties of the UART module.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pp](index.html) module"]
pub struct PP_SPEC;
impl crate::RegisterSpec for PP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pp::R](R) reader structure"]
impl crate::Readable for PP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PP to value 0"]
impl crate::Resettable for PP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
