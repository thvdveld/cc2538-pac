#[doc = "Register `PMCTL` reader"]
pub type R = crate::R<PMCTL_SPEC>;
#[doc = "Register `PMCTL` writer"]
pub type W = crate::W<PMCTL_SPEC>;
#[doc = "Field `PM` reader - 00: No action 01: PM1 10: PM2 11: PM3"]
pub type PM_R = crate::FieldReader;
#[doc = "Field `PM` writer - 00: No action 01: PM1 10: PM2 11: PM3"]
pub type PM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - 00: No action 01: PM1 10: PM2 11: PM3"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 00: No action 01: PM1 10: PM2 11: PM3"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<PMCTL_SPEC, 0> {
        PM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This register controls the power mode. Note: The Corresponding PM is not entered before the WFI instruction is asserted. To enter PM1-3 the DEEPSLEEP bit in SYSCTRL must be 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMCTL_SPEC;
impl crate::RegisterSpec for PMCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmctl::R`](R) reader structure"]
impl crate::Readable for PMCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmctl::W`](W) writer structure"]
impl crate::Writable for PMCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMCTL to value 0"]
impl crate::Resettable for PMCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
