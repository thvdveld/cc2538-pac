#[doc = "Register `ST0` reader"]
pub type R = crate::R<St0Spec>;
#[doc = "Register `ST0` writer"]
pub type W = crate::W<St0Spec>;
#[doc = "Field `ST0` reader - Sleep Timer count and compare value. When read, this register returns the low bits \\[7:0\\]
of the Sleep Timer count. When writing this register sets the low bits \\[7:0\\]
of the compare value."]
pub type St0R = crate::FieldReader;
#[doc = "Field `ST0` writer - Sleep Timer count and compare value. When read, this register returns the low bits \\[7:0\\]
of the Sleep Timer count. When writing this register sets the low bits \\[7:0\\]
of the compare value."]
pub type St0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sleep Timer count and compare value. When read, this register returns the low bits \\[7:0\\]
of the Sleep Timer count. When writing this register sets the low bits \\[7:0\\]
of the compare value."]
    #[inline(always)]
    pub fn st0(&self) -> St0R {
        St0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sleep Timer count and compare value. When read, this register returns the low bits \\[7:0\\]
of the Sleep Timer count. When writing this register sets the low bits \\[7:0\\]
of the compare value."]
    #[inline(always)]
    #[must_use]
    pub fn st0(&mut self) -> St0W<St0Spec> {
        St0W::new(self, 0)
    }
}
#[doc = "Sleep Timer 0 count and compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St0Spec;
impl crate::RegisterSpec for St0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st0::R`](R) reader structure"]
impl crate::Readable for St0Spec {}
#[doc = "`write(|w| ..)` method takes [`st0::W`](W) writer structure"]
impl crate::Writable for St0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ST0 to value 0"]
impl crate::Resettable for St0Spec {
    const RESET_VALUE: u32 = 0;
}
