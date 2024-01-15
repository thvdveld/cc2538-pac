#[doc = "Register `RFDATA` reader"]
pub type R = crate::R<RFDATA_SPEC>;
#[doc = "Register `RFDATA` writer"]
pub type W = crate::W<RFDATA_SPEC>;
#[doc = "Field `RFD` reader - Data written to the register is written to the TX FIFO. When reading this register, data from the RX FIFO is read."]
pub type RFD_R = crate::FieldReader;
#[doc = "Field `RFD` writer - Data written to the register is written to the TX FIFO. When reading this register, data from the RX FIFO is read."]
pub type RFD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data written to the register is written to the TX FIFO. When reading this register, data from the RX FIFO is read."]
    #[inline(always)]
    pub fn rfd(&self) -> RFD_R {
        RFD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data written to the register is written to the TX FIFO. When reading this register, data from the RX FIFO is read."]
    #[inline(always)]
    #[must_use]
    pub fn rfd(&mut self) -> RFD_W<RFDATA_SPEC> {
        RFD_W::new(self, 0)
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
#[doc = "The TX FIFO and RX FIFO may be accessed through this register. Data is written to the TX FIFO when writing to the RFD register. Data is read from the RX FIFO when the RFD register is read. The XREG registers RXFIFOCNT and TXFIFOCNT provide information on the amount of data in the FIFOs. The FIFO contents can be cleared by issuing SFLUSHRX and SFLUSHTX.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFDATA_SPEC;
impl crate::RegisterSpec for RFDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfdata::R`](R) reader structure"]
impl crate::Readable for RFDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rfdata::W`](W) writer structure"]
impl crate::Writable for RFDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFDATA to value 0"]
impl crate::Resettable for RFDATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
