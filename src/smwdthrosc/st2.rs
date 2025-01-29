#[doc = "Register `ST2` reader"]
pub type R = crate::R<St2Spec>;
#[doc = "Register `ST2` writer"]
pub type W = crate::W<St2Spec>;
#[doc = "Field `ST2` reader - Sleep Timer count and compare value When read, this register returns the high bits \\[23:16\\]
of the Sleep Timer count. When writing this register sets the high bits \\[23:16\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
pub type St2R = crate::FieldReader;
#[doc = "Field `ST2` writer - Sleep Timer count and compare value When read, this register returns the high bits \\[23:16\\]
of the Sleep Timer count. When writing this register sets the high bits \\[23:16\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
pub type St2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sleep Timer count and compare value When read, this register returns the high bits \\[23:16\\]
of the Sleep Timer count. When writing this register sets the high bits \\[23:16\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
    #[inline(always)]
    pub fn st2(&self) -> St2R {
        St2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sleep Timer count and compare value When read, this register returns the high bits \\[23:16\\]
of the Sleep Timer count. When writing this register sets the high bits \\[23:16\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
    #[inline(always)]
    pub fn st2(&mut self) -> St2W<St2Spec> {
        St2W::new(self, 0)
    }
}
#[doc = "Sleep Timer 2 count and compare\n\nYou can [`read`](crate::Reg::read) this register and get [`st2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St2Spec;
impl crate::RegisterSpec for St2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st2::R`](R) reader structure"]
impl crate::Readable for St2Spec {}
#[doc = "`write(|w| ..)` method takes [`st2::W`](W) writer structure"]
impl crate::Writable for St2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ST2 to value 0"]
impl crate::Resettable for St2Spec {
    const RESET_VALUE: u32 = 0;
}
