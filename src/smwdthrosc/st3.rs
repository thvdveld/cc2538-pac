#[doc = "Register `ST3` reader"]
pub type R = crate::R<St3Spec>;
#[doc = "Register `ST3` writer"]
pub type W = crate::W<St3Spec>;
#[doc = "Field `ST3` reader - Sleep Timer count and compare value When read, this register returns the high bits \\[31:24\\]
of the Sleep Timer count. When writing this register sets the high bits \\[31:24\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
pub type St3R = crate::FieldReader;
#[doc = "Field `ST3` writer - Sleep Timer count and compare value When read, this register returns the high bits \\[31:24\\]
of the Sleep Timer count. When writing this register sets the high bits \\[31:24\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
pub type St3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sleep Timer count and compare value When read, this register returns the high bits \\[31:24\\]
of the Sleep Timer count. When writing this register sets the high bits \\[31:24\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
    #[inline(always)]
    pub fn st3(&self) -> St3R {
        St3R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sleep Timer count and compare value When read, this register returns the high bits \\[31:24\\]
of the Sleep Timer count. When writing this register sets the high bits \\[31:24\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
    #[inline(always)]
    #[must_use]
    pub fn st3(&mut self) -> St3W<St3Spec> {
        St3W::new(self, 0)
    }
}
#[doc = "Sleep Timer 3 count and compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St3Spec;
impl crate::RegisterSpec for St3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st3::R`](R) reader structure"]
impl crate::Readable for St3Spec {}
#[doc = "`write(|w| ..)` method takes [`st3::W`](W) writer structure"]
impl crate::Writable for St3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ST3 to value 0"]
impl crate::Resettable for St3Spec {
    const RESET_VALUE: u32 = 0;
}
