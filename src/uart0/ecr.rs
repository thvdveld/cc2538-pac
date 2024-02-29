#[doc = "Register `ECR` writer"]
pub type W = crate::W<EcrSpec>;
#[doc = "Field `DATA` writer - Error clear A write to this register of any data clears the framing, parity, break, and overrun flags."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Error clear A write to this register of any data clears the framing, parity, break, and overrun flags."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<EcrSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "UART receive status and error clear The RSR/ECR register is the receive status register/error clear register. A write of any value to the ECR register clears the framing, parity, break, and overrun errors. All the bits are cleared on reset. Write-only error clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcrSpec;
impl crate::RegisterSpec for EcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ecr::W`](W) writer structure"]
impl crate::Writable for EcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECR to value 0"]
impl crate::Resettable for EcrSpec {
    const RESET_VALUE: u32 = 0;
}
