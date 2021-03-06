#[doc = "Register `CS0_CSIL` reader"]
pub struct R(crate::R<CS0_CSIL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CS0_CSIL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CS0_CSIL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CS0_CSIL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CS0_CSIL` writer"]
pub struct W(crate::W<CS0_CSIL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CS0_CSIL_SPEC>;
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
impl From<crate::W<CS0_CSIL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CS0_CSIL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLRSETUPEND_or_Reserved8` reader - USB_CS0.CLRSETUPEND \\[RW\\]: Software sets this bit to clear the USB_CS0.SETUPEND bit. It is cleared automatically. USB_CSIL.Reserved \\[RO\\]: Reserved"]
pub type CLRSETUPEND_OR_RESERVED8_R = crate::BitReader<bool>;
#[doc = "Field `CLRSETUPEND_or_Reserved8` writer - USB_CS0.CLRSETUPEND \\[RW\\]: Software sets this bit to clear the USB_CS0.SETUPEND bit. It is cleared automatically. USB_CSIL.Reserved \\[RO\\]: Reserved"]
pub type CLRSETUPEND_OR_RESERVED8_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CS0_CSIL_SPEC, bool, O>;
#[doc = "Field `CLROUTPKTRDY_or_CLRDATATOG` reader - USB_CS0.CLROUTPKTRDY \\[RW\\]: Software sets this bit to clear the USB_CS0.OUTPKTRDY bit. It is cleared automatically. USB_CSIL.CLRDATATOG \\[RW\\]: Software sets this bit to reset the IN endpoint data toggle to 0."]
pub type CLROUTPKTRDY_OR_CLRDATATOG_R = crate::BitReader<bool>;
#[doc = "Field `CLROUTPKTRDY_or_CLRDATATOG` writer - USB_CS0.CLROUTPKTRDY \\[RW\\]: Software sets this bit to clear the USB_CS0.OUTPKTRDY bit. It is cleared automatically. USB_CSIL.CLRDATATOG \\[RW\\]: Software sets this bit to reset the IN endpoint data toggle to 0."]
pub type CLROUTPKTRDY_OR_CLRDATATOG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CS0_CSIL_SPEC, bool, O>;
#[doc = "Field `SENDSTALL_or_SENTSTALL` reader - USB_CS0.SENDSTALL \\[RW\\]: Software sets this bit to terminate the current transaction with a STALL handshake. The bit is cleared automatically when the STALL handshake has been transmitted. USB_CSIL.SENTSTALL \\[RW\\]: For bulk/interrupt mode IN endpoints: This bit is set when a STALL handshake is transmitted. The FIFO is flushed and the USB_CSIL.INPKTRDY bit cleared. Software should clear this bit."]
pub type SENDSTALL_OR_SENTSTALL_R = crate::BitReader<bool>;
#[doc = "Field `SENDSTALL_or_SENTSTALL` writer - USB_CS0.SENDSTALL \\[RW\\]: Software sets this bit to terminate the current transaction with a STALL handshake. The bit is cleared automatically when the STALL handshake has been transmitted. USB_CSIL.SENTSTALL \\[RW\\]: For bulk/interrupt mode IN endpoints: This bit is set when a STALL handshake is transmitted. The FIFO is flushed and the USB_CSIL.INPKTRDY bit cleared. Software should clear this bit."]
pub type SENDSTALL_OR_SENTSTALL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CS0_CSIL_SPEC, bool, O>;
#[doc = "Field `SETUPEND_or_SENDSTALL` reader - USB_CS0.SETUPEND \\[RO\\]: This bit is set when a control transaction ends before the USB_CS0.DATAEND bit has been set. An interrupt is generated and the FIFO flushed at this time. Software clears this bit by setting USB_CS0.CLRSETUPEND. CSIL.SENDSTALL \\[RW\\]: For bulk/interrupt mode IN endpoints: Software sets this bit to issue a STALL handshake. Software clears this bit to terminate the stall condition."]
pub type SETUPEND_OR_SENDSTALL_R = crate::BitReader<bool>;
#[doc = "Field `DATAEND_or_FLUSHPACKET` reader - USB_CS0.DATAEND \\[RW\\]: This bit is used to signal the end of the data stage, and must be set: 1. When the last data packet is loaded and USB_CS0.INPKTRDY is set. 2. When the last data packet is unloaded and USB_CS0.CLROUTPKTRDY is set. 3. When USB_CS0.INPKTRDY is set to send a zero-length packet. The USB controller clears this bit automatically. USB_CSIL.FLUSHPACKET \\[RW\\]: Software sets this bit to flush the next packet to be transmitted from the IN endpoint FIFO. The FIFO pointer is reset and the USB_CSIL.INPKTRDY bit is cleared. Note: If the FIFO contains two packets, USB_CSIL.FLUSHPACKET will need to be set twice to completely clear the FIFO."]
pub type DATAEND_OR_FLUSHPACKET_R = crate::BitReader<bool>;
#[doc = "Field `DATAEND_or_FLUSHPACKET` writer - USB_CS0.DATAEND \\[RW\\]: This bit is used to signal the end of the data stage, and must be set: 1. When the last data packet is loaded and USB_CS0.INPKTRDY is set. 2. When the last data packet is unloaded and USB_CS0.CLROUTPKTRDY is set. 3. When USB_CS0.INPKTRDY is set to send a zero-length packet. The USB controller clears this bit automatically. USB_CSIL.FLUSHPACKET \\[RW\\]: Software sets this bit to flush the next packet to be transmitted from the IN endpoint FIFO. The FIFO pointer is reset and the USB_CSIL.INPKTRDY bit is cleared. Note: If the FIFO contains two packets, USB_CSIL.FLUSHPACKET will need to be set twice to completely clear the FIFO."]
pub type DATAEND_OR_FLUSHPACKET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CS0_CSIL_SPEC, bool, O>;
#[doc = "Field `SENTSTALL_or_UNDERRUN` reader - USB_CS0.SENTSTALL \\[RW\\]: This bit is set when a STALL handshake is sent. An interrupt is generated is generated when this bit is set. Software must clear this bit. USB_CSIL.UNDERRUN \\[RW\\]: In isochronous mode, this bit is set when a zero length data packet is sent after receiving an IN token with USB_CSIL.INPKTRDY not set. In bulk/interrupt mode, this bit is set when a NAK is returned in response to an IN token. Software should clear this bit."]
pub type SENTSTALL_OR_UNDERRUN_R = crate::BitReader<bool>;
#[doc = "Field `SENTSTALL_or_UNDERRUN` writer - USB_CS0.SENTSTALL \\[RW\\]: This bit is set when a STALL handshake is sent. An interrupt is generated is generated when this bit is set. Software must clear this bit. USB_CSIL.UNDERRUN \\[RW\\]: In isochronous mode, this bit is set when a zero length data packet is sent after receiving an IN token with USB_CSIL.INPKTRDY not set. In bulk/interrupt mode, this bit is set when a NAK is returned in response to an IN token. Software should clear this bit."]
pub type SENTSTALL_OR_UNDERRUN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CS0_CSIL_SPEC, bool, O>;
#[doc = "Field `INPKTRDY_or_PKTPRESENT` reader - USB_CS0. INPKTRDY \\[RW\\]: Software sets this bit after loading a data packet into the endpoint 0 FIFO. It is cleared automatically when the data packet has been transmitted. An interrupt is generated when the bit is cleared. USB_CSIL.PKTPRESENT \\[RO\\]: This bit is set when there is at least one packet in the IN endpoint FIFO."]
pub type INPKTRDY_OR_PKTPRESENT_R = crate::BitReader<bool>;
#[doc = "Field `INPKTRDY_or_PKTPRESENT` writer - USB_CS0. INPKTRDY \\[RW\\]: Software sets this bit after loading a data packet into the endpoint 0 FIFO. It is cleared automatically when the data packet has been transmitted. An interrupt is generated when the bit is cleared. USB_CSIL.PKTPRESENT \\[RO\\]: This bit is set when there is at least one packet in the IN endpoint FIFO."]
pub type INPKTRDY_OR_PKTPRESENT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CS0_CSIL_SPEC, bool, O>;
#[doc = "Field `OUTPKTRDY_or_INPKTRDY` reader - USB_CS0.OUTPKTRDY \\[RO\\]: Endpoint 0 data packet received An interrupt request (EP0) is generated if the interrupt is enabled. Software must read the endpoint 0 FIFO empty, and clear this bit by setting USB_CS0.CLROUTPKTRDY USB_CSIL.INPKTRDY \\[RW\\]: IN endpoint {1-5} packet transfer pending Software sets this bit after loading a data packet into the FIFO. It is cleared automatically when a data packet has been transmitted. An interrupt is generated (if enabled) when the bit is cleared. When using double-buffering, the bit is cleared immediately if the other FIFO is empty."]
pub type OUTPKTRDY_OR_INPKTRDY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 7 - USB_CS0.CLRSETUPEND \\[RW\\]: Software sets this bit to clear the USB_CS0.SETUPEND bit. It is cleared automatically. USB_CSIL.Reserved \\[RO\\]: Reserved"]
    #[inline(always)]
    pub fn clrsetupend_or_reserved8(&self) -> CLRSETUPEND_OR_RESERVED8_R {
        CLRSETUPEND_OR_RESERVED8_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - USB_CS0.CLROUTPKTRDY \\[RW\\]: Software sets this bit to clear the USB_CS0.OUTPKTRDY bit. It is cleared automatically. USB_CSIL.CLRDATATOG \\[RW\\]: Software sets this bit to reset the IN endpoint data toggle to 0."]
    #[inline(always)]
    pub fn clroutpktrdy_or_clrdatatog(&self) -> CLROUTPKTRDY_OR_CLRDATATOG_R {
        CLROUTPKTRDY_OR_CLRDATATOG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - USB_CS0.SENDSTALL \\[RW\\]: Software sets this bit to terminate the current transaction with a STALL handshake. The bit is cleared automatically when the STALL handshake has been transmitted. USB_CSIL.SENTSTALL \\[RW\\]: For bulk/interrupt mode IN endpoints: This bit is set when a STALL handshake is transmitted. The FIFO is flushed and the USB_CSIL.INPKTRDY bit cleared. Software should clear this bit."]
    #[inline(always)]
    pub fn sendstall_or_sentstall(&self) -> SENDSTALL_OR_SENTSTALL_R {
        SENDSTALL_OR_SENTSTALL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - USB_CS0.SETUPEND \\[RO\\]: This bit is set when a control transaction ends before the USB_CS0.DATAEND bit has been set. An interrupt is generated and the FIFO flushed at this time. Software clears this bit by setting USB_CS0.CLRSETUPEND. CSIL.SENDSTALL \\[RW\\]: For bulk/interrupt mode IN endpoints: Software sets this bit to issue a STALL handshake. Software clears this bit to terminate the stall condition."]
    #[inline(always)]
    pub fn setupend_or_sendstall(&self) -> SETUPEND_OR_SENDSTALL_R {
        SETUPEND_OR_SENDSTALL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - USB_CS0.DATAEND \\[RW\\]: This bit is used to signal the end of the data stage, and must be set: 1. When the last data packet is loaded and USB_CS0.INPKTRDY is set. 2. When the last data packet is unloaded and USB_CS0.CLROUTPKTRDY is set. 3. When USB_CS0.INPKTRDY is set to send a zero-length packet. The USB controller clears this bit automatically. USB_CSIL.FLUSHPACKET \\[RW\\]: Software sets this bit to flush the next packet to be transmitted from the IN endpoint FIFO. The FIFO pointer is reset and the USB_CSIL.INPKTRDY bit is cleared. Note: If the FIFO contains two packets, USB_CSIL.FLUSHPACKET will need to be set twice to completely clear the FIFO."]
    #[inline(always)]
    pub fn dataend_or_flushpacket(&self) -> DATAEND_OR_FLUSHPACKET_R {
        DATAEND_OR_FLUSHPACKET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - USB_CS0.SENTSTALL \\[RW\\]: This bit is set when a STALL handshake is sent. An interrupt is generated is generated when this bit is set. Software must clear this bit. USB_CSIL.UNDERRUN \\[RW\\]: In isochronous mode, this bit is set when a zero length data packet is sent after receiving an IN token with USB_CSIL.INPKTRDY not set. In bulk/interrupt mode, this bit is set when a NAK is returned in response to an IN token. Software should clear this bit."]
    #[inline(always)]
    pub fn sentstall_or_underrun(&self) -> SENTSTALL_OR_UNDERRUN_R {
        SENTSTALL_OR_UNDERRUN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - USB_CS0. INPKTRDY \\[RW\\]: Software sets this bit after loading a data packet into the endpoint 0 FIFO. It is cleared automatically when the data packet has been transmitted. An interrupt is generated when the bit is cleared. USB_CSIL.PKTPRESENT \\[RO\\]: This bit is set when there is at least one packet in the IN endpoint FIFO."]
    #[inline(always)]
    pub fn inpktrdy_or_pktpresent(&self) -> INPKTRDY_OR_PKTPRESENT_R {
        INPKTRDY_OR_PKTPRESENT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - USB_CS0.OUTPKTRDY \\[RO\\]: Endpoint 0 data packet received An interrupt request (EP0) is generated if the interrupt is enabled. Software must read the endpoint 0 FIFO empty, and clear this bit by setting USB_CS0.CLROUTPKTRDY USB_CSIL.INPKTRDY \\[RW\\]: IN endpoint {1-5} packet transfer pending Software sets this bit after loading a data packet into the FIFO. It is cleared automatically when a data packet has been transmitted. An interrupt is generated (if enabled) when the bit is cleared. When using double-buffering, the bit is cleared immediately if the other FIFO is empty."]
    #[inline(always)]
    pub fn outpktrdy_or_inpktrdy(&self) -> OUTPKTRDY_OR_INPKTRDY_R {
        OUTPKTRDY_OR_INPKTRDY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - USB_CS0.CLRSETUPEND \\[RW\\]: Software sets this bit to clear the USB_CS0.SETUPEND bit. It is cleared automatically. USB_CSIL.Reserved \\[RO\\]: Reserved"]
    #[inline(always)]
    pub fn clrsetupend_or_reserved8(&mut self) -> CLRSETUPEND_OR_RESERVED8_W<7> {
        CLRSETUPEND_OR_RESERVED8_W::new(self)
    }
    #[doc = "Bit 6 - USB_CS0.CLROUTPKTRDY \\[RW\\]: Software sets this bit to clear the USB_CS0.OUTPKTRDY bit. It is cleared automatically. USB_CSIL.CLRDATATOG \\[RW\\]: Software sets this bit to reset the IN endpoint data toggle to 0."]
    #[inline(always)]
    pub fn clroutpktrdy_or_clrdatatog(&mut self) -> CLROUTPKTRDY_OR_CLRDATATOG_W<6> {
        CLROUTPKTRDY_OR_CLRDATATOG_W::new(self)
    }
    #[doc = "Bit 5 - USB_CS0.SENDSTALL \\[RW\\]: Software sets this bit to terminate the current transaction with a STALL handshake. The bit is cleared automatically when the STALL handshake has been transmitted. USB_CSIL.SENTSTALL \\[RW\\]: For bulk/interrupt mode IN endpoints: This bit is set when a STALL handshake is transmitted. The FIFO is flushed and the USB_CSIL.INPKTRDY bit cleared. Software should clear this bit."]
    #[inline(always)]
    pub fn sendstall_or_sentstall(&mut self) -> SENDSTALL_OR_SENTSTALL_W<5> {
        SENDSTALL_OR_SENTSTALL_W::new(self)
    }
    #[doc = "Bit 3 - USB_CS0.DATAEND \\[RW\\]: This bit is used to signal the end of the data stage, and must be set: 1. When the last data packet is loaded and USB_CS0.INPKTRDY is set. 2. When the last data packet is unloaded and USB_CS0.CLROUTPKTRDY is set. 3. When USB_CS0.INPKTRDY is set to send a zero-length packet. The USB controller clears this bit automatically. USB_CSIL.FLUSHPACKET \\[RW\\]: Software sets this bit to flush the next packet to be transmitted from the IN endpoint FIFO. The FIFO pointer is reset and the USB_CSIL.INPKTRDY bit is cleared. Note: If the FIFO contains two packets, USB_CSIL.FLUSHPACKET will need to be set twice to completely clear the FIFO."]
    #[inline(always)]
    pub fn dataend_or_flushpacket(&mut self) -> DATAEND_OR_FLUSHPACKET_W<3> {
        DATAEND_OR_FLUSHPACKET_W::new(self)
    }
    #[doc = "Bit 2 - USB_CS0.SENTSTALL \\[RW\\]: This bit is set when a STALL handshake is sent. An interrupt is generated is generated when this bit is set. Software must clear this bit. USB_CSIL.UNDERRUN \\[RW\\]: In isochronous mode, this bit is set when a zero length data packet is sent after receiving an IN token with USB_CSIL.INPKTRDY not set. In bulk/interrupt mode, this bit is set when a NAK is returned in response to an IN token. Software should clear this bit."]
    #[inline(always)]
    pub fn sentstall_or_underrun(&mut self) -> SENTSTALL_OR_UNDERRUN_W<2> {
        SENTSTALL_OR_UNDERRUN_W::new(self)
    }
    #[doc = "Bit 1 - USB_CS0. INPKTRDY \\[RW\\]: Software sets this bit after loading a data packet into the endpoint 0 FIFO. It is cleared automatically when the data packet has been transmitted. An interrupt is generated when the bit is cleared. USB_CSIL.PKTPRESENT \\[RO\\]: This bit is set when there is at least one packet in the IN endpoint FIFO."]
    #[inline(always)]
    pub fn inpktrdy_or_pktpresent(&mut self) -> INPKTRDY_OR_PKTPRESENT_W<1> {
        INPKTRDY_OR_PKTPRESENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indexed register: For USB_INDEX = 0: Endpoint 0 control and status For USB_INDEX = 1-5: IN endpoint {1-5} control and status (low byte)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cs0_csil](index.html) module"]
pub struct CS0_CSIL_SPEC;
impl crate::RegisterSpec for CS0_CSIL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cs0_csil::R](R) reader structure"]
impl crate::Readable for CS0_CSIL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cs0_csil::W](W) writer structure"]
impl crate::Writable for CS0_CSIL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CS0_CSIL to value 0"]
impl crate::Resettable for CS0_CSIL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
