#[doc = "Register `LSS` reader"]
pub struct R(crate::R<LSS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TSS` reader - Timer snap shot This field contains the value of the free-running timer when either the sync edge 5 or the sync edge 1 was detected."]
pub type TSS_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timer snap shot This field contains the value of the free-running timer when either the sync edge 5 or the sync edge 1 was detected."]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "LIN snap shot The LSS register captures the free-running timer value when either the sync edge 1 or the sync edge 5 is detected in LIN mode.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lss](index.html) module"]
pub struct LSS_SPEC;
impl crate::RegisterSpec for LSS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lss::R](R) reader structure"]
impl crate::Readable for LSS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LSS to value 0"]
impl crate::Resettable for LSS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
