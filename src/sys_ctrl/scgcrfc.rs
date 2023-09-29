#[doc = "Register `SCGCRFC` reader"]
pub type R = crate::R<SCGCRFC_SPEC>;
#[doc = "Register `SCGCRFC` writer"]
pub type W = crate::W<SCGCRFC_SPEC>;
#[doc = "Field `RFC0` reader - 0: Clock for RF CORE is gated. 1: Clock for RF CORE is enabled."]
pub type RFC0_R = crate::BitReader;
#[doc = "Field `RFC0` writer - 0: Clock for RF CORE is gated. 1: Clock for RF CORE is enabled."]
pub type RFC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 0: Clock for RF CORE is gated. 1: Clock for RF CORE is enabled."]
    #[inline(always)]
    pub fn rfc0(&self) -> RFC0_R {
        RFC0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Clock for RF CORE is gated. 1: Clock for RF CORE is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn rfc0(&mut self) -> RFC0_W<SCGCRFC_SPEC, 0> {
        RFC0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This register defines the module clocks for RF CORE when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcrfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcrfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCGCRFC_SPEC;
impl crate::RegisterSpec for SCGCRFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgcrfc::R`](R) reader structure"]
impl crate::Readable for SCGCRFC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scgcrfc::W`](W) writer structure"]
impl crate::Writable for SCGCRFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGCRFC to value 0"]
impl crate::Resettable for SCGCRFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
