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
pub struct ST1_R(crate::FieldReader<u8, u8>);
impl ST1_R {
    pub(crate) fn new(bits: u8) -> Self {
        ST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST1` writer - Sleep Timer count and compare value When read, this register returns the middle bits \\[15:8\\]
of the Sleep Timer count. When writing this register sets the middle bits \\[15:8\\]
of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
pub struct ST1_W<'a> {
    w: &'a mut W,
}
impl<'a> ST1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
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
    pub fn st1(&mut self) -> ST1_W {
        ST1_W { w: self }
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
}
#[doc = "`reset()` method sets ST1 to value 0"]
impl crate::Resettable for ST1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
