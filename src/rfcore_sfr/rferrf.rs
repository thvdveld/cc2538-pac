#[doc = "Register `RFERRF` reader"]
pub type R = crate::R<RferrfSpec>;
#[doc = "Register `RFERRF` writer"]
pub type W = crate::W<RferrfSpec>;
#[doc = "Field `NLOCK` reader - The frequency synthesizer failed to achieve lock after time-out, or lock is lost during reception. The receiver must be restarted to clear this error situation. 0: No interrupt pending 1: Interrupt pending"]
pub type NlockR = crate::BitReader;
#[doc = "Field `NLOCK` writer - The frequency synthesizer failed to achieve lock after time-out, or lock is lost during reception. The receiver must be restarted to clear this error situation. 0: No interrupt pending 1: Interrupt pending"]
pub type NlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXABO` reader - Reception of a frame was aborted. 0: No interrupt pending 1: Interrupt pending"]
pub type RxaboR = crate::BitReader;
#[doc = "Field `RXABO` writer - Reception of a frame was aborted. 0: No interrupt pending 1: Interrupt pending"]
pub type RxaboW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVERF` reader - RX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
pub type RxoverfR = crate::BitReader;
#[doc = "Field `RXOVERF` writer - RX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
pub type RxoverfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUNDERF` reader - RX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
pub type RxunderfR = crate::BitReader;
#[doc = "Field `RXUNDERF` writer - RX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
pub type RxunderfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOVERF` reader - TX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
pub type TxoverfR = crate::BitReader;
#[doc = "Field `TXOVERF` writer - TX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
pub type TxoverfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNDERF` reader - TX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
pub type TxunderfR = crate::BitReader;
#[doc = "Field `TXUNDERF` writer - TX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
pub type TxunderfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STROBEERR` reader - A command strobe was issued when it could not be processed. Triggered if trying to disable the radio when it is already disabled, or when trying to do a SACK, SACKPEND, or SNACK command when not in active RX. 0: No interrupt pending 1: Interrupt pending"]
pub type StrobeerrR = crate::BitReader;
#[doc = "Field `STROBEERR` writer - A command strobe was issued when it could not be processed. Triggered if trying to disable the radio when it is already disabled, or when trying to do a SACK, SACKPEND, or SNACK command when not in active RX. 0: No interrupt pending 1: Interrupt pending"]
pub type StrobeerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The frequency synthesizer failed to achieve lock after time-out, or lock is lost during reception. The receiver must be restarted to clear this error situation. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn nlock(&self) -> NlockR {
        NlockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reception of a frame was aborted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxabo(&self) -> RxaboR {
        RxaboR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxoverf(&self) -> RxoverfR {
        RxoverfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxunderf(&self) -> RxunderfR {
        RxunderfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn txoverf(&self) -> TxoverfR {
        TxoverfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn txunderf(&self) -> TxunderfR {
        TxunderfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - A command strobe was issued when it could not be processed. Triggered if trying to disable the radio when it is already disabled, or when trying to do a SACK, SACKPEND, or SNACK command when not in active RX. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn strobeerr(&self) -> StrobeerrR {
        StrobeerrR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The frequency synthesizer failed to achieve lock after time-out, or lock is lost during reception. The receiver must be restarted to clear this error situation. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn nlock(&mut self) -> NlockW<RferrfSpec> {
        NlockW::new(self, 0)
    }
    #[doc = "Bit 1 - Reception of a frame was aborted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn rxabo(&mut self) -> RxaboW<RferrfSpec> {
        RxaboW::new(self, 1)
    }
    #[doc = "Bit 2 - RX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn rxoverf(&mut self) -> RxoverfW<RferrfSpec> {
        RxoverfW::new(self, 2)
    }
    #[doc = "Bit 3 - RX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn rxunderf(&mut self) -> RxunderfW<RferrfSpec> {
        RxunderfW::new(self, 3)
    }
    #[doc = "Bit 4 - TX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn txoverf(&mut self) -> TxoverfW<RferrfSpec> {
        TxoverfW::new(self, 4)
    }
    #[doc = "Bit 5 - TX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn txunderf(&mut self) -> TxunderfW<RferrfSpec> {
        TxunderfW::new(self, 5)
    }
    #[doc = "Bit 6 - A command strobe was issued when it could not be processed. Triggered if trying to disable the radio when it is already disabled, or when trying to do a SACK, SACKPEND, or SNACK command when not in active RX. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn strobeerr(&mut self) -> StrobeerrW<RferrfSpec> {
        StrobeerrW::new(self, 6)
    }
}
#[doc = "RF error interrupt flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rferrf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rferrf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RferrfSpec;
impl crate::RegisterSpec for RferrfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rferrf::R`](R) reader structure"]
impl crate::Readable for RferrfSpec {}
#[doc = "`write(|w| ..)` method takes [`rferrf::W`](W) writer structure"]
impl crate::Writable for RferrfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFERRF to value 0"]
impl crate::Resettable for RferrfSpec {
    const RESET_VALUE: u32 = 0;
}
