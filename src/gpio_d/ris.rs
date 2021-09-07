#[doc = "Register `RIS` reader"]
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RIS` reader - Reflects the status of interrupts trigger conditions detected on pins (raw, before masking) Bits set: Requirements met by corresponding pins Bits clear: Requirements not met"]
pub struct RIS_R(crate::FieldReader<u8, u8>);
impl RIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Reflects the status of interrupts trigger conditions detected on pins (raw, before masking) Bits set: Requirements met by corresponding pins Bits clear: Requirements not met"]
    #[inline(always)]
    pub fn ris(&self) -> RIS_R {
        RIS_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "The RIS register is the raw interrupt status register. Bits read high in RIS reflect the status of interrupts trigger conditions detected (raw, before masking), indicating that all the requirements are met, before they are finally allowed to trigger by IE. Bits read as 0 indicate that corresponding input pins have not initiated an interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ris::R](R) reader structure"]
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
