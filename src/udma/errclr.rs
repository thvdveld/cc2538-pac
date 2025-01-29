#[doc = "Register `ERRCLR` reader"]
pub type R = crate::R<ErrclrSpec>;
#[doc = "Register `ERRCLR` writer"]
pub type W = crate::W<ErrclrSpec>;
#[doc = "Field `ERRCLR` reader - uDMA bus error status 0: No bus error is pending 1: A bus error is pending This bit is cleared by writing 1 to it."]
pub type ErrclrR = crate::BitReader;
#[doc = "Field `ERRCLR` writer - uDMA bus error status 0: No bus error is pending 1: A bus error is pending This bit is cleared by writing 1 to it."]
pub type ErrclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - uDMA bus error status 0: No bus error is pending 1: A bus error is pending This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn errclr(&self) -> ErrclrR {
        ErrclrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - uDMA bus error status 0: No bus error is pending 1: A bus error is pending This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn errclr(&mut self) -> ErrclrW<ErrclrSpec> {
        ErrclrW::new(self, 0)
    }
}
#[doc = "DMA bus error clear The ERRCLR register is used to read and clear the uDMA bus error status. The error status is set if the uDMA controller encountered a bus error while performing a transfer. If a bus error occurs on a channel, that channel is automatically disabled by the uDMA controller. The other channels are unaffected.\n\nYou can [`read`](crate::Reg::read) this register and get [`errclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrclrSpec;
impl crate::RegisterSpec for ErrclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errclr::R`](R) reader structure"]
impl crate::Readable for ErrclrSpec {}
#[doc = "`write(|w| ..)` method takes [`errclr::W`](W) writer structure"]
impl crate::Writable for ErrclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERRCLR to value 0"]
impl crate::Resettable for ErrclrSpec {
    const RESET_VALUE: u32 = 0;
}
