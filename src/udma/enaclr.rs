#[doc = "Register `ENACLR` writer"]
pub type W = crate::W<ENACLR_SPEC>;
#[doc = "Field `CLR` writer - Channel \\[n\\]
enable clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\]
bit in the DMAENASET register meaning that channel \\[n\\]
is disabled for uDMA transfers. Note: The controller disables a channel when it completes the uDMA cycle."]
pub type CLR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
enable clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\]
bit in the DMAENASET register meaning that channel \\[n\\]
is disabled for uDMA transfers. Note: The controller disables a channel when it completes the uDMA cycle."]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> CLR_W<ENACLR_SPEC> {
        CLR_W::new(self, 0)
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
#[doc = "DMA channel enable clear Each bit of the ENACLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the ENASET register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enaclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENACLR_SPEC;
impl crate::RegisterSpec for ENACLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`enaclr::W`](W) writer structure"]
impl crate::Writable for ENACLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENACLR to value 0"]
impl crate::Resettable for ENACLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
