#[doc = "Register `CSOL` reader"]
pub struct R(crate::R<CSOL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSOL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSOL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSOL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSOL` writer"]
pub struct W(crate::W<CSOL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSOL_SPEC>;
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
impl From<crate::W<CSOL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSOL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLRDATATOG` reader - Software sets this bit to reset the endpoint data toggle to 0."]
pub type CLRDATATOG_R = crate::BitReader<bool>;
#[doc = "Field `CLRDATATOG` writer - Software sets this bit to reset the endpoint data toggle to 0."]
pub type CLRDATATOG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSOL_SPEC, bool, O>;
#[doc = "Field `SENTSTALL` reader - This bit is set when a STALL handshake is transmitted. An interrupt is generated when this bit is set. Software should clear this bit."]
pub type SENTSTALL_R = crate::BitReader<bool>;
#[doc = "Field `SENTSTALL` writer - This bit is set when a STALL handshake is transmitted. An interrupt is generated when this bit is set. Software should clear this bit."]
pub type SENTSTALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSOL_SPEC, bool, O>;
#[doc = "Field `SENDSTALL` reader - For bulk/interrupt mode OUT endpoints: Software sets this bit to issue a STALL handshake. Software clears this bit to terminate the stall condition."]
pub type SENDSTALL_R = crate::BitReader<bool>;
#[doc = "Field `SENDSTALL` writer - For bulk/interrupt mode OUT endpoints: Software sets this bit to issue a STALL handshake. Software clears this bit to terminate the stall condition."]
pub type SENDSTALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSOL_SPEC, bool, O>;
#[doc = "Field `FLUSHPACKET` reader - Software sets this bit to flush the next packet to be read from the endpoint OUT FIFO. Note: If the FIFO contains two packets, USB_CSOL.FLUSHPACKET will need to be set twice to completely clear the FIFO."]
pub type FLUSHPACKET_R = crate::BitReader<bool>;
#[doc = "Field `FLUSHPACKET` writer - Software sets this bit to flush the next packet to be read from the endpoint OUT FIFO. Note: If the FIFO contains two packets, USB_CSOL.FLUSHPACKET will need to be set twice to completely clear the FIFO."]
pub type FLUSHPACKET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSOL_SPEC, bool, O>;
#[doc = "Field `DATAERROR` reader - For isochronous mode OUT endpoints: This bit is set when USB_CSOL.OUTPKTRDY is set if the data packet has a CRC or bit-stuff error. It is cleared automatically when USB_CSOL.OUTPKTRDY is cleared."]
pub type DATAERROR_R = crate::BitReader<bool>;
#[doc = "Field `OVERRUN` reader - For isochronous mode OUT endpoints: This bit is set when an OUT packet cannot be loaded into the OUT endpoint FIFO. Firmware should clear this bit."]
pub type OVERRUN_R = crate::BitReader<bool>;
#[doc = "Field `OVERRUN` writer - For isochronous mode OUT endpoints: This bit is set when an OUT packet cannot be loaded into the OUT endpoint FIFO. Firmware should clear this bit."]
pub type OVERRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSOL_SPEC, bool, O>;
#[doc = "Field `FIFOFULL` reader - This bit is set when no more packets can be loaded into the OUT endpoint FIFO."]
pub type FIFOFULL_R = crate::BitReader<bool>;
#[doc = "Field `OUTPKTRDY` reader - This bit is set when a data packet has been received. Software should clear this bit when the packet has been unloaded from the OUT endpoint FIFO. An interrupt is generated when the bit is set."]
pub type OUTPKTRDY_R = crate::BitReader<bool>;
#[doc = "Field `OUTPKTRDY` writer - This bit is set when a data packet has been received. Software should clear this bit when the packet has been unloaded from the OUT endpoint FIFO. An interrupt is generated when the bit is set."]
pub type OUTPKTRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSOL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 7 - Software sets this bit to reset the endpoint data toggle to 0."]
    #[inline(always)]
    pub fn clrdatatog(&self) -> CLRDATATOG_R {
        CLRDATATOG_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit is set when a STALL handshake is transmitted. An interrupt is generated when this bit is set. Software should clear this bit."]
    #[inline(always)]
    pub fn sentstall(&self) -> SENTSTALL_R {
        SENTSTALL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - For bulk/interrupt mode OUT endpoints: Software sets this bit to issue a STALL handshake. Software clears this bit to terminate the stall condition."]
    #[inline(always)]
    pub fn sendstall(&self) -> SENDSTALL_R {
        SENDSTALL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Software sets this bit to flush the next packet to be read from the endpoint OUT FIFO. Note: If the FIFO contains two packets, USB_CSOL.FLUSHPACKET will need to be set twice to completely clear the FIFO."]
    #[inline(always)]
    pub fn flushpacket(&self) -> FLUSHPACKET_R {
        FLUSHPACKET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - For isochronous mode OUT endpoints: This bit is set when USB_CSOL.OUTPKTRDY is set if the data packet has a CRC or bit-stuff error. It is cleared automatically when USB_CSOL.OUTPKTRDY is cleared."]
    #[inline(always)]
    pub fn dataerror(&self) -> DATAERROR_R {
        DATAERROR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - For isochronous mode OUT endpoints: This bit is set when an OUT packet cannot be loaded into the OUT endpoint FIFO. Firmware should clear this bit."]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is set when no more packets can be loaded into the OUT endpoint FIFO."]
    #[inline(always)]
    pub fn fifofull(&self) -> FIFOFULL_R {
        FIFOFULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - This bit is set when a data packet has been received. Software should clear this bit when the packet has been unloaded from the OUT endpoint FIFO. An interrupt is generated when the bit is set."]
    #[inline(always)]
    pub fn outpktrdy(&self) -> OUTPKTRDY_R {
        OUTPKTRDY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Software sets this bit to reset the endpoint data toggle to 0."]
    #[inline(always)]
    pub fn clrdatatog(&mut self) -> CLRDATATOG_W<7> {
        CLRDATATOG_W::new(self)
    }
    #[doc = "Bit 6 - This bit is set when a STALL handshake is transmitted. An interrupt is generated when this bit is set. Software should clear this bit."]
    #[inline(always)]
    pub fn sentstall(&mut self) -> SENTSTALL_W<6> {
        SENTSTALL_W::new(self)
    }
    #[doc = "Bit 5 - For bulk/interrupt mode OUT endpoints: Software sets this bit to issue a STALL handshake. Software clears this bit to terminate the stall condition."]
    #[inline(always)]
    pub fn sendstall(&mut self) -> SENDSTALL_W<5> {
        SENDSTALL_W::new(self)
    }
    #[doc = "Bit 4 - Software sets this bit to flush the next packet to be read from the endpoint OUT FIFO. Note: If the FIFO contains two packets, USB_CSOL.FLUSHPACKET will need to be set twice to completely clear the FIFO."]
    #[inline(always)]
    pub fn flushpacket(&mut self) -> FLUSHPACKET_W<4> {
        FLUSHPACKET_W::new(self)
    }
    #[doc = "Bit 2 - For isochronous mode OUT endpoints: This bit is set when an OUT packet cannot be loaded into the OUT endpoint FIFO. Firmware should clear this bit."]
    #[inline(always)]
    pub fn overrun(&mut self) -> OVERRUN_W<2> {
        OVERRUN_W::new(self)
    }
    #[doc = "Bit 0 - This bit is set when a data packet has been received. Software should clear this bit when the packet has been unloaded from the OUT endpoint FIFO. An interrupt is generated when the bit is set."]
    #[inline(always)]
    pub fn outpktrdy(&mut self) -> OUTPKTRDY_W<0> {
        OUTPKTRDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (low byte)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csol](index.html) module"]
pub struct CSOL_SPEC;
impl crate::RegisterSpec for CSOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csol::R](R) reader structure"]
impl crate::Readable for CSOL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csol::W](W) writer structure"]
impl crate::Writable for CSOL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSOL to value 0"]
impl crate::Resettable for CSOL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
