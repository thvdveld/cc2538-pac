#[doc = "Register `DCGCRFC` reader"]
pub type R = crate::R<DcgcrfcSpec>;
#[doc = "Register `DCGCRFC` writer"]
pub type W = crate::W<DcgcrfcSpec>;
#[doc = "Field `RFC0` reader - 0: Clock for RF CORE is gated. 1: Clock for RF CORE is enabled."]
pub type Rfc0R = crate::BitReader;
#[doc = "Field `RFC0` writer - 0: Clock for RF CORE is gated. 1: Clock for RF CORE is enabled."]
pub type Rfc0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: Clock for RF CORE is gated. 1: Clock for RF CORE is enabled."]
    #[inline(always)]
    pub fn rfc0(&self) -> Rfc0R {
        Rfc0R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Clock for RF CORE is gated. 1: Clock for RF CORE is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn rfc0(&mut self) -> Rfc0W<DcgcrfcSpec> {
        Rfc0W::new(self, 0)
    }
}
#[doc = "This register defines the module clocks for RF CORE when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcrfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcrfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcgcrfcSpec;
impl crate::RegisterSpec for DcgcrfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcgcrfc::R`](R) reader structure"]
impl crate::Readable for DcgcrfcSpec {}
#[doc = "`write(|w| ..)` method takes [`dcgcrfc::W`](W) writer structure"]
impl crate::Writable for DcgcrfcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCGCRFC to value 0"]
impl crate::Resettable for DcgcrfcSpec {
    const RESET_VALUE: u32 = 0;
}
