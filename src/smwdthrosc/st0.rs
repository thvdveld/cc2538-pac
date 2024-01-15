#[doc = "Register `ST0` reader"]
pub type R = crate::R<ST0_SPEC>;
#[doc = "Register `ST0` writer"]
pub type W = crate::W<ST0_SPEC>;
#[doc = "Field `ST0` reader - Sleep Timer count and compare value. When read, this register returns the low bits \\[7:0\\]
of the Sleep Timer count. When writing this register sets the low bits \\[7:0\\]
of the compare value."]
pub type ST0_R = crate::FieldReader;
#[doc = "Field `ST0` writer - Sleep Timer count and compare value. When read, this register returns the low bits \\[7:0\\]
of the Sleep Timer count. When writing this register sets the low bits \\[7:0\\]
of the compare value."]
pub type ST0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sleep Timer count and compare value. When read, this register returns the low bits \\[7:0\\]
of the Sleep Timer count. When writing this register sets the low bits \\[7:0\\]
of the compare value."]
    #[inline(always)]
    pub fn st0(&self) -> ST0_R {
        ST0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sleep Timer count and compare value. When read, this register returns the low bits \\[7:0\\]
of the Sleep Timer count. When writing this register sets the low bits \\[7:0\\]
of the compare value."]
    #[inline(always)]
    #[must_use]
    pub fn st0(&mut self) -> ST0_W<ST0_SPEC> {
        ST0_W::new(self, 0)
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
#[doc = "Sleep Timer 0 count and compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST0_SPEC;
impl crate::RegisterSpec for ST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st0::R`](R) reader structure"]
impl crate::Readable for ST0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st0::W`](W) writer structure"]
impl crate::Writable for ST0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ST0 to value 0"]
impl crate::Resettable for ST0_SPEC {
    const RESET_VALUE: u32 = 0;
}
