#[doc = "Register `ST3` reader"]
pub struct R(crate::R<ST3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ST3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ST3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ST3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ST3` writer"]
pub struct W(crate::W<ST3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ST3_SPEC>;
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
impl From<crate::W<ST3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ST3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ST3` reader - Sleep Timer count and compare value When read, this register returns the high bits \\[31:24\\]
of the Sleep Timer count. When writing this register sets the high bits \\[31:24\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
pub type ST3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ST3` writer - Sleep Timer count and compare value When read, this register returns the high bits \\[31:24\\]
of the Sleep Timer count. When writing this register sets the high bits \\[31:24\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
pub type ST3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ST3_SPEC, u8, u8, 8, O>;
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
    pub fn st3(&mut self) -> ST3_W<0> {
        ST3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sleep Timer 3 count and compare\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st3](index.html) module"]
pub struct ST3_SPEC;
impl crate::RegisterSpec for ST3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [st3::R](R) reader structure"]
impl crate::Readable for ST3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [st3::W](W) writer structure"]
impl crate::Writable for ST3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ST3 to value 0"]
impl crate::Resettable for ST3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
