#[doc = "Register `F2` reader"]
pub struct R(crate::R<F2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<F2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<F2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<F2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `F2` writer"]
pub struct W(crate::W<F2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<F2_SPEC>;
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
impl From<crate::W<F2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<F2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBF2` reader - Endpoint 2 FIFO register Reading this register unloads one byte from the EP2 OUT FIFO. Writing to this register loads one byte into the EP2 IN FIFO."]
pub type USBF2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBF2` writer - Endpoint 2 FIFO register Reading this register unloads one byte from the EP2 OUT FIFO. Writing to this register loads one byte into the EP2 IN FIFO."]
pub type USBF2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, F2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Endpoint 2 FIFO register Reading this register unloads one byte from the EP2 OUT FIFO. Writing to this register loads one byte into the EP2 IN FIFO."]
    #[inline(always)]
    pub fn usbf2(&self) -> USBF2_R {
        USBF2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Endpoint 2 FIFO register Reading this register unloads one byte from the EP2 OUT FIFO. Writing to this register loads one byte into the EP2 IN FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn usbf2(&mut self) -> USBF2_W<0> {
        USBF2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IN/OUT endpoint 2 FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f2](index.html) module"]
pub struct F2_SPEC;
impl crate::RegisterSpec for F2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [f2::R](R) reader structure"]
impl crate::Readable for F2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [f2::W](W) writer structure"]
impl crate::Writable for F2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets F2 to value 0"]
impl crate::Resettable for F2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
