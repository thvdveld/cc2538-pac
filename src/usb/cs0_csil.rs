#[doc = "Register `CS0_CSIL` reader"]
pub type R = crate::R<Cs0CsilSpec>;
#[doc = "Register `CS0_CSIL` writer"]
pub type W = crate::W<Cs0CsilSpec>;
#[doc = "Field `OUTPKTRDY_or_INPKTRDY` reader - USB_CS0.OUTPKTRDY \\[RO\\]: Endpoint 0 data packet received An interrupt request (EP0) is generated if the interrupt is enabled. Software must read the endpoint 0 FIFO empty, and clear this bit by setting USB_CS0.CLROUTPKTRDY USB_CSIL.INPKTRDY \\[RW\\]: IN endpoint {1-5} packet transfer pending Software sets this bit after loading a data packet into the FIFO. It is cleared automatically when a data packet has been transmitted. An interrupt is generated (if enabled) when the bit is cleared. When using double-buffering, the bit is cleared immediately if the other FIFO is empty."]
pub type OutpktrdyOrInpktrdyR = crate::BitReader;
#[doc = "Field `INPKTRDY_or_PKTPRESENT` reader - USB_CS0. INPKTRDY \\[RW\\]: Software sets this bit after loading a data packet into the endpoint 0 FIFO. It is cleared automatically when the data packet has been transmitted. An interrupt is generated when the bit is cleared. USB_CSIL.PKTPRESENT \\[RO\\]: This bit is set when there is at least one packet in the IN endpoint FIFO."]
pub type InpktrdyOrPktpresentR = crate::BitReader;
#[doc = "Field `INPKTRDY_or_PKTPRESENT` writer - USB_CS0. INPKTRDY \\[RW\\]: Software sets this bit after loading a data packet into the endpoint 0 FIFO. It is cleared automatically when the data packet has been transmitted. An interrupt is generated when the bit is cleared. USB_CSIL.PKTPRESENT \\[RO\\]: This bit is set when there is at least one packet in the IN endpoint FIFO."]
pub type InpktrdyOrPktpresentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENTSTALL_or_UNDERRUN` reader - USB_CS0.SENTSTALL \\[RW\\]: This bit is set when a STALL handshake is sent. An interrupt is generated is generated when this bit is set. Software must clear this bit. USB_CSIL.UNDERRUN \\[RW\\]: In isochronous mode, this bit is set when a zero length data packet is sent after receiving an IN token with USB_CSIL.INPKTRDY not set. In bulk/interrupt mode, this bit is set when a NAK is returned in response to an IN token. Software should clear this bit."]
pub type SentstallOrUnderrunR = crate::BitReader;
#[doc = "Field `SENTSTALL_or_UNDERRUN` writer - USB_CS0.SENTSTALL \\[RW\\]: This bit is set when a STALL handshake is sent. An interrupt is generated is generated when this bit is set. Software must clear this bit. USB_CSIL.UNDERRUN \\[RW\\]: In isochronous mode, this bit is set when a zero length data packet is sent after receiving an IN token with USB_CSIL.INPKTRDY not set. In bulk/interrupt mode, this bit is set when a NAK is returned in response to an IN token. Software should clear this bit."]
pub type SentstallOrUnderrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAEND_or_FLUSHPACKET` reader - USB_CS0.DATAEND \\[RW\\]: This bit is used to signal the end of the data stage, and must be set: 1. When the last data packet is loaded and USB_CS0.INPKTRDY is set. 2. When the last data packet is unloaded and USB_CS0.CLROUTPKTRDY is set. 3. When USB_CS0.INPKTRDY is set to send a zero-length packet. The USB controller clears this bit automatically. USB_CSIL.FLUSHPACKET \\[RW\\]: Software sets this bit to flush the next packet to be transmitted from the IN endpoint FIFO. The FIFO pointer is reset and the USB_CSIL.INPKTRDY bit is cleared. Note: If the FIFO contains two packets, USB_CSIL.FLUSHPACKET will need to be set twice to completely clear the FIFO."]
pub type DataendOrFlushpacketR = crate::BitReader;
#[doc = "Field `DATAEND_or_FLUSHPACKET` writer - USB_CS0.DATAEND \\[RW\\]: This bit is used to signal the end of the data stage, and must be set: 1. When the last data packet is loaded and USB_CS0.INPKTRDY is set. 2. When the last data packet is unloaded and USB_CS0.CLROUTPKTRDY is set. 3. When USB_CS0.INPKTRDY is set to send a zero-length packet. The USB controller clears this bit automatically. USB_CSIL.FLUSHPACKET \\[RW\\]: Software sets this bit to flush the next packet to be transmitted from the IN endpoint FIFO. The FIFO pointer is reset and the USB_CSIL.INPKTRDY bit is cleared. Note: If the FIFO contains two packets, USB_CSIL.FLUSHPACKET will need to be set twice to completely clear the FIFO."]
pub type DataendOrFlushpacketW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETUPEND_or_SENDSTALL` reader - USB_CS0.SETUPEND \\[RO\\]: This bit is set when a control transaction ends before the USB_CS0.DATAEND bit has been set. An interrupt is generated and the FIFO flushed at this time. Software clears this bit by setting USB_CS0.CLRSETUPEND. CSIL.SENDSTALL \\[RW\\]: For bulk/interrupt mode IN endpoints: Software sets this bit to issue a STALL handshake. Software clears this bit to terminate the stall condition."]
pub type SetupendOrSendstallR = crate::BitReader;
#[doc = "Field `SENDSTALL_or_SENTSTALL` reader - USB_CS0.SENDSTALL \\[RW\\]: Software sets this bit to terminate the current transaction with a STALL handshake. The bit is cleared automatically when the STALL handshake has been transmitted. USB_CSIL.SENTSTALL \\[RW\\]: For bulk/interrupt mode IN endpoints: This bit is set when a STALL handshake is transmitted. The FIFO is flushed and the USB_CSIL.INPKTRDY bit cleared. Software should clear this bit."]
pub type SendstallOrSentstallR = crate::BitReader;
#[doc = "Field `SENDSTALL_or_SENTSTALL` writer - USB_CS0.SENDSTALL \\[RW\\]: Software sets this bit to terminate the current transaction with a STALL handshake. The bit is cleared automatically when the STALL handshake has been transmitted. USB_CSIL.SENTSTALL \\[RW\\]: For bulk/interrupt mode IN endpoints: This bit is set when a STALL handshake is transmitted. The FIFO is flushed and the USB_CSIL.INPKTRDY bit cleared. Software should clear this bit."]
pub type SendstallOrSentstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLROUTPKTRDY_or_CLRDATATOG` reader - USB_CS0.CLROUTPKTRDY \\[RW\\]: Software sets this bit to clear the USB_CS0.OUTPKTRDY bit. It is cleared automatically. USB_CSIL.CLRDATATOG \\[RW\\]: Software sets this bit to reset the IN endpoint data toggle to 0."]
pub type ClroutpktrdyOrClrdatatogR = crate::BitReader;
#[doc = "Field `CLROUTPKTRDY_or_CLRDATATOG` writer - USB_CS0.CLROUTPKTRDY \\[RW\\]: Software sets this bit to clear the USB_CS0.OUTPKTRDY bit. It is cleared automatically. USB_CSIL.CLRDATATOG \\[RW\\]: Software sets this bit to reset the IN endpoint data toggle to 0."]
pub type ClroutpktrdyOrClrdatatogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRSETUPEND_or_Reserved8` reader - USB_CS0.CLRSETUPEND \\[RW\\]: Software sets this bit to clear the USB_CS0.SETUPEND bit. It is cleared automatically. USB_CSIL.Reserved \\[RO\\]: Reserved"]
pub type ClrsetupendOrReserved8R = crate::BitReader;
#[doc = "Field `CLRSETUPEND_or_Reserved8` writer - USB_CS0.CLRSETUPEND \\[RW\\]: Software sets this bit to clear the USB_CS0.SETUPEND bit. It is cleared automatically. USB_CSIL.Reserved \\[RO\\]: Reserved"]
pub type ClrsetupendOrReserved8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB_CS0.OUTPKTRDY \\[RO\\]: Endpoint 0 data packet received An interrupt request (EP0) is generated if the interrupt is enabled. Software must read the endpoint 0 FIFO empty, and clear this bit by setting USB_CS0.CLROUTPKTRDY USB_CSIL.INPKTRDY \\[RW\\]: IN endpoint {1-5} packet transfer pending Software sets this bit after loading a data packet into the FIFO. It is cleared automatically when a data packet has been transmitted. An interrupt is generated (if enabled) when the bit is cleared. When using double-buffering, the bit is cleared immediately if the other FIFO is empty."]
    #[inline(always)]
    pub fn outpktrdy_or_inpktrdy(&self) -> OutpktrdyOrInpktrdyR {
        OutpktrdyOrInpktrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB_CS0. INPKTRDY \\[RW\\]: Software sets this bit after loading a data packet into the endpoint 0 FIFO. It is cleared automatically when the data packet has been transmitted. An interrupt is generated when the bit is cleared. USB_CSIL.PKTPRESENT \\[RO\\]: This bit is set when there is at least one packet in the IN endpoint FIFO."]
    #[inline(always)]
    pub fn inpktrdy_or_pktpresent(&self) -> InpktrdyOrPktpresentR {
        InpktrdyOrPktpresentR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB_CS0.SENTSTALL \\[RW\\]: This bit is set when a STALL handshake is sent. An interrupt is generated is generated when this bit is set. Software must clear this bit. USB_CSIL.UNDERRUN \\[RW\\]: In isochronous mode, this bit is set when a zero length data packet is sent after receiving an IN token with USB_CSIL.INPKTRDY not set. In bulk/interrupt mode, this bit is set when a NAK is returned in response to an IN token. Software should clear this bit."]
    #[inline(always)]
    pub fn sentstall_or_underrun(&self) -> SentstallOrUnderrunR {
        SentstallOrUnderrunR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB_CS0.DATAEND \\[RW\\]: This bit is used to signal the end of the data stage, and must be set: 1. When the last data packet is loaded and USB_CS0.INPKTRDY is set. 2. When the last data packet is unloaded and USB_CS0.CLROUTPKTRDY is set. 3. When USB_CS0.INPKTRDY is set to send a zero-length packet. The USB controller clears this bit automatically. USB_CSIL.FLUSHPACKET \\[RW\\]: Software sets this bit to flush the next packet to be transmitted from the IN endpoint FIFO. The FIFO pointer is reset and the USB_CSIL.INPKTRDY bit is cleared. Note: If the FIFO contains two packets, USB_CSIL.FLUSHPACKET will need to be set twice to completely clear the FIFO."]
    #[inline(always)]
    pub fn dataend_or_flushpacket(&self) -> DataendOrFlushpacketR {
        DataendOrFlushpacketR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB_CS0.SETUPEND \\[RO\\]: This bit is set when a control transaction ends before the USB_CS0.DATAEND bit has been set. An interrupt is generated and the FIFO flushed at this time. Software clears this bit by setting USB_CS0.CLRSETUPEND. CSIL.SENDSTALL \\[RW\\]: For bulk/interrupt mode IN endpoints: Software sets this bit to issue a STALL handshake. Software clears this bit to terminate the stall condition."]
    #[inline(always)]
    pub fn setupend_or_sendstall(&self) -> SetupendOrSendstallR {
        SetupendOrSendstallR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB_CS0.SENDSTALL \\[RW\\]: Software sets this bit to terminate the current transaction with a STALL handshake. The bit is cleared automatically when the STALL handshake has been transmitted. USB_CSIL.SENTSTALL \\[RW\\]: For bulk/interrupt mode IN endpoints: This bit is set when a STALL handshake is transmitted. The FIFO is flushed and the USB_CSIL.INPKTRDY bit cleared. Software should clear this bit."]
    #[inline(always)]
    pub fn sendstall_or_sentstall(&self) -> SendstallOrSentstallR {
        SendstallOrSentstallR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB_CS0.CLROUTPKTRDY \\[RW\\]: Software sets this bit to clear the USB_CS0.OUTPKTRDY bit. It is cleared automatically. USB_CSIL.CLRDATATOG \\[RW\\]: Software sets this bit to reset the IN endpoint data toggle to 0."]
    #[inline(always)]
    pub fn clroutpktrdy_or_clrdatatog(&self) -> ClroutpktrdyOrClrdatatogR {
        ClroutpktrdyOrClrdatatogR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB_CS0.CLRSETUPEND \\[RW\\]: Software sets this bit to clear the USB_CS0.SETUPEND bit. It is cleared automatically. USB_CSIL.Reserved \\[RO\\]: Reserved"]
    #[inline(always)]
    pub fn clrsetupend_or_reserved8(&self) -> ClrsetupendOrReserved8R {
        ClrsetupendOrReserved8R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - USB_CS0. INPKTRDY \\[RW\\]: Software sets this bit after loading a data packet into the endpoint 0 FIFO. It is cleared automatically when the data packet has been transmitted. An interrupt is generated when the bit is cleared. USB_CSIL.PKTPRESENT \\[RO\\]: This bit is set when there is at least one packet in the IN endpoint FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn inpktrdy_or_pktpresent(&mut self) -> InpktrdyOrPktpresentW<Cs0CsilSpec> {
        InpktrdyOrPktpresentW::new(self, 1)
    }
    #[doc = "Bit 2 - USB_CS0.SENTSTALL \\[RW\\]: This bit is set when a STALL handshake is sent. An interrupt is generated is generated when this bit is set. Software must clear this bit. USB_CSIL.UNDERRUN \\[RW\\]: In isochronous mode, this bit is set when a zero length data packet is sent after receiving an IN token with USB_CSIL.INPKTRDY not set. In bulk/interrupt mode, this bit is set when a NAK is returned in response to an IN token. Software should clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn sentstall_or_underrun(&mut self) -> SentstallOrUnderrunW<Cs0CsilSpec> {
        SentstallOrUnderrunW::new(self, 2)
    }
    #[doc = "Bit 3 - USB_CS0.DATAEND \\[RW\\]: This bit is used to signal the end of the data stage, and must be set: 1. When the last data packet is loaded and USB_CS0.INPKTRDY is set. 2. When the last data packet is unloaded and USB_CS0.CLROUTPKTRDY is set. 3. When USB_CS0.INPKTRDY is set to send a zero-length packet. The USB controller clears this bit automatically. USB_CSIL.FLUSHPACKET \\[RW\\]: Software sets this bit to flush the next packet to be transmitted from the IN endpoint FIFO. The FIFO pointer is reset and the USB_CSIL.INPKTRDY bit is cleared. Note: If the FIFO contains two packets, USB_CSIL.FLUSHPACKET will need to be set twice to completely clear the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn dataend_or_flushpacket(&mut self) -> DataendOrFlushpacketW<Cs0CsilSpec> {
        DataendOrFlushpacketW::new(self, 3)
    }
    #[doc = "Bit 5 - USB_CS0.SENDSTALL \\[RW\\]: Software sets this bit to terminate the current transaction with a STALL handshake. The bit is cleared automatically when the STALL handshake has been transmitted. USB_CSIL.SENTSTALL \\[RW\\]: For bulk/interrupt mode IN endpoints: This bit is set when a STALL handshake is transmitted. The FIFO is flushed and the USB_CSIL.INPKTRDY bit cleared. Software should clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn sendstall_or_sentstall(&mut self) -> SendstallOrSentstallW<Cs0CsilSpec> {
        SendstallOrSentstallW::new(self, 5)
    }
    #[doc = "Bit 6 - USB_CS0.CLROUTPKTRDY \\[RW\\]: Software sets this bit to clear the USB_CS0.OUTPKTRDY bit. It is cleared automatically. USB_CSIL.CLRDATATOG \\[RW\\]: Software sets this bit to reset the IN endpoint data toggle to 0."]
    #[inline(always)]
    #[must_use]
    pub fn clroutpktrdy_or_clrdatatog(&mut self) -> ClroutpktrdyOrClrdatatogW<Cs0CsilSpec> {
        ClroutpktrdyOrClrdatatogW::new(self, 6)
    }
    #[doc = "Bit 7 - USB_CS0.CLRSETUPEND \\[RW\\]: Software sets this bit to clear the USB_CS0.SETUPEND bit. It is cleared automatically. USB_CSIL.Reserved \\[RO\\]: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn clrsetupend_or_reserved8(&mut self) -> ClrsetupendOrReserved8W<Cs0CsilSpec> {
        ClrsetupendOrReserved8W::new(self, 7)
    }
}
#[doc = "Indexed register: For USB_INDEX = 0: Endpoint 0 control and status For USB_INDEX = 1-5: IN endpoint {1-5} control and status (low byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cs0_csil::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs0_csil::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cs0CsilSpec;
impl crate::RegisterSpec for Cs0CsilSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cs0_csil::R`](R) reader structure"]
impl crate::Readable for Cs0CsilSpec {}
#[doc = "`write(|w| ..)` method takes [`cs0_csil::W`](W) writer structure"]
impl crate::Writable for Cs0CsilSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CS0_CSIL to value 0"]
impl crate::Resettable for Cs0CsilSpec {
    const RESET_VALUE: u32 = 0;
}
