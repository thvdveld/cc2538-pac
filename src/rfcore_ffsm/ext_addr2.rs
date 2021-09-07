#[doc = "Register `EXT_ADDR2` reader"]
pub struct R(crate::R<EXT_ADDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT_ADDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT_ADDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT_ADDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT_ADDR2` writer"]
pub struct W(crate::W<EXT_ADDR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT_ADDR2_SPEC>;
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
impl From<crate::W<EXT_ADDR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT_ADDR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXT_ADDR2` reader - EXT_ADDR\\[23:16\\]
The IEEE extended address used during destination address filtering"]
pub struct EXT_ADDR2_R(crate::FieldReader<u8, u8>);
impl EXT_ADDR2_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXT_ADDR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXT_ADDR2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXT_ADDR2` writer - EXT_ADDR\\[23:16\\]
The IEEE extended address used during destination address filtering"]
pub struct EXT_ADDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_ADDR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - EXT_ADDR\\[23:16\\]
The IEEE extended address used during destination address filtering"]
    #[inline(always)]
    pub fn ext_addr2(&self) -> EXT_ADDR2_R {
        EXT_ADDR2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXT_ADDR\\[23:16\\]
The IEEE extended address used during destination address filtering"]
    #[inline(always)]
    pub fn ext_addr2(&mut self) -> EXT_ADDR2_W {
        EXT_ADDR2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_addr2](index.html) module"]
pub struct EXT_ADDR2_SPEC;
impl crate::RegisterSpec for EXT_ADDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext_addr2::R](R) reader structure"]
impl crate::Readable for EXT_ADDR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext_addr2::W](W) writer structure"]
impl crate::Writable for EXT_ADDR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXT_ADDR2 to value 0"]
impl crate::Resettable for EXT_ADDR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
