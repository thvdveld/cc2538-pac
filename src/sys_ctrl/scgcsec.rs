#[doc = "Register `SCGCSEC` reader"]
pub type R = crate::R<ScgcsecSpec>;
#[doc = "Register `SCGCSEC` writer"]
pub type W = crate::W<ScgcsecSpec>;
#[doc = "Field `PKA` reader - 0: Clock for PKA is gated. 1: Clock for PKA is enabled."]
pub type PkaR = crate::BitReader;
#[doc = "Field `PKA` writer - 0: Clock for PKA is gated. 1: Clock for PKA is enabled."]
pub type PkaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES` reader - 0: Clock for AES is gated. 1: Clock for AES is enabled."]
pub type AesR = crate::BitReader;
#[doc = "Field `AES` writer - 0: Clock for AES is gated. 1: Clock for AES is enabled."]
pub type AesW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: Clock for PKA is gated. 1: Clock for PKA is enabled."]
    #[inline(always)]
    pub fn pka(&self) -> PkaR {
        PkaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: Clock for AES is gated. 1: Clock for AES is enabled."]
    #[inline(always)]
    pub fn aes(&self) -> AesR {
        AesR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Clock for PKA is gated. 1: Clock for PKA is enabled."]
    #[inline(always)]
    pub fn pka(&mut self) -> PkaW<ScgcsecSpec> {
        PkaW::new(self, 0)
    }
    #[doc = "Bit 1 - 0: Clock for AES is gated. 1: Clock for AES is enabled."]
    #[inline(always)]
    pub fn aes(&mut self) -> AesW<ScgcsecSpec> {
        AesW::new(self, 1)
    }
}
#[doc = "This register defines the module clocks for the security module when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`scgcsec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgcsec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScgcsecSpec;
impl crate::RegisterSpec for ScgcsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgcsec::R`](R) reader structure"]
impl crate::Readable for ScgcsecSpec {}
#[doc = "`write(|w| ..)` method takes [`scgcsec::W`](W) writer structure"]
impl crate::Writable for ScgcsecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCGCSEC to value 0"]
impl crate::Resettable for ScgcsecSpec {
    const RESET_VALUE: u32 = 0;
}
