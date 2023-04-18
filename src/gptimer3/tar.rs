#[doc = "Register `TAR` reader"]
pub struct R(crate::R<TAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TAR` reader - GPTM Timer A register"]
pub type TAR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPTM Timer A register"]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(self.bits)
    }
}
#[doc = "GPTM Timer A This register shows the current value of the Timer A counter. When a GPTM is configured to one of the 32-bit modes, TAR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B (TBR) register). In the16-bit Input edge count, input edge time, and PWM modes, bits \\[15:0\\]
contain the value of the counter and bits 23:16 contain the value of the prescaler, which is the upper 8 bits of the count. Bits \\[31:24\\]
always read as 0. To read the value of the prescaler in 16-bit, one-shot and periodic modes, read bits \\[23:16\\]
in the TAV register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tar](index.html) module"]
pub struct TAR_SPEC;
impl crate::RegisterSpec for TAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tar::R](R) reader structure"]
impl crate::Readable for TAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TAR to value 0"]
impl crate::Resettable for TAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
