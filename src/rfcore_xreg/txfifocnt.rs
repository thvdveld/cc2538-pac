#[doc = "Register `TXFIFOCNT` reader"]
pub struct R(crate::R<TXFIFOCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXFIFOCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXFIFOCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXFIFOCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXFIFOCNT` reader - Number of bytes in the TX FIFO (unsigned integer)"]
pub struct TXFIFOCNT_R(crate::FieldReader<u8, u8>);
impl TXFIFOCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXFIFOCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFOCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Number of bytes in the TX FIFO (unsigned integer)"]
    #[inline(always)]
    pub fn txfifocnt(&self) -> TXFIFOCNT_R {
        TXFIFOCNT_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Number of bytes in TX FIFO\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfifocnt](index.html) module"]
pub struct TXFIFOCNT_SPEC;
impl crate::RegisterSpec for TXFIFOCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txfifocnt::R](R) reader structure"]
impl crate::Readable for TXFIFOCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXFIFOCNT to value 0"]
impl crate::Resettable for TXFIFOCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
