#[doc = "Register `F1` reader"]
pub struct R(crate::R<F1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<F1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<F1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<F1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `F1` writer"]
pub struct W(crate::W<F1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<F1_SPEC>;
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
impl From<crate::W<F1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<F1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBF1` reader - Endpoint 1 FIFO register Reading this register unloads one byte from the EP1 OUT FIFO. Writing to this register loads one byte into the EP1 IN FIFO."]
pub struct USBF1_R(crate::FieldReader<u8, u8>);
impl USBF1_R {
    pub(crate) fn new(bits: u8) -> Self {
        USBF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBF1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBF1` writer - Endpoint 1 FIFO register Reading this register unloads one byte from the EP1 OUT FIFO. Writing to this register loads one byte into the EP1 IN FIFO."]
pub struct USBF1_W<'a> {
    w: &'a mut W,
}
impl<'a> USBF1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Endpoint 1 FIFO register Reading this register unloads one byte from the EP1 OUT FIFO. Writing to this register loads one byte into the EP1 IN FIFO."]
    #[inline(always)]
    pub fn usbf1(&self) -> USBF1_R {
        USBF1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Endpoint 1 FIFO register Reading this register unloads one byte from the EP1 OUT FIFO. Writing to this register loads one byte into the EP1 IN FIFO."]
    #[inline(always)]
    pub fn usbf1(&mut self) -> USBF1_W {
        USBF1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IN/OUT endpoint 1 FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f1](index.html) module"]
pub struct F1_SPEC;
impl crate::RegisterSpec for F1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [f1::R](R) reader structure"]
impl crate::Readable for F1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [f1::W](W) writer structure"]
impl crate::Writable for F1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets F1 to value 0"]
impl crate::Resettable for F1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
