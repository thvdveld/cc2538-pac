#[doc = "Register `F0` reader"]
pub struct R(crate::R<F0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<F0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<F0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<F0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `F0` writer"]
pub struct W(crate::W<F0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<F0_SPEC>;
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
impl From<crate::W<F0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<F0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBF0` reader - Endpoint 0 FIFO Reading this register unloads one byte from the endpoint 0 FIFO. Writing to this register loads one byte into the endpoint 0 FIFO. The FIFO memory for EP0 is used for incoming and outgoing data packets."]
pub type USBF0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBF0` writer - Endpoint 0 FIFO Reading this register unloads one byte from the endpoint 0 FIFO. Writing to this register loads one byte into the endpoint 0 FIFO. The FIFO memory for EP0 is used for incoming and outgoing data packets."]
pub type USBF0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, F0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Endpoint 0 FIFO Reading this register unloads one byte from the endpoint 0 FIFO. Writing to this register loads one byte into the endpoint 0 FIFO. The FIFO memory for EP0 is used for incoming and outgoing data packets."]
    #[inline(always)]
    pub fn usbf0(&self) -> USBF0_R {
        USBF0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Endpoint 0 FIFO Reading this register unloads one byte from the endpoint 0 FIFO. Writing to this register loads one byte into the endpoint 0 FIFO. The FIFO memory for EP0 is used for incoming and outgoing data packets."]
    #[inline(always)]
    pub fn usbf0(&mut self) -> USBF0_W<0> {
        USBF0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint 0 FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f0](index.html) module"]
pub struct F0_SPEC;
impl crate::RegisterSpec for F0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [f0::R](R) reader structure"]
impl crate::Readable for F0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [f0::W](W) writer structure"]
impl crate::Writable for F0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets F0 to value 0"]
impl crate::Resettable for F0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
