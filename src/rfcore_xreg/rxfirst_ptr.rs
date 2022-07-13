#[doc = "Register `RXFIRST_PTR` reader"]
pub struct R(crate::R<RXFIRST_PTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFIRST_PTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFIRST_PTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFIRST_PTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXFIRST_PTR` reader - RAM address offset of the first byte in the RX FIFO"]
pub type RXFIRST_PTR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - RAM address offset of the first byte in the RX FIFO"]
    #[inline(always)]
    pub fn rxfirst_ptr(&self) -> RXFIRST_PTR_R {
        RXFIRST_PTR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RX FIFO pointer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfirst_ptr](index.html) module"]
pub struct RXFIRST_PTR_SPEC;
impl crate::RegisterSpec for RXFIRST_PTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxfirst_ptr::R](R) reader structure"]
impl crate::Readable for RXFIRST_PTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXFIRST_PTR to value 0"]
impl crate::Resettable for RXFIRST_PTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
