#[doc = "Register `MTM1` reader"]
pub type R = crate::R<Mtm1Spec>;
#[doc = "Register `MTM1` writer"]
pub type W = crate::W<Mtm1Spec>;
#[doc = "Field `MTM1` reader - Indirectly returns and modifies bits \\[15:8\\]
of an internal register, depending on the value of MTMSEL.MTMSEL. When reading the MTM0 register with MTMSEL.MTMSEL set to 000, the timer (MTtim) value is latched. Reading this register with MTMSEL.MTMSEL set to 000 returns the latched value of MTtim\\[15:8\\]."]
pub type Mtm1R = crate::FieldReader;
#[doc = "Field `MTM1` writer - Indirectly returns and modifies bits \\[15:8\\]
of an internal register, depending on the value of MTMSEL.MTMSEL. When reading the MTM0 register with MTMSEL.MTMSEL set to 000, the timer (MTtim) value is latched. Reading this register with MTMSEL.MTMSEL set to 000 returns the latched value of MTtim\\[15:8\\]."]
pub type Mtm1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indirectly returns and modifies bits \\[15:8\\]
of an internal register, depending on the value of MTMSEL.MTMSEL. When reading the MTM0 register with MTMSEL.MTMSEL set to 000, the timer (MTtim) value is latched. Reading this register with MTMSEL.MTMSEL set to 000 returns the latched value of MTtim\\[15:8\\]."]
    #[inline(always)]
    pub fn mtm1(&self) -> Mtm1R {
        Mtm1R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indirectly returns and modifies bits \\[15:8\\]
of an internal register, depending on the value of MTMSEL.MTMSEL. When reading the MTM0 register with MTMSEL.MTMSEL set to 000, the timer (MTtim) value is latched. Reading this register with MTMSEL.MTMSEL set to 000 returns the latched value of MTtim\\[15:8\\]."]
    #[inline(always)]
    pub fn mtm1(&mut self) -> Mtm1W<Mtm1Spec> {
        Mtm1W::new(self, 0)
    }
}
#[doc = "MAC Timer multiplexed register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mtm1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtm1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mtm1Spec;
impl crate::RegisterSpec for Mtm1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtm1::R`](R) reader structure"]
impl crate::Readable for Mtm1Spec {}
#[doc = "`write(|w| ..)` method takes [`mtm1::W`](W) writer structure"]
impl crate::Writable for Mtm1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTM1 to value 0"]
impl crate::Resettable for Mtm1Spec {
    const RESET_VALUE: u32 = 0;
}
