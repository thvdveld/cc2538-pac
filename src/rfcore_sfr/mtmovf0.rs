#[doc = "Register `MTMOVF0` reader"]
pub type R = crate::R<Mtmovf0Spec>;
#[doc = "Register `MTMOVF0` writer"]
pub type W = crate::W<Mtmovf0Spec>;
#[doc = "Field `MTMOVF0` reader - Indirectly returns and modifies bits \\[7:0\\]
of an internal register, depending on the value of MTMSEL.MTMOVFSEL. When reading the MTMOVF0 register with MTMSEL.MTMOVFSEL set to 000 and MTCTRL.LATCH_MODE set to 0, the overflow counter value (MTovf) is latched. When reading the MTM0 register with MTMSEL.MTMOVFSEL set to 000 and MTCTRL.LATCH_MODE set to 1, the overflow counter value (MTovf) is latched."]
pub type Mtmovf0R = crate::FieldReader;
#[doc = "Field `MTMOVF0` writer - Indirectly returns and modifies bits \\[7:0\\]
of an internal register, depending on the value of MTMSEL.MTMOVFSEL. When reading the MTMOVF0 register with MTMSEL.MTMOVFSEL set to 000 and MTCTRL.LATCH_MODE set to 0, the overflow counter value (MTovf) is latched. When reading the MTM0 register with MTMSEL.MTMOVFSEL set to 000 and MTCTRL.LATCH_MODE set to 1, the overflow counter value (MTovf) is latched."]
pub type Mtmovf0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indirectly returns and modifies bits \\[7:0\\]
of an internal register, depending on the value of MTMSEL.MTMOVFSEL. When reading the MTMOVF0 register with MTMSEL.MTMOVFSEL set to 000 and MTCTRL.LATCH_MODE set to 0, the overflow counter value (MTovf) is latched. When reading the MTM0 register with MTMSEL.MTMOVFSEL set to 000 and MTCTRL.LATCH_MODE set to 1, the overflow counter value (MTovf) is latched."]
    #[inline(always)]
    pub fn mtmovf0(&self) -> Mtmovf0R {
        Mtmovf0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indirectly returns and modifies bits \\[7:0\\]
of an internal register, depending on the value of MTMSEL.MTMOVFSEL. When reading the MTMOVF0 register with MTMSEL.MTMOVFSEL set to 000 and MTCTRL.LATCH_MODE set to 0, the overflow counter value (MTovf) is latched. When reading the MTM0 register with MTMSEL.MTMOVFSEL set to 000 and MTCTRL.LATCH_MODE set to 1, the overflow counter value (MTovf) is latched."]
    #[inline(always)]
    pub fn mtmovf0(&mut self) -> Mtmovf0W<Mtmovf0Spec> {
        Mtmovf0W::new(self, 0)
    }
}
#[doc = "MAC Timer multiplexed overflow register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mtmovf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtmovf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mtmovf0Spec;
impl crate::RegisterSpec for Mtmovf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtmovf0::R`](R) reader structure"]
impl crate::Readable for Mtmovf0Spec {}
#[doc = "`write(|w| ..)` method takes [`mtmovf0::W`](W) writer structure"]
impl crate::Writable for Mtmovf0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTMOVF0 to value 0"]
impl crate::Resettable for Mtmovf0Spec {
    const RESET_VALUE: u32 = 0;
}
