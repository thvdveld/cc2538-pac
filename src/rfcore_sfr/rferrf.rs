#[doc = "Register `RFERRF` reader"]
pub struct R(crate::R<RFERRF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFERRF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFERRF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFERRF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFERRF` writer"]
pub struct W(crate::W<RFERRF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFERRF_SPEC>;
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
impl From<crate::W<RFERRF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFERRF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NLOCK` reader - The frequency synthesizer failed to achieve lock after time-out, or lock is lost during reception. The receiver must be restarted to clear this error situation. 0: No interrupt pending 1: Interrupt pending"]
pub type NLOCK_R = crate::BitReader<bool>;
#[doc = "Field `NLOCK` writer - The frequency synthesizer failed to achieve lock after time-out, or lock is lost during reception. The receiver must be restarted to clear this error situation. 0: No interrupt pending 1: Interrupt pending"]
pub type NLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFERRF_SPEC, bool, O>;
#[doc = "Field `RXABO` reader - Reception of a frame was aborted. 0: No interrupt pending 1: Interrupt pending"]
pub type RXABO_R = crate::BitReader<bool>;
#[doc = "Field `RXABO` writer - Reception of a frame was aborted. 0: No interrupt pending 1: Interrupt pending"]
pub type RXABO_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFERRF_SPEC, bool, O>;
#[doc = "Field `RXOVERF` reader - RX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
pub type RXOVERF_R = crate::BitReader<bool>;
#[doc = "Field `RXOVERF` writer - RX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
pub type RXOVERF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFERRF_SPEC, bool, O>;
#[doc = "Field `RXUNDERF` reader - RX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
pub type RXUNDERF_R = crate::BitReader<bool>;
#[doc = "Field `RXUNDERF` writer - RX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
pub type RXUNDERF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFERRF_SPEC, bool, O>;
#[doc = "Field `TXOVERF` reader - TX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
pub type TXOVERF_R = crate::BitReader<bool>;
#[doc = "Field `TXOVERF` writer - TX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
pub type TXOVERF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFERRF_SPEC, bool, O>;
#[doc = "Field `TXUNDERF` reader - TX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
pub type TXUNDERF_R = crate::BitReader<bool>;
#[doc = "Field `TXUNDERF` writer - TX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
pub type TXUNDERF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFERRF_SPEC, bool, O>;
#[doc = "Field `STROBEERR` reader - A command strobe was issued when it could not be processed. Triggered if trying to disable the radio when it is already disabled, or when trying to do a SACK, SACKPEND, or SNACK command when not in active RX. 0: No interrupt pending 1: Interrupt pending"]
pub type STROBEERR_R = crate::BitReader<bool>;
#[doc = "Field `STROBEERR` writer - A command strobe was issued when it could not be processed. Triggered if trying to disable the radio when it is already disabled, or when trying to do a SACK, SACKPEND, or SNACK command when not in active RX. 0: No interrupt pending 1: Interrupt pending"]
pub type STROBEERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFERRF_SPEC, bool, O>;
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
    pub fn nlock(&mut self) -> NLOCK_W<0> {
        NLOCK_W::new(self)
    }
    #[doc = "Bit 1 - Reception of a frame was aborted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn rxabo(&mut self) -> RXABO_W<1> {
        RXABO_W::new(self)
    }
    #[doc = "Bit 2 - RX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn rxoverf(&mut self) -> RXOVERF_W<2> {
        RXOVERF_W::new(self)
    }
    #[doc = "Bit 3 - RX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn rxunderf(&mut self) -> RXUNDERF_W<3> {
        RXUNDERF_W::new(self)
    }
    #[doc = "Bit 4 - TX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn txoverf(&mut self) -> TXOVERF_W<4> {
        TXOVERF_W::new(self)
    }
    #[doc = "Bit 5 - TX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn txunderf(&mut self) -> TXUNDERF_W<5> {
        TXUNDERF_W::new(self)
    }
    #[doc = "Bit 6 - A command strobe was issued when it could not be processed. Triggered if trying to disable the radio when it is already disabled, or when trying to do a SACK, SACKPEND, or SNACK command when not in active RX. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn strobeerr(&mut self) -> STROBEERR_W<6> {
        STROBEERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RF error interrupt flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rferrf](index.html) module"]
pub struct RFERRF_SPEC;
impl crate::RegisterSpec for RFERRF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rferrf::R](R) reader structure"]
impl crate::Readable for RFERRF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rferrf::W](W) writer structure"]
impl crate::Writable for RFERRF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFERRF to value 0"]
impl crate::Resettable for RFERRF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
