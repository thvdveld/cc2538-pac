#[doc = "Register `CSOL` reader"]
pub type R = crate::R<CsolSpec>;
#[doc = "Register `CSOL` writer"]
pub type W = crate::W<CsolSpec>;
#[doc = "Field `OUTPKTRDY` reader - This bit is set when a data packet has been received. Software should clear this bit when the packet has been unloaded from the OUT endpoint FIFO. An interrupt is generated when the bit is set."]
pub type OutpktrdyR = crate::BitReader;
#[doc = "Field `OUTPKTRDY` writer - This bit is set when a data packet has been received. Software should clear this bit when the packet has been unloaded from the OUT endpoint FIFO. An interrupt is generated when the bit is set."]
pub type OutpktrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOFULL` reader - This bit is set when no more packets can be loaded into the OUT endpoint FIFO."]
pub type FifofullR = crate::BitReader;
#[doc = "Field `OVERRUN` reader - For isochronous mode OUT endpoints: This bit is set when an OUT packet cannot be loaded into the OUT endpoint FIFO. Firmware should clear this bit."]
pub type OverrunR = crate::BitReader;
#[doc = "Field `OVERRUN` writer - For isochronous mode OUT endpoints: This bit is set when an OUT packet cannot be loaded into the OUT endpoint FIFO. Firmware should clear this bit."]
pub type OverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAERROR` reader - For isochronous mode OUT endpoints: This bit is set when USB_CSOL.OUTPKTRDY is set if the data packet has a CRC or bit-stuff error. It is cleared automatically when USB_CSOL.OUTPKTRDY is cleared."]
pub type DataerrorR = crate::BitReader;
#[doc = "Field `FLUSHPACKET` reader - Software sets this bit to flush the next packet to be read from the endpoint OUT FIFO. Note: If the FIFO contains two packets, USB_CSOL.FLUSHPACKET will need to be set twice to completely clear the FIFO."]
pub type FlushpacketR = crate::BitReader;
#[doc = "Field `FLUSHPACKET` writer - Software sets this bit to flush the next packet to be read from the endpoint OUT FIFO. Note: If the FIFO contains two packets, USB_CSOL.FLUSHPACKET will need to be set twice to completely clear the FIFO."]
pub type FlushpacketW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENDSTALL` reader - For bulk/interrupt mode OUT endpoints: Software sets this bit to issue a STALL handshake. Software clears this bit to terminate the stall condition."]
pub type SendstallR = crate::BitReader;
#[doc = "Field `SENDSTALL` writer - For bulk/interrupt mode OUT endpoints: Software sets this bit to issue a STALL handshake. Software clears this bit to terminate the stall condition."]
pub type SendstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENTSTALL` reader - This bit is set when a STALL handshake is transmitted. An interrupt is generated when this bit is set. Software should clear this bit."]
pub type SentstallR = crate::BitReader;
#[doc = "Field `SENTSTALL` writer - This bit is set when a STALL handshake is transmitted. An interrupt is generated when this bit is set. Software should clear this bit."]
pub type SentstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRDATATOG` reader - Software sets this bit to reset the endpoint data toggle to 0."]
pub type ClrdatatogR = crate::BitReader;
#[doc = "Field `CLRDATATOG` writer - Software sets this bit to reset the endpoint data toggle to 0."]
pub type ClrdatatogW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is set when a data packet has been received. Software should clear this bit when the packet has been unloaded from the OUT endpoint FIFO. An interrupt is generated when the bit is set."]
    #[inline(always)]
    pub fn outpktrdy(&self) -> OutpktrdyR {
        OutpktrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is set when no more packets can be loaded into the OUT endpoint FIFO."]
    #[inline(always)]
    pub fn fifofull(&self) -> FifofullR {
        FifofullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - For isochronous mode OUT endpoints: This bit is set when an OUT packet cannot be loaded into the OUT endpoint FIFO. Firmware should clear this bit."]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - For isochronous mode OUT endpoints: This bit is set when USB_CSOL.OUTPKTRDY is set if the data packet has a CRC or bit-stuff error. It is cleared automatically when USB_CSOL.OUTPKTRDY is cleared."]
    #[inline(always)]
    pub fn dataerror(&self) -> DataerrorR {
        DataerrorR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software sets this bit to flush the next packet to be read from the endpoint OUT FIFO. Note: If the FIFO contains two packets, USB_CSOL.FLUSHPACKET will need to be set twice to completely clear the FIFO."]
    #[inline(always)]
    pub fn flushpacket(&self) -> FlushpacketR {
        FlushpacketR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - For bulk/interrupt mode OUT endpoints: Software sets this bit to issue a STALL handshake. Software clears this bit to terminate the stall condition."]
    #[inline(always)]
    pub fn sendstall(&self) -> SendstallR {
        SendstallR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit is set when a STALL handshake is transmitted. An interrupt is generated when this bit is set. Software should clear this bit."]
    #[inline(always)]
    pub fn sentstall(&self) -> SentstallR {
        SentstallR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software sets this bit to reset the endpoint data toggle to 0."]
    #[inline(always)]
    pub fn clrdatatog(&self) -> ClrdatatogR {
        ClrdatatogR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is set when a data packet has been received. Software should clear this bit when the packet has been unloaded from the OUT endpoint FIFO. An interrupt is generated when the bit is set."]
    #[inline(always)]
    pub fn outpktrdy(&mut self) -> OutpktrdyW<CsolSpec> {
        OutpktrdyW::new(self, 0)
    }
    #[doc = "Bit 2 - For isochronous mode OUT endpoints: This bit is set when an OUT packet cannot be loaded into the OUT endpoint FIFO. Firmware should clear this bit."]
    #[inline(always)]
    pub fn overrun(&mut self) -> OverrunW<CsolSpec> {
        OverrunW::new(self, 2)
    }
    #[doc = "Bit 4 - Software sets this bit to flush the next packet to be read from the endpoint OUT FIFO. Note: If the FIFO contains two packets, USB_CSOL.FLUSHPACKET will need to be set twice to completely clear the FIFO."]
    #[inline(always)]
    pub fn flushpacket(&mut self) -> FlushpacketW<CsolSpec> {
        FlushpacketW::new(self, 4)
    }
    #[doc = "Bit 5 - For bulk/interrupt mode OUT endpoints: Software sets this bit to issue a STALL handshake. Software clears this bit to terminate the stall condition."]
    #[inline(always)]
    pub fn sendstall(&mut self) -> SendstallW<CsolSpec> {
        SendstallW::new(self, 5)
    }
    #[doc = "Bit 6 - This bit is set when a STALL handshake is transmitted. An interrupt is generated when this bit is set. Software should clear this bit."]
    #[inline(always)]
    pub fn sentstall(&mut self) -> SentstallW<CsolSpec> {
        SentstallW::new(self, 6)
    }
    #[doc = "Bit 7 - Software sets this bit to reset the endpoint data toggle to 0."]
    #[inline(always)]
    pub fn clrdatatog(&mut self) -> ClrdatatogW<CsolSpec> {
        ClrdatatogW::new(self, 7)
    }
}
#[doc = "Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (low byte)\n\nYou can [`read`](crate::Reg::read) this register and get [`csol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsolSpec;
impl crate::RegisterSpec for CsolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csol::R`](R) reader structure"]
impl crate::Readable for CsolSpec {}
#[doc = "`write(|w| ..)` method takes [`csol::W`](W) writer structure"]
impl crate::Writable for CsolSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSOL to value 0"]
impl crate::Resettable for CsolSpec {
    const RESET_VALUE: u32 = 0;
}
