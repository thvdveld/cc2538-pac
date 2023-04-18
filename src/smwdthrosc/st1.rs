#[doc = "Register `ST1` reader"]
pub struct R(crate::R<ST1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ST1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ST1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ST1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ST1` writer"]
pub struct W(crate::W<ST1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ST1_SPEC>;
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
impl From<crate::W<ST1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ST1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ST1` reader - Sleep Timer count and compare value When read, this register returns the middle bits \\[15:8\\]
of the Sleep Timer count. When writing this register sets the middle bits \\[15:8\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
pub type ST1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ST1` writer - Sleep Timer count and compare value When read, this register returns the middle bits \\[15:8\\]
of the Sleep Timer count. When writing this register sets the middle bits \\[15:8\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
pub type ST1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ST1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Sleep Timer count and compare value When read, this register returns the middle bits \\[15:8\\]
of the Sleep Timer count. When writing this register sets the middle bits \\[15:8\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
    #[inline(always)]
    pub fn st1(&self) -> ST1_R {
        ST1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sleep Timer count and compare value When read, this register returns the middle bits \\[15:8\\]
of the Sleep Timer count. When writing this register sets the middle bits \\[15:8\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
    #[inline(always)]
    #[must_use]
    pub fn st1(&mut self) -> ST1_W<0> {
        ST1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sleep Timer 1 count and compare\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st1](index.html) module"]
pub struct ST1_SPEC;
impl crate::RegisterSpec for ST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [st1::R](R) reader structure"]
impl crate::Readable for ST1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [st1::W](W) writer structure"]
impl crate::Writable for ST1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST1 to value 0"]
impl crate::Resettable for ST1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
