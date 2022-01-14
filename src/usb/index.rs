#[doc = "Register `INDEX` reader"]
pub struct R(crate::R<INDEX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INDEX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INDEX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INDEX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INDEX` writer"]
pub struct W(crate::W<INDEX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INDEX_SPEC>;
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
impl From<crate::W<INDEX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INDEX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBINDEX` reader - Index of the currently selected endpoint The index is set to 0 to enable access to endpoint 0 control and status registers The index is set to 1, 2, 3, 4 or 5 to enable access to IN/OUT endpoint 1, 2, 3, 4 or 5 control and status registers, respectively"]
pub struct USBINDEX_R(crate::FieldReader<u8, u8>);
impl USBINDEX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USBINDEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBINDEX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBINDEX` writer - Index of the currently selected endpoint The index is set to 0 to enable access to endpoint 0 control and status registers The index is set to 1, 2, 3, 4 or 5 to enable access to IN/OUT endpoint 1, 2, 3, 4 or 5 control and status registers, respectively"]
pub struct USBINDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> USBINDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Index of the currently selected endpoint The index is set to 0 to enable access to endpoint 0 control and status registers The index is set to 1, 2, 3, 4 or 5 to enable access to IN/OUT endpoint 1, 2, 3, 4 or 5 control and status registers, respectively"]
    #[inline(always)]
    pub fn usbindex(&self) -> USBINDEX_R {
        USBINDEX_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Index of the currently selected endpoint The index is set to 0 to enable access to endpoint 0 control and status registers The index is set to 1, 2, 3, 4 or 5 to enable access to IN/OUT endpoint 1, 2, 3, 4 or 5 control and status registers, respectively"]
    #[inline(always)]
    pub fn usbindex(&mut self) -> USBINDEX_W {
        USBINDEX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Index register for selecting the endpoint status and control registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [index](index.html) module"]
pub struct INDEX_SPEC;
impl crate::RegisterSpec for INDEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [index::R](R) reader structure"]
impl crate::Readable for INDEX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [index::W](W) writer structure"]
impl crate::Writable for INDEX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INDEX to value 0"]
impl crate::Resettable for INDEX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
