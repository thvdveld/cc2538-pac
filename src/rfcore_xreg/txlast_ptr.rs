#[doc = "Register `TXLAST_PTR` reader"]
pub struct R(crate::R<TXLAST_PTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXLAST_PTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXLAST_PTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXLAST_PTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXLAST_PTR` reader - RAM address offset of the last byte + 1 byte of the TX FIFO"]
pub type TXLAST_PTR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - RAM address offset of the last byte + 1 byte of the TX FIFO"]
    #[inline(always)]
    pub fn txlast_ptr(&self) -> TXLAST_PTR_R {
        TXLAST_PTR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "TX FIFO pointer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txlast_ptr](index.html) module"]
pub struct TXLAST_PTR_SPEC;
impl crate::RegisterSpec for TXLAST_PTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txlast_ptr::R](R) reader structure"]
impl crate::Readable for TXLAST_PTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXLAST_PTR to value 0"]
impl crate::Resettable for TXLAST_PTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
