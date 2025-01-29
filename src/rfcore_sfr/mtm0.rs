#[doc = "Register `MTM0` reader"]
pub type R = crate::R<Mtm0Spec>;
#[doc = "Register `MTM0` writer"]
pub type W = crate::W<Mtm0Spec>;
#[doc = "Field `MTM0` reader - Indirectly returns and modifies bits \\[7:0\\]
of an internal register depending on the value of MTMSEL.MTMSEL. When reading the MTM0 register with MTMSEL.MTMSEL set to 000 and MTCTRL.LATCH_MODE set to 0, the timer (MTtim) value is latched. When reading the MTM0 register with MTMSEL.MTMSEL set to 000 and MTCTRL.LATCH_MODE set to 1, the timer (MTtim) and overflow counter (MTovf) values are latched."]
pub type Mtm0R = crate::FieldReader;
#[doc = "Field `MTM0` writer - Indirectly returns and modifies bits \\[7:0\\]
of an internal register depending on the value of MTMSEL.MTMSEL. When reading the MTM0 register with MTMSEL.MTMSEL set to 000 and MTCTRL.LATCH_MODE set to 0, the timer (MTtim) value is latched. When reading the MTM0 register with MTMSEL.MTMSEL set to 000 and MTCTRL.LATCH_MODE set to 1, the timer (MTtim) and overflow counter (MTovf) values are latched."]
pub type Mtm0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indirectly returns and modifies bits \\[7:0\\]
of an internal register depending on the value of MTMSEL.MTMSEL. When reading the MTM0 register with MTMSEL.MTMSEL set to 000 and MTCTRL.LATCH_MODE set to 0, the timer (MTtim) value is latched. When reading the MTM0 register with MTMSEL.MTMSEL set to 000 and MTCTRL.LATCH_MODE set to 1, the timer (MTtim) and overflow counter (MTovf) values are latched."]
    #[inline(always)]
    pub fn mtm0(&self) -> Mtm0R {
        Mtm0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indirectly returns and modifies bits \\[7:0\\]
of an internal register depending on the value of MTMSEL.MTMSEL. When reading the MTM0 register with MTMSEL.MTMSEL set to 000 and MTCTRL.LATCH_MODE set to 0, the timer (MTtim) value is latched. When reading the MTM0 register with MTMSEL.MTMSEL set to 000 and MTCTRL.LATCH_MODE set to 1, the timer (MTtim) and overflow counter (MTovf) values are latched."]
    #[inline(always)]
    pub fn mtm0(&mut self) -> Mtm0W<Mtm0Spec> {
        Mtm0W::new(self, 0)
    }
}
#[doc = "MAC Timer multiplexed register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mtm0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtm0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mtm0Spec;
impl crate::RegisterSpec for Mtm0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtm0::R`](R) reader structure"]
impl crate::Readable for Mtm0Spec {}
#[doc = "`write(|w| ..)` method takes [`mtm0::W`](W) writer structure"]
impl crate::Writable for Mtm0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTM0 to value 0"]
impl crate::Resettable for Mtm0Spec {
    const RESET_VALUE: u32 = 0;
}
