#[doc = "Register `ST1` reader"]
pub type R = crate::R<St1Spec>;
#[doc = "Register `ST1` writer"]
pub type W = crate::W<St1Spec>;
#[doc = "Field `ST1` reader - Sleep Timer count and compare value When read, this register returns the middle bits \\[15:8\\]
of the Sleep Timer count. When writing this register sets the middle bits \\[15:8\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
pub type St1R = crate::FieldReader;
#[doc = "Field `ST1` writer - Sleep Timer count and compare value When read, this register returns the middle bits \\[15:8\\]
of the Sleep Timer count. When writing this register sets the middle bits \\[15:8\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
pub type St1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sleep Timer count and compare value When read, this register returns the middle bits \\[15:8\\]
of the Sleep Timer count. When writing this register sets the middle bits \\[15:8\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
    #[inline(always)]
    pub fn st1(&self) -> St1R {
        St1R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sleep Timer count and compare value When read, this register returns the middle bits \\[15:8\\]
of the Sleep Timer count. When writing this register sets the middle bits \\[15:8\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
    #[inline(always)]
    pub fn st1(&mut self) -> St1W<St1Spec> {
        St1W::new(self, 0)
    }
}
#[doc = "Sleep Timer 1 count and compare\n\nYou can [`read`](crate::Reg::read) this register and get [`st1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St1Spec;
impl crate::RegisterSpec for St1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st1::R`](R) reader structure"]
impl crate::Readable for St1Spec {}
#[doc = "`write(|w| ..)` method takes [`st1::W`](W) writer structure"]
impl crate::Writable for St1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ST1 to value 0"]
impl crate::Resettable for St1Spec {
    const RESET_VALUE: u32 = 0;
}
