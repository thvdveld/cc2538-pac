#[doc = "Register `MTMOVF2` reader"]
pub type R = crate::R<Mtmovf2Spec>;
#[doc = "Register `MTMOVF2` writer"]
pub type W = crate::W<Mtmovf2Spec>;
#[doc = "Field `MTMOVF2` reader - Indirectly returns and modifies bits \\[23:16\\]
of an internal register, depending on the value of MTMSEL.MTMOVFSEL. Reading this register with MTMSEL.MTMOVFSEL set to 000 returns the latched value of MTovf\\[23:16\\]."]
pub type Mtmovf2R = crate::FieldReader;
#[doc = "Field `MTMOVF2` writer - Indirectly returns and modifies bits \\[23:16\\]
of an internal register, depending on the value of MTMSEL.MTMOVFSEL. Reading this register with MTMSEL.MTMOVFSEL set to 000 returns the latched value of MTovf\\[23:16\\]."]
pub type Mtmovf2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indirectly returns and modifies bits \\[23:16\\]
of an internal register, depending on the value of MTMSEL.MTMOVFSEL. Reading this register with MTMSEL.MTMOVFSEL set to 000 returns the latched value of MTovf\\[23:16\\]."]
    #[inline(always)]
    pub fn mtmovf2(&self) -> Mtmovf2R {
        Mtmovf2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indirectly returns and modifies bits \\[23:16\\]
of an internal register, depending on the value of MTMSEL.MTMOVFSEL. Reading this register with MTMSEL.MTMOVFSEL set to 000 returns the latched value of MTovf\\[23:16\\]."]
    #[inline(always)]
    pub fn mtmovf2(&mut self) -> Mtmovf2W<Mtmovf2Spec> {
        Mtmovf2W::new(self, 0)
    }
}
#[doc = "MAC Timer multiplexed overflow register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`mtmovf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtmovf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mtmovf2Spec;
impl crate::RegisterSpec for Mtmovf2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtmovf2::R`](R) reader structure"]
impl crate::Readable for Mtmovf2Spec {}
#[doc = "`write(|w| ..)` method takes [`mtmovf2::W`](W) writer structure"]
impl crate::Writable for Mtmovf2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTMOVF2 to value 0"]
impl crate::Resettable for Mtmovf2Spec {
    const RESET_VALUE: u32 = 0;
}
