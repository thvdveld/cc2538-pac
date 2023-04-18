#[doc = "Register `CPSR` reader"]
pub struct R(crate::R<CPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPSR` writer"]
pub struct W(crate::W<CPSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CPSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPSDVSR` reader - SSI clock prescale divisor (R/W) Reset value: 0x0 This value must be an even number from 2 to 254, depending on the frequency of SSICLK. The LSB always returns zero on reads."]
pub type CPSDVSR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPSDVSR` writer - SSI clock prescale divisor (R/W) Reset value: 0x0 This value must be an even number from 2 to 254, depending on the frequency of SSICLK. The LSB always returns zero on reads."]
pub type CPSDVSR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPSR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - SSI clock prescale divisor (R/W) Reset value: 0x0 This value must be an even number from 2 to 254, depending on the frequency of SSICLK. The LSB always returns zero on reads."]
    #[inline(always)]
    pub fn cpsdvsr(&self) -> CPSDVSR_R {
        CPSDVSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SSI clock prescale divisor (R/W) Reset value: 0x0 This value must be an even number from 2 to 254, depending on the frequency of SSICLK. The LSB always returns zero on reads."]
    #[inline(always)]
    #[must_use]
    pub fn cpsdvsr(&mut self) -> CPSDVSR_W<0> {
        CPSDVSR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The CPSR register specifies the division factor which is used to derive the SSIClk from the system clock. The clock is further divided by a value from 1 to 256, which is 1 + SCR. SCR is programmed in the SSICR0 register. The frequency of the SSIClk is defined by: SSIClk = SysClk / (CPSDVSR x (1 + SCR)) The value programmed into this register must be an even number between 2 and 254. The least-significant bit of the programmed number is hard-coded to zero. If an odd number is written to this register, data read back from this register has the least-significant bit as zero.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpsr](index.html) module"]
pub struct CPSR_SPEC;
impl crate::RegisterSpec for CPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpsr::R](R) reader structure"]
impl crate::Readable for CPSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpsr::W](W) writer structure"]
impl crate::Writable for CPSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPSR to value 0"]
impl crate::Resettable for CPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
