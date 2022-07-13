#[doc = "Register `ST2` reader"]
pub struct R(crate::R<ST2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ST2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ST2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ST2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ST2` writer"]
pub struct W(crate::W<ST2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ST2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ST2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ST2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ST2` reader - Sleep Timer count and compare value When read, this register returns the high bits \\[23:16\\]
of the Sleep Timer count. When writing this register sets the high bits \\[23:16\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
pub type ST2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ST2` writer - Sleep Timer count and compare value When read, this register returns the high bits \\[23:16\\]
of the Sleep Timer count. When writing this register sets the high bits \\[23:16\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
pub type ST2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ST2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Sleep Timer count and compare value When read, this register returns the high bits \\[23:16\\]
of the Sleep Timer count. When writing this register sets the high bits \\[23:16\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
    #[inline(always)]
    pub fn st2(&self) -> ST2_R {
        ST2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sleep Timer count and compare value When read, this register returns the high bits \\[23:16\\]
of the Sleep Timer count. When writing this register sets the high bits \\[23:16\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
    #[inline(always)]
    pub fn st2(&mut self) -> ST2_W<0> {
        ST2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sleep Timer 2 count and compare\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2](index.html) module"]
pub struct ST2_SPEC;
impl crate::RegisterSpec for ST2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [st2::R](R) reader structure"]
impl crate::Readable for ST2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [st2::W](W) writer structure"]
impl crate::Writable for ST2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ST2 to value 0"]
impl crate::Resettable for ST2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
