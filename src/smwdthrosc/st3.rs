#[doc = "Register `ST3` reader"]
pub type R = crate::R<ST3_SPEC>;
#[doc = "Register `ST3` writer"]
pub type W = crate::W<ST3_SPEC>;
#[doc = "Field `ST3` reader - Sleep Timer count and compare value When read, this register returns the high bits \\[31:24\\]
of the Sleep Timer count. When writing this register sets the high bits \\[31:24\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
pub type ST3_R = crate::FieldReader;
#[doc = "Field `ST3` writer - Sleep Timer count and compare value When read, this register returns the high bits \\[31:24\\]
of the Sleep Timer count. When writing this register sets the high bits \\[31:24\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
pub type ST3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sleep Timer count and compare value When read, this register returns the high bits \\[31:24\\]
of the Sleep Timer count. When writing this register sets the high bits \\[31:24\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
    #[inline(always)]
    pub fn st3(&self) -> ST3_R {
        ST3_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sleep Timer count and compare value When read, this register returns the high bits \\[31:24\\]
of the Sleep Timer count. When writing this register sets the high bits \\[31:24\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
    #[inline(always)]
    #[must_use]
    pub fn st3(&mut self) -> ST3_W<ST3_SPEC> {
        ST3_W::new(self, 0)
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
#[doc = "Sleep Timer 3 count and compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST3_SPEC;
impl crate::RegisterSpec for ST3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st3::R`](R) reader structure"]
impl crate::Readable for ST3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st3::W`](W) writer structure"]
impl crate::Writable for ST3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ST3 to value 0"]
impl crate::Resettable for ST3_SPEC {
    const RESET_VALUE: u32 = 0;
}
