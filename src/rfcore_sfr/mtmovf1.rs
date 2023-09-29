#[doc = "Register `MTMOVF1` reader"]
pub type R = crate::R<MTMOVF1_SPEC>;
#[doc = "Register `MTMOVF1` writer"]
pub type W = crate::W<MTMOVF1_SPEC>;
#[doc = "Field `MTMOVF1` reader - Indirectly returns and modifies bits \\[15:8\\]
of an internal register, depending on the value of MTMSEL.MTMSEL. Reading this register with MTMSEL.MTMOVFSEL set to 000 returns the latched value of MTovf\\[15:8\\]."]
pub type MTMOVF1_R = crate::FieldReader;
#[doc = "Field `MTMOVF1` writer - Indirectly returns and modifies bits \\[15:8\\]
of an internal register, depending on the value of MTMSEL.MTMSEL. Reading this register with MTMSEL.MTMOVFSEL set to 000 returns the latched value of MTovf\\[15:8\\]."]
pub type MTMOVF1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Indirectly returns and modifies bits \\[15:8\\]
of an internal register, depending on the value of MTMSEL.MTMSEL. Reading this register with MTMSEL.MTMOVFSEL set to 000 returns the latched value of MTovf\\[15:8\\]."]
    #[inline(always)]
    pub fn mtmovf1(&self) -> MTMOVF1_R {
        MTMOVF1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indirectly returns and modifies bits \\[15:8\\]
of an internal register, depending on the value of MTMSEL.MTMSEL. Reading this register with MTMSEL.MTMOVFSEL set to 000 returns the latched value of MTovf\\[15:8\\]."]
    #[inline(always)]
    #[must_use]
    pub fn mtmovf1(&mut self) -> MTMOVF1_W<MTMOVF1_SPEC, 0> {
        MTMOVF1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MAC Timer multiplexed overflow register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtmovf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtmovf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTMOVF1_SPEC;
impl crate::RegisterSpec for MTMOVF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtmovf1::R`](R) reader structure"]
impl crate::Readable for MTMOVF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtmovf1::W`](W) writer structure"]
impl crate::Writable for MTMOVF1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTMOVF1 to value 0"]
impl crate::Resettable for MTMOVF1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
