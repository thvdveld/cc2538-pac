#[doc = "Register `F5` reader"]
pub struct R(crate::R<F5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<F5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<F5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<F5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `F5` writer"]
pub struct W(crate::W<F5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<F5_SPEC>;
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
impl From<crate::W<F5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<F5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBF5` reader - Endpoint 5 FIFO register Reading this register unloads one byte from the EP5 OUT FIFO. Writing to this register loads one byte into the EP5 IN FIFO."]
pub type USBF5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBF5` writer - Endpoint 5 FIFO register Reading this register unloads one byte from the EP5 OUT FIFO. Writing to this register loads one byte into the EP5 IN FIFO."]
pub type USBF5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, F5_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Endpoint 5 FIFO register Reading this register unloads one byte from the EP5 OUT FIFO. Writing to this register loads one byte into the EP5 IN FIFO."]
    #[inline(always)]
    pub fn usbf5(&self) -> USBF5_R {
        USBF5_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Endpoint 5 FIFO register Reading this register unloads one byte from the EP5 OUT FIFO. Writing to this register loads one byte into the EP5 IN FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn usbf5(&mut self) -> USBF5_W<0> {
        USBF5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IN/OUT endpoint 5 FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f5](index.html) module"]
pub struct F5_SPEC;
impl crate::RegisterSpec for F5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [f5::R](R) reader structure"]
impl crate::Readable for F5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [f5::W](W) writer structure"]
impl crate::Writable for F5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets F5 to value 0"]
impl crate::Resettable for F5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
