#[doc = "Register `RFERRF` reader"]
pub type R = crate::R<RFERRF_SPEC>;
#[doc = "Register `RFERRF` writer"]
pub type W = crate::W<RFERRF_SPEC>;
#[doc = "Field `NLOCK` reader - The frequency synthesizer failed to achieve lock after time-out, or lock is lost during reception. The receiver must be restarted to clear this error situation. 0: No interrupt pending 1: Interrupt pending"]
pub type NLOCK_R = crate::BitReader;
#[doc = "Field `NLOCK` writer - The frequency synthesizer failed to achieve lock after time-out, or lock is lost during reception. The receiver must be restarted to clear this error situation. 0: No interrupt pending 1: Interrupt pending"]
pub type NLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXABO` reader - Reception of a frame was aborted. 0: No interrupt pending 1: Interrupt pending"]
pub type RXABO_R = crate::BitReader;
#[doc = "Field `RXABO` writer - Reception of a frame was aborted. 0: No interrupt pending 1: Interrupt pending"]
pub type RXABO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXOVERF` reader - RX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
pub type RXOVERF_R = crate::BitReader;
#[doc = "Field `RXOVERF` writer - RX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
pub type RXOVERF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXUNDERF` reader - RX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
pub type RXUNDERF_R = crate::BitReader;
#[doc = "Field `RXUNDERF` writer - RX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
pub type RXUNDERF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXOVERF` reader - TX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
pub type TXOVERF_R = crate::BitReader;
#[doc = "Field `TXOVERF` writer - TX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
pub type TXOVERF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXUNDERF` reader - TX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
pub type TXUNDERF_R = crate::BitReader;
#[doc = "Field `TXUNDERF` writer - TX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
pub type TXUNDERF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STROBEERR` reader - A command strobe was issued when it could not be processed. Triggered if trying to disable the radio when it is already disabled, or when trying to do a SACK, SACKPEND, or SNACK command when not in active RX. 0: No interrupt pending 1: Interrupt pending"]
pub type STROBEERR_R = crate::BitReader;
#[doc = "Field `STROBEERR` writer - A command strobe was issued when it could not be processed. Triggered if trying to disable the radio when it is already disabled, or when trying to do a SACK, SACKPEND, or SNACK command when not in active RX. 0: No interrupt pending 1: Interrupt pending"]
pub type STROBEERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - The frequency synthesizer failed to achieve lock after time-out, or lock is lost during reception. The receiver must be restarted to clear this error situation. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn nlock(&self) -> NLOCK_R {
        NLOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reception of a frame was aborted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxabo(&self) -> RXABO_R {
        RXABO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxoverf(&self) -> RXOVERF_R {
        RXOVERF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxunderf(&self) -> RXUNDERF_R {
        RXUNDERF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn txoverf(&self) -> TXOVERF_R {
        TXOVERF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn txunderf(&self) -> TXUNDERF_R {
        TXUNDERF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - A command strobe was issued when it could not be processed. Triggered if trying to disable the radio when it is already disabled, or when trying to do a SACK, SACKPEND, or SNACK command when not in active RX. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn strobeerr(&self) -> STROBEERR_R {
        STROBEERR_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The frequency synthesizer failed to achieve lock after time-out, or lock is lost during reception. The receiver must be restarted to clear this error situation. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn nlock(&mut self) -> NLOCK_W<RFERRF_SPEC, 0> {
        NLOCK_W::new(self)
    }
    #[doc = "Bit 1 - Reception of a frame was aborted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn rxabo(&mut self) -> RXABO_W<RFERRF_SPEC, 1> {
        RXABO_W::new(self)
    }
    #[doc = "Bit 2 - RX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn rxoverf(&mut self) -> RXOVERF_W<RFERRF_SPEC, 2> {
        RXOVERF_W::new(self)
    }
    #[doc = "Bit 3 - RX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn rxunderf(&mut self) -> RXUNDERF_W<RFERRF_SPEC, 3> {
        RXUNDERF_W::new(self)
    }
    #[doc = "Bit 4 - TX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn txoverf(&mut self) -> TXOVERF_W<RFERRF_SPEC, 4> {
        TXOVERF_W::new(self)
    }
    #[doc = "Bit 5 - TX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn txunderf(&mut self) -> TXUNDERF_W<RFERRF_SPEC, 5> {
        TXUNDERF_W::new(self)
    }
    #[doc = "Bit 6 - A command strobe was issued when it could not be processed. Triggered if trying to disable the radio when it is already disabled, or when trying to do a SACK, SACKPEND, or SNACK command when not in active RX. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn strobeerr(&mut self) -> STROBEERR_W<RFERRF_SPEC, 6> {
        STROBEERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RF error interrupt flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rferrf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rferrf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFERRF_SPEC;
impl crate::RegisterSpec for RFERRF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rferrf::R`](R) reader structure"]
impl crate::Readable for RFERRF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rferrf::W`](W) writer structure"]
impl crate::Writable for RFERRF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFERRF to value 0"]
impl crate::Resettable for RFERRF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
