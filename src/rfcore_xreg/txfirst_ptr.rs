#[doc = "Register `TXFIRST_PTR` reader"]
pub struct R(crate::R<TXFIRST_PTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXFIRST_PTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXFIRST_PTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXFIRST_PTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXFIRST_PTR` reader - RAM address offset of the next byte to be transmitted from the TX FIFO"]
pub struct TXFIRST_PTR_R(crate::FieldReader<u8, u8>);
impl TXFIRST_PTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXFIRST_PTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIRST_PTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - RAM address offset of the next byte to be transmitted from the TX FIFO"]
    #[inline(always)]
    pub fn txfirst_ptr(&self) -> TXFIRST_PTR_R {
        TXFIRST_PTR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "TX FIFO pointer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfirst_ptr](index.html) module"]
pub struct TXFIRST_PTR_SPEC;
impl crate::RegisterSpec for TXFIRST_PTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txfirst_ptr::R](R) reader structure"]
impl crate::Readable for TXFIRST_PTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXFIRST_PTR to value 0"]
impl crate::Resettable for TXFIRST_PTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
