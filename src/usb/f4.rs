#[doc = "Register `F4` reader"]
pub struct R(crate::R<F4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<F4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<F4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<F4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `F4` writer"]
pub struct W(crate::W<F4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<F4_SPEC>;
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
impl From<crate::W<F4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<F4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBF4` reader - Endpoint 4 FIFO register Reading this register unloads one byte from the EP4 OUT FIFO. Writing to this register loads one byte into the EP4 IN FIFO."]
pub struct USBF4_R(crate::FieldReader<u8, u8>);
impl USBF4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USBF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBF4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBF4` writer - Endpoint 4 FIFO register Reading this register unloads one byte from the EP4 OUT FIFO. Writing to this register loads one byte into the EP4 IN FIFO."]
pub struct USBF4_W<'a> {
    w: &'a mut W,
}
impl<'a> USBF4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Endpoint 4 FIFO register Reading this register unloads one byte from the EP4 OUT FIFO. Writing to this register loads one byte into the EP4 IN FIFO."]
    #[inline(always)]
    pub fn usbf4(&self) -> USBF4_R {
        USBF4_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Endpoint 4 FIFO register Reading this register unloads one byte from the EP4 OUT FIFO. Writing to this register loads one byte into the EP4 IN FIFO."]
    #[inline(always)]
    pub fn usbf4(&mut self) -> USBF4_W {
        USBF4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IN/OUT endpoint 4 FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f4](index.html) module"]
pub struct F4_SPEC;
impl crate::RegisterSpec for F4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [f4::R](R) reader structure"]
impl crate::Readable for F4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [f4::W](W) writer structure"]
impl crate::Writable for F4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets F4 to value 0"]
impl crate::Resettable for F4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
