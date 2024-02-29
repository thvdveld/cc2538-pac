#[doc = "Register `SRCRESMASK2` reader"]
pub type R = crate::R<Srcresmask2Spec>;
#[doc = "Register `SRCRESMASK2` writer"]
pub type W = crate::W<Srcresmask2Spec>;
#[doc = "Field `SRCRESMASK2` reader - 24-bit mask that indicates source address match for each individual entry in the source address table"]
pub type Srcresmask2R = crate::FieldReader;
#[doc = "Field `SRCRESMASK2` writer - 24-bit mask that indicates source address match for each individual entry in the source address table"]
pub type Srcresmask2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 24-bit mask that indicates source address match for each individual entry in the source address table"]
    #[inline(always)]
    pub fn srcresmask2(&self) -> Srcresmask2R {
        Srcresmask2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 24-bit mask that indicates source address match for each individual entry in the source address table"]
    #[inline(always)]
    #[must_use]
    pub fn srcresmask2(&mut self) -> Srcresmask2W<Srcresmask2Spec> {
        Srcresmask2W::new(self, 0)
    }
}
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcresmask2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcresmask2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srcresmask2Spec;
impl crate::RegisterSpec for Srcresmask2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcresmask2::R`](R) reader structure"]
impl crate::Readable for Srcresmask2Spec {}
#[doc = "`write(|w| ..)` method takes [`srcresmask2::W`](W) writer structure"]
impl crate::Writable for Srcresmask2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCRESMASK2 to value 0"]
impl crate::Resettable for Srcresmask2Spec {
    const RESET_VALUE: u32 = 0;
}
