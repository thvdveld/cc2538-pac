#[doc = "Register `MTMOVF2` reader"]
pub type R = crate::R<MTMOVF2_SPEC>;
#[doc = "Register `MTMOVF2` writer"]
pub type W = crate::W<MTMOVF2_SPEC>;
#[doc = "Field `MTMOVF2` reader - Indirectly returns and modifies bits \\[23:16\\]
of an internal register, depending on the value of MTMSEL.MTMOVFSEL. Reading this register with MTMSEL.MTMOVFSEL set to 000 returns the latched value of MTovf\\[23:16\\]."]
pub type MTMOVF2_R = crate::FieldReader;
#[doc = "Field `MTMOVF2` writer - Indirectly returns and modifies bits \\[23:16\\]
of an internal register, depending on the value of MTMSEL.MTMOVFSEL. Reading this register with MTMSEL.MTMOVFSEL set to 000 returns the latched value of MTovf\\[23:16\\]."]
pub type MTMOVF2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indirectly returns and modifies bits \\[23:16\\]
of an internal register, depending on the value of MTMSEL.MTMOVFSEL. Reading this register with MTMSEL.MTMOVFSEL set to 000 returns the latched value of MTovf\\[23:16\\]."]
    #[inline(always)]
    pub fn mtmovf2(&self) -> MTMOVF2_R {
        MTMOVF2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indirectly returns and modifies bits \\[23:16\\]
of an internal register, depending on the value of MTMSEL.MTMOVFSEL. Reading this register with MTMSEL.MTMOVFSEL set to 000 returns the latched value of MTovf\\[23:16\\]."]
    #[inline(always)]
    #[must_use]
    pub fn mtmovf2(&mut self) -> MTMOVF2_W<MTMOVF2_SPEC> {
        MTMOVF2_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MAC Timer multiplexed overflow register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtmovf2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtmovf2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTMOVF2_SPEC;
impl crate::RegisterSpec for MTMOVF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtmovf2::R`](R) reader structure"]
impl crate::Readable for MTMOVF2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtmovf2::W`](W) writer structure"]
impl crate::Writable for MTMOVF2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTMOVF2 to value 0"]
impl crate::Resettable for MTMOVF2_SPEC {
    const RESET_VALUE: u32 = 0;
}
