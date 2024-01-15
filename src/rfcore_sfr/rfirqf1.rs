#[doc = "Register `RFIRQF1` reader"]
pub type R = crate::R<RFIRQF1_SPEC>;
#[doc = "Register `RFIRQF1` writer"]
pub type W = crate::W<RFIRQF1_SPEC>;
#[doc = "Field `TXACKDONE` reader - An acknowledgement frame has been completely transmitted. 0: No interrupt pending 1: Interrupt pending"]
pub type TXACKDONE_R = crate::BitReader;
#[doc = "Field `TXACKDONE` writer - An acknowledgement frame has been completely transmitted. 0: No interrupt pending 1: Interrupt pending"]
pub type TXACKDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDONE` reader - A complete frame has been transmitted. 0: No interrupt pending 1: Interrupt pending"]
pub type TXDONE_R = crate::BitReader;
#[doc = "Field `TXDONE` writer - A complete frame has been transmitted. 0: No interrupt pending 1: Interrupt pending"]
pub type TXDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFIDLE` reader - Radio state-machine has entered the IDLE state. 0: No interrupt pending 1: Interrupt pending"]
pub type RFIDLE_R = crate::BitReader;
#[doc = "Field `RFIDLE` writer - Radio state-machine has entered the IDLE state. 0: No interrupt pending 1: Interrupt pending"]
pub type RFIDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSP_MANINT` reader - Manual interrupt generated from CSP 0: No interrupt pending 1: Interrupt pending"]
pub type CSP_MANINT_R = crate::BitReader;
#[doc = "Field `CSP_MANINT` writer - Manual interrupt generated from CSP 0: No interrupt pending 1: Interrupt pending"]
pub type CSP_MANINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSP_STOP` reader - CSP has stopped program execution. 0: No interrupt pending 1: Interrupt pending"]
pub type CSP_STOP_R = crate::BitReader;
#[doc = "Field `CSP_STOP` writer - CSP has stopped program execution. 0: No interrupt pending 1: Interrupt pending"]
pub type CSP_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSP_WAIT` reader - Execution continued after a wait instruction in CSP. 0: No interrupt pending 1: Interrupt pending"]
pub type CSP_WAIT_R = crate::BitReader;
#[doc = "Field `CSP_WAIT` writer - Execution continued after a wait instruction in CSP. 0: No interrupt pending 1: Interrupt pending"]
pub type CSP_WAIT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - An acknowledgement frame has been completely transmitted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn txackdone(&self) -> TXACKDONE_R {
        TXACKDONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A complete frame has been transmitted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn txdone(&self) -> TXDONE_R {
        TXDONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Radio state-machine has entered the IDLE state. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rfidle(&self) -> RFIDLE_R {
        RFIDLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Manual interrupt generated from CSP 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn csp_manint(&self) -> CSP_MANINT_R {
        CSP_MANINT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CSP has stopped program execution. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn csp_stop(&self) -> CSP_STOP_R {
        CSP_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Execution continued after a wait instruction in CSP. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn csp_wait(&self) -> CSP_WAIT_R {
        CSP_WAIT_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - An acknowledgement frame has been completely transmitted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn txackdone(&mut self) -> TXACKDONE_W<RFIRQF1_SPEC> {
        TXACKDONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - A complete frame has been transmitted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn txdone(&mut self) -> TXDONE_W<RFIRQF1_SPEC> {
        TXDONE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Radio state-machine has entered the IDLE state. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn rfidle(&mut self) -> RFIDLE_W<RFIRQF1_SPEC> {
        RFIDLE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Manual interrupt generated from CSP 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn csp_manint(&mut self) -> CSP_MANINT_W<RFIRQF1_SPEC> {
        CSP_MANINT_W::new(self, 3)
    }
    #[doc = "Bit 4 - CSP has stopped program execution. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn csp_stop(&mut self) -> CSP_STOP_W<RFIRQF1_SPEC> {
        CSP_STOP_W::new(self, 4)
    }
    #[doc = "Bit 5 - Execution continued after a wait instruction in CSP. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn csp_wait(&mut self) -> CSP_WAIT_W<RFIRQF1_SPEC> {
        CSP_WAIT_W::new(self, 5)
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
#[doc = "RF interrupt flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfirqf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfirqf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFIRQF1_SPEC;
impl crate::RegisterSpec for RFIRQF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfirqf1::R`](R) reader structure"]
impl crate::Readable for RFIRQF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rfirqf1::W`](W) writer structure"]
impl crate::Writable for RFIRQF1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFIRQF1 to value 0"]
impl crate::Resettable for RFIRQF1_SPEC {
    const RESET_VALUE: u32 = 0;
}
