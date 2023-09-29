#[doc = "Register `ERRCLR` reader"]
pub type R = crate::R<ERRCLR_SPEC>;
#[doc = "Register `ERRCLR` writer"]
pub type W = crate::W<ERRCLR_SPEC>;
#[doc = "Field `ERRCLR` reader - uDMA bus error status 0: No bus error is pending 1: A bus error is pending This bit is cleared by writing 1 to it."]
pub type ERRCLR_R = crate::BitReader;
#[doc = "Field `ERRCLR` writer - uDMA bus error status 0: No bus error is pending 1: A bus error is pending This bit is cleared by writing 1 to it."]
pub type ERRCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - uDMA bus error status 0: No bus error is pending 1: A bus error is pending This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn errclr(&self) -> ERRCLR_R {
        ERRCLR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - uDMA bus error status 0: No bus error is pending 1: A bus error is pending This bit is cleared by writing 1 to it."]
    #[inline(always)]
    #[must_use]
    pub fn errclr(&mut self) -> ERRCLR_W<ERRCLR_SPEC, 0> {
        ERRCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA bus error clear The ERRCLR register is used to read and clear the uDMA bus error status. The error status is set if the uDMA controller encountered a bus error while performing a transfer. If a bus error occurs on a channel, that channel is automatically disabled by the uDMA controller. The other channels are unaffected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`errclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ERRCLR_SPEC;
impl crate::RegisterSpec for ERRCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errclr::R`](R) reader structure"]
impl crate::Readable for ERRCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`errclr::W`](W) writer structure"]
impl crate::Writable for ERRCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERRCLR to value 0"]
impl crate::Resettable for ERRCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
