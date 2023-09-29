#[doc = "Register `MTM0` reader"]
pub type R = crate::R<MTM0_SPEC>;
#[doc = "Register `MTM0` writer"]
pub type W = crate::W<MTM0_SPEC>;
#[doc = "Field `MTM0` reader - Indirectly returns and modifies bits \\[7:0\\]
of an internal register depending on the value of MTMSEL.MTMSEL. When reading the MTM0 register with MTMSEL.MTMSEL set to 000 and MTCTRL.LATCH_MODE set to 0, the timer (MTtim) value is latched. When reading the MTM0 register with MTMSEL.MTMSEL set to 000 and MTCTRL.LATCH_MODE set to 1, the timer (MTtim) and overflow counter (MTovf) values are latched."]
pub type MTM0_R = crate::FieldReader;
#[doc = "Field `MTM0` writer - Indirectly returns and modifies bits \\[7:0\\]
of an internal register depending on the value of MTMSEL.MTMSEL. When reading the MTM0 register with MTMSEL.MTMSEL set to 000 and MTCTRL.LATCH_MODE set to 0, the timer (MTtim) value is latched. When reading the MTM0 register with MTMSEL.MTMSEL set to 000 and MTCTRL.LATCH_MODE set to 1, the timer (MTtim) and overflow counter (MTovf) values are latched."]
pub type MTM0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Indirectly returns and modifies bits \\[7:0\\]
of an internal register depending on the value of MTMSEL.MTMSEL. When reading the MTM0 register with MTMSEL.MTMSEL set to 000 and MTCTRL.LATCH_MODE set to 0, the timer (MTtim) value is latched. When reading the MTM0 register with MTMSEL.MTMSEL set to 000 and MTCTRL.LATCH_MODE set to 1, the timer (MTtim) and overflow counter (MTovf) values are latched."]
    #[inline(always)]
    pub fn mtm0(&self) -> MTM0_R {
        MTM0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indirectly returns and modifies bits \\[7:0\\]
of an internal register depending on the value of MTMSEL.MTMSEL. When reading the MTM0 register with MTMSEL.MTMSEL set to 000 and MTCTRL.LATCH_MODE set to 0, the timer (MTtim) value is latched. When reading the MTM0 register with MTMSEL.MTMSEL set to 000 and MTCTRL.LATCH_MODE set to 1, the timer (MTtim) and overflow counter (MTovf) values are latched."]
    #[inline(always)]
    #[must_use]
    pub fn mtm0(&mut self) -> MTM0_W<MTM0_SPEC, 0> {
        MTM0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MAC Timer multiplexed register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtm0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtm0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTM0_SPEC;
impl crate::RegisterSpec for MTM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtm0::R`](R) reader structure"]
impl crate::Readable for MTM0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtm0::W`](W) writer structure"]
impl crate::Writable for MTM0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTM0 to value 0"]
impl crate::Resettable for MTM0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
