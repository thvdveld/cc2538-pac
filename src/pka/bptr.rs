#[doc = "Register `BPTR` reader"]
pub struct R(crate::R<BPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BPTR` writer"]
pub struct W(crate::W<BPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BPTR_SPEC>;
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
impl From<crate::W<BPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BPTR` reader - This register specifies the location of vector B within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\]
must be zero to ensure that the vector starts at an 8-byte boundary."]
pub struct BPTR_R(crate::FieldReader<u16, u16>);
impl BPTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BPTR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BPTR` writer - This register specifies the location of vector B within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\]
must be zero to ensure that the vector starts at an 8-byte boundary."]
pub struct BPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> BPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - This register specifies the location of vector B within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\]
must be zero to ensure that the vector starts at an 8-byte boundary."]
    #[inline(always)]
    pub fn bptr(&self) -> BPTR_R {
        BPTR_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - This register specifies the location of vector B within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\]
must be zero to ensure that the vector starts at an 8-byte boundary."]
    #[inline(always)]
    pub fn bptr(&mut self) -> BPTR_W {
        BPTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PKA vector B address During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bptr](index.html) module"]
pub struct BPTR_SPEC;
impl crate::RegisterSpec for BPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bptr::R](R) reader structure"]
impl crate::Readable for BPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bptr::W](W) writer structure"]
impl crate::Writable for BPTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BPTR to value 0"]
impl crate::Resettable for BPTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
