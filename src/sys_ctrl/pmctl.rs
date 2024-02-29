#[doc = "Register `PMCTL` reader"]
pub type R = crate::R<PmctlSpec>;
#[doc = "Register `PMCTL` writer"]
pub type W = crate::W<PmctlSpec>;
#[doc = "Field `PM` reader - 00: No action 01: PM1 10: PM2 11: PM3"]
pub type PmR = crate::FieldReader;
#[doc = "Field `PM` writer - 00: No action 01: PM1 10: PM2 11: PM3"]
pub type PmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 00: No action 01: PM1 10: PM2 11: PM3"]
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 00: No action 01: PM1 10: PM2 11: PM3"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PmW<PmctlSpec> {
        PmW::new(self, 0)
    }
}
#[doc = "This register controls the power mode. Note: The Corresponding PM is not entered before the WFI instruction is asserted. To enter PM1-3 the DEEPSLEEP bit in SYSCTRL must be 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmctlSpec;
impl crate::RegisterSpec for PmctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmctl::R`](R) reader structure"]
impl crate::Readable for PmctlSpec {}
#[doc = "`write(|w| ..)` method takes [`pmctl::W`](W) writer structure"]
impl crate::Writable for PmctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMCTL to value 0"]
impl crate::Resettable for PmctlSpec {
    const RESET_VALUE: u32 = 0;
}
