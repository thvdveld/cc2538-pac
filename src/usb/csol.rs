#[doc = "Register `CSOL` reader"]
pub type R = crate::R<CSOL_SPEC>;
#[doc = "Register `CSOL` writer"]
pub type W = crate::W<CSOL_SPEC>;
#[doc = "Field `OUTPKTRDY` reader - This bit is set when a data packet has been received. Software should clear this bit when the packet has been unloaded from the OUT endpoint FIFO. An interrupt is generated when the bit is set."]
pub type OUTPKTRDY_R = crate::BitReader;
#[doc = "Field `OUTPKTRDY` writer - This bit is set when a data packet has been received. Software should clear this bit when the packet has been unloaded from the OUT endpoint FIFO. An interrupt is generated when the bit is set."]
pub type OUTPKTRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOFULL` reader - This bit is set when no more packets can be loaded into the OUT endpoint FIFO."]
pub type FIFOFULL_R = crate::BitReader;
#[doc = "Field `OVERRUN` reader - For isochronous mode OUT endpoints: This bit is set when an OUT packet cannot be loaded into the OUT endpoint FIFO. Firmware should clear this bit."]
pub type OVERRUN_R = crate::BitReader;
#[doc = "Field `OVERRUN` writer - For isochronous mode OUT endpoints: This bit is set when an OUT packet cannot be loaded into the OUT endpoint FIFO. Firmware should clear this bit."]
pub type OVERRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAERROR` reader - For isochronous mode OUT endpoints: This bit is set when USB_CSOL.OUTPKTRDY is set if the data packet has a CRC or bit-stuff error. It is cleared automatically when USB_CSOL.OUTPKTRDY is cleared."]
pub type DATAERROR_R = crate::BitReader;
#[doc = "Field `FLUSHPACKET` reader - Software sets this bit to flush the next packet to be read from the endpoint OUT FIFO. Note: If the FIFO contains two packets, USB_CSOL.FLUSHPACKET will need to be set twice to completely clear the FIFO."]
pub type FLUSHPACKET_R = crate::BitReader;
#[doc = "Field `FLUSHPACKET` writer - Software sets this bit to flush the next packet to be read from the endpoint OUT FIFO. Note: If the FIFO contains two packets, USB_CSOL.FLUSHPACKET will need to be set twice to completely clear the FIFO."]
pub type FLUSHPACKET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENDSTALL` reader - For bulk/interrupt mode OUT endpoints: Software sets this bit to issue a STALL handshake. Software clears this bit to terminate the stall condition."]
pub type SENDSTALL_R = crate::BitReader;
#[doc = "Field `SENDSTALL` writer - For bulk/interrupt mode OUT endpoints: Software sets this bit to issue a STALL handshake. Software clears this bit to terminate the stall condition."]
pub type SENDSTALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENTSTALL` reader - This bit is set when a STALL handshake is transmitted. An interrupt is generated when this bit is set. Software should clear this bit."]
pub type SENTSTALL_R = crate::BitReader;
#[doc = "Field `SENTSTALL` writer - This bit is set when a STALL handshake is transmitted. An interrupt is generated when this bit is set. Software should clear this bit."]
pub type SENTSTALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRDATATOG` reader - Software sets this bit to reset the endpoint data toggle to 0."]
pub type CLRDATATOG_R = crate::BitReader;
#[doc = "Field `CLRDATATOG` writer - Software sets this bit to reset the endpoint data toggle to 0."]
pub type CLRDATATOG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is set when a data packet has been received. Software should clear this bit when the packet has been unloaded from the OUT endpoint FIFO. An interrupt is generated when the bit is set."]
    #[inline(always)]
    pub fn outpktrdy(&self) -> OUTPKTRDY_R {
        OUTPKTRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is set when no more packets can be loaded into the OUT endpoint FIFO."]
    #[inline(always)]
    pub fn fifofull(&self) -> FIFOFULL_R {
        FIFOFULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - For isochronous mode OUT endpoints: This bit is set when an OUT packet cannot be loaded into the OUT endpoint FIFO. Firmware should clear this bit."]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - For isochronous mode OUT endpoints: This bit is set when USB_CSOL.OUTPKTRDY is set if the data packet has a CRC or bit-stuff error. It is cleared automatically when USB_CSOL.OUTPKTRDY is cleared."]
    #[inline(always)]
    pub fn dataerror(&self) -> DATAERROR_R {
        DATAERROR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software sets this bit to flush the next packet to be read from the endpoint OUT FIFO. Note: If the FIFO contains two packets, USB_CSOL.FLUSHPACKET will need to be set twice to completely clear the FIFO."]
    #[inline(always)]
    pub fn flushpacket(&self) -> FLUSHPACKET_R {
        FLUSHPACKET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - For bulk/interrupt mode OUT endpoints: Software sets this bit to issue a STALL handshake. Software clears this bit to terminate the stall condition."]
    #[inline(always)]
    pub fn sendstall(&self) -> SENDSTALL_R {
        SENDSTALL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit is set when a STALL handshake is transmitted. An interrupt is generated when this bit is set. Software should clear this bit."]
    #[inline(always)]
    pub fn sentstall(&self) -> SENTSTALL_R {
        SENTSTALL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software sets this bit to reset the endpoint data toggle to 0."]
    #[inline(always)]
    pub fn clrdatatog(&self) -> CLRDATATOG_R {
        CLRDATATOG_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is set when a data packet has been received. Software should clear this bit when the packet has been unloaded from the OUT endpoint FIFO. An interrupt is generated when the bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn outpktrdy(&mut self) -> OUTPKTRDY_W<CSOL_SPEC> {
        OUTPKTRDY_W::new(self, 0)
    }
    #[doc = "Bit 2 - For isochronous mode OUT endpoints: This bit is set when an OUT packet cannot be loaded into the OUT endpoint FIFO. Firmware should clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn overrun(&mut self) -> OVERRUN_W<CSOL_SPEC> {
        OVERRUN_W::new(self, 2)
    }
    #[doc = "Bit 4 - Software sets this bit to flush the next packet to be read from the endpoint OUT FIFO. Note: If the FIFO contains two packets, USB_CSOL.FLUSHPACKET will need to be set twice to completely clear the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn flushpacket(&mut self) -> FLUSHPACKET_W<CSOL_SPEC> {
        FLUSHPACKET_W::new(self, 4)
    }
    #[doc = "Bit 5 - For bulk/interrupt mode OUT endpoints: Software sets this bit to issue a STALL handshake. Software clears this bit to terminate the stall condition."]
    #[inline(always)]
    #[must_use]
    pub fn sendstall(&mut self) -> SENDSTALL_W<CSOL_SPEC> {
        SENDSTALL_W::new(self, 5)
    }
    #[doc = "Bit 6 - This bit is set when a STALL handshake is transmitted. An interrupt is generated when this bit is set. Software should clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn sentstall(&mut self) -> SENTSTALL_W<CSOL_SPEC> {
        SENTSTALL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Software sets this bit to reset the endpoint data toggle to 0."]
    #[inline(always)]
    #[must_use]
    pub fn clrdatatog(&mut self) -> CLRDATATOG_W<CSOL_SPEC> {
        CLRDATATOG_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (low byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csol::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csol::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSOL_SPEC;
impl crate::RegisterSpec for CSOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csol::R`](R) reader structure"]
impl crate::Readable for CSOL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csol::W`](W) writer structure"]
impl crate::Writable for CSOL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSOL to value 0"]
impl crate::Resettable for CSOL_SPEC {
    const RESET_VALUE: u32 = 0;
}
