#[doc = "Register `RXP1_PTR` reader"]
pub struct R(crate::R<RXP1_PTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXP1_PTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXP1_PTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXP1_PTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXP1_PTR` reader - RAM address offset of the first byte of the first frame in the RX FIFO"]
pub type RXP1_PTR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - RAM address offset of the first byte of the first frame in the RX FIFO"]
    #[inline(always)]
    pub fn rxp1_ptr(&self) -> RXP1_PTR_R {
        RXP1_PTR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RX FIFO pointer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxp1_ptr](index.html) module"]
pub struct RXP1_PTR_SPEC;
impl crate::RegisterSpec for RXP1_PTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxp1_ptr::R](R) reader structure"]
impl crate::Readable for RXP1_PTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXP1_PTR to value 0"]
impl crate::Resettable for RXP1_PTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
