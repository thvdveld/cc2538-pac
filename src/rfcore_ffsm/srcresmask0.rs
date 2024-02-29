#[doc = "Register `SRCRESMASK0` reader"]
pub type R = crate::R<Srcresmask0Spec>;
#[doc = "Register `SRCRESMASK0` writer"]
pub type W = crate::W<Srcresmask0Spec>;
#[doc = "Field `SRCRESMASK0` reader - Extended address matching When there is a match on entry ext_n, bits 2n and 2n + 1 are set in SRCRESMASK."]
pub type Srcresmask0R = crate::FieldReader;
#[doc = "Field `SRCRESMASK0` writer - Extended address matching When there is a match on entry ext_n, bits 2n and 2n + 1 are set in SRCRESMASK."]
pub type Srcresmask0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Extended address matching When there is a match on entry ext_n, bits 2n and 2n + 1 are set in SRCRESMASK."]
    #[inline(always)]
    pub fn srcresmask0(&self) -> Srcresmask0R {
        Srcresmask0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Extended address matching When there is a match on entry ext_n, bits 2n and 2n + 1 are set in SRCRESMASK."]
    #[inline(always)]
    #[must_use]
    pub fn srcresmask0(&mut self) -> Srcresmask0W<Srcresmask0Spec> {
        Srcresmask0W::new(self, 0)
    }
}
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcresmask0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcresmask0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srcresmask0Spec;
impl crate::RegisterSpec for Srcresmask0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcresmask0::R`](R) reader structure"]
impl crate::Readable for Srcresmask0Spec {}
#[doc = "`write(|w| ..)` method takes [`srcresmask0::W`](W) writer structure"]
impl crate::Writable for Srcresmask0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCRESMASK0 to value 0"]
impl crate::Resettable for Srcresmask0Spec {
    const RESET_VALUE: u32 = 0;
}
