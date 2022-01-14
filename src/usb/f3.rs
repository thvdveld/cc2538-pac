#[doc = "Register `F3` reader"]
pub struct R(crate::R<F3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<F3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<F3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<F3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `F3` writer"]
pub struct W(crate::W<F3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<F3_SPEC>;
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
impl From<crate::W<F3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<F3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBF3` reader - Endpoint 3 FIFO register Reading this register unloads one byte from the EP3 OUT FIFO. Writing to this register loads one byte into the EP3 IN FIFO."]
pub struct USBF3_R(crate::FieldReader<u8, u8>);
impl USBF3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USBF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBF3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBF3` writer - Endpoint 3 FIFO register Reading this register unloads one byte from the EP3 OUT FIFO. Writing to this register loads one byte into the EP3 IN FIFO."]
pub struct USBF3_W<'a> {
    w: &'a mut W,
}
impl<'a> USBF3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Endpoint 3 FIFO register Reading this register unloads one byte from the EP3 OUT FIFO. Writing to this register loads one byte into the EP3 IN FIFO."]
    #[inline(always)]
    pub fn usbf3(&self) -> USBF3_R {
        USBF3_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Endpoint 3 FIFO register Reading this register unloads one byte from the EP3 OUT FIFO. Writing to this register loads one byte into the EP3 IN FIFO."]
    #[inline(always)]
    pub fn usbf3(&mut self) -> USBF3_W {
        USBF3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IN/OUT endpoint 3 FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f3](index.html) module"]
pub struct F3_SPEC;
impl crate::RegisterSpec for F3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [f3::R](R) reader structure"]
impl crate::Readable for F3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [f3::W](W) writer structure"]
impl crate::Writable for F3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets F3 to value 0"]
impl crate::Resettable for F3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
