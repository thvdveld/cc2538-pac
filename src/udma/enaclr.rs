#[doc = "Register `ENACLR` writer"]
pub type W = crate::W<EnaclrSpec>;
#[doc = "Field `CLR` writer - Channel \\[n\\]
enable clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\]
bit in the DMAENASET register meaning that channel \\[n\\]
is disabled for uDMA transfers. Note: The controller disables a channel when it completes the uDMA cycle."]
pub type ClrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
enable clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\]
bit in the DMAENASET register meaning that channel \\[n\\]
is disabled for uDMA transfers. Note: The controller disables a channel when it completes the uDMA cycle."]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> ClrW<EnaclrSpec> {
        ClrW::new(self, 0)
    }
}
#[doc = "DMA channel enable clear Each bit of the ENACLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the ENASET register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enaclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnaclrSpec;
impl crate::RegisterSpec for EnaclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`enaclr::W`](W) writer structure"]
impl crate::Writable for EnaclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENACLR to value 0"]
impl crate::Resettable for EnaclrSpec {
    const RESET_VALUE: u32 = 0;
}
