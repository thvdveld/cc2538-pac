#[doc = "Register `RFIRQF1` reader"]
pub type R = crate::R<Rfirqf1Spec>;
#[doc = "Register `RFIRQF1` writer"]
pub type W = crate::W<Rfirqf1Spec>;
#[doc = "Field `TXACKDONE` reader - An acknowledgement frame has been completely transmitted. 0: No interrupt pending 1: Interrupt pending"]
pub type TxackdoneR = crate::BitReader;
#[doc = "Field `TXACKDONE` writer - An acknowledgement frame has been completely transmitted. 0: No interrupt pending 1: Interrupt pending"]
pub type TxackdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDONE` reader - A complete frame has been transmitted. 0: No interrupt pending 1: Interrupt pending"]
pub type TxdoneR = crate::BitReader;
#[doc = "Field `TXDONE` writer - A complete frame has been transmitted. 0: No interrupt pending 1: Interrupt pending"]
pub type TxdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFIDLE` reader - Radio state-machine has entered the IDLE state. 0: No interrupt pending 1: Interrupt pending"]
pub type RfidleR = crate::BitReader;
#[doc = "Field `RFIDLE` writer - Radio state-machine has entered the IDLE state. 0: No interrupt pending 1: Interrupt pending"]
pub type RfidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSP_MANINT` reader - Manual interrupt generated from CSP 0: No interrupt pending 1: Interrupt pending"]
pub type CspManintR = crate::BitReader;
#[doc = "Field `CSP_MANINT` writer - Manual interrupt generated from CSP 0: No interrupt pending 1: Interrupt pending"]
pub type CspManintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSP_STOP` reader - CSP has stopped program execution. 0: No interrupt pending 1: Interrupt pending"]
pub type CspStopR = crate::BitReader;
#[doc = "Field `CSP_STOP` writer - CSP has stopped program execution. 0: No interrupt pending 1: Interrupt pending"]
pub type CspStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSP_WAIT` reader - Execution continued after a wait instruction in CSP. 0: No interrupt pending 1: Interrupt pending"]
pub type CspWaitR = crate::BitReader;
#[doc = "Field `CSP_WAIT` writer - Execution continued after a wait instruction in CSP. 0: No interrupt pending 1: Interrupt pending"]
pub type CspWaitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - An acknowledgement frame has been completely transmitted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn txackdone(&self) -> TxackdoneR {
        TxackdoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A complete frame has been transmitted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn txdone(&self) -> TxdoneR {
        TxdoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Radio state-machine has entered the IDLE state. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rfidle(&self) -> RfidleR {
        RfidleR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Manual interrupt generated from CSP 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn csp_manint(&self) -> CspManintR {
        CspManintR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CSP has stopped program execution. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn csp_stop(&self) -> CspStopR {
        CspStopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Execution continued after a wait instruction in CSP. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn csp_wait(&self) -> CspWaitR {
        CspWaitR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - An acknowledgement frame has been completely transmitted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn txackdone(&mut self) -> TxackdoneW<Rfirqf1Spec> {
        TxackdoneW::new(self, 0)
    }
    #[doc = "Bit 1 - A complete frame has been transmitted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn txdone(&mut self) -> TxdoneW<Rfirqf1Spec> {
        TxdoneW::new(self, 1)
    }
    #[doc = "Bit 2 - Radio state-machine has entered the IDLE state. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn rfidle(&mut self) -> RfidleW<Rfirqf1Spec> {
        RfidleW::new(self, 2)
    }
    #[doc = "Bit 3 - Manual interrupt generated from CSP 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn csp_manint(&mut self) -> CspManintW<Rfirqf1Spec> {
        CspManintW::new(self, 3)
    }
    #[doc = "Bit 4 - CSP has stopped program execution. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn csp_stop(&mut self) -> CspStopW<Rfirqf1Spec> {
        CspStopW::new(self, 4)
    }
    #[doc = "Bit 5 - Execution continued after a wait instruction in CSP. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn csp_wait(&mut self) -> CspWaitW<Rfirqf1Spec> {
        CspWaitW::new(self, 5)
    }
}
#[doc = "RF interrupt flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfirqf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfirqf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rfirqf1Spec;
impl crate::RegisterSpec for Rfirqf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfirqf1::R`](R) reader structure"]
impl crate::Readable for Rfirqf1Spec {}
#[doc = "`write(|w| ..)` method takes [`rfirqf1::W`](W) writer structure"]
impl crate::Writable for Rfirqf1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFIRQF1 to value 0"]
impl crate::Resettable for Rfirqf1Spec {
    const RESET_VALUE: u32 = 0;
}
