#[doc = "Register `TAV` reader"]
pub type R = crate::R<TAV_SPEC>;
#[doc = "Register `TAV` writer"]
pub type W = crate::W<TAV_SPEC>;
#[doc = "Field `TAV` reader - GPTM Timer A register"]
pub type TAV_R = crate::FieldReader<u32>;
#[doc = "Field `TAV` writer - GPTM Timer A register"]
pub type TAV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - GPTM Timer A register"]
    #[inline(always)]
    pub fn tav(&self) -> TAV_R {
        TAV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPTM Timer A register"]
    #[inline(always)]
    #[must_use]
    pub fn tav(&mut self) -> TAV_W<TAV_SPEC, 0> {
        TAV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPTM Timer A value When read, this register shows the current, free-running value of Timer A in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry when using the snapshot feature with the periodic operating mode. When written, the value written into this register is loaded into the TAR register on the next clock cycle. When a GPTM is configured to one of the 32-bit modes, TAV appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Value (TBV) register). In a 16-bit mode, bits \\[15:0\\]
contain the value of the counter and bits \\[23:16\\]
contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in input edge count, input edge time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in \\[23:16\\]
is a true prescaler, meaning bits \\[23:16\\]
count down before decrementing the value in bits \\[15:0\\]. The prescaler its \\[31:24\\]
always read as 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tav::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tav::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAV_SPEC;
impl crate::RegisterSpec for TAV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tav::R`](R) reader structure"]
impl crate::Readable for TAV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tav::W`](W) writer structure"]
impl crate::Writable for TAV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAV to value 0"]
impl crate::Resettable for TAV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
