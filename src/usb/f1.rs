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
pub type USBF1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBF1` writer - Endpoint 1 FIFO register Reading this register unloads one byte from the EP1 OUT FIFO. Writing to this register loads one byte into the EP1 IN FIFO."]
pub type USBF1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, F1_SPEC, u8, u8, 8, O>;
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
    pub fn usbf1(&mut self) -> USBF1_W<0> {
        USBF1_W::new(self)
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
