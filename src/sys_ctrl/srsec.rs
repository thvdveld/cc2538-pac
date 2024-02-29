#[doc = "Register `SRSEC` reader"]
pub type R = crate::R<SrsecSpec>;
#[doc = "Register `SRSEC` writer"]
pub type W = crate::W<SrsecSpec>;
#[doc = "Field `PKA` reader - 0: PKA module is not reset 1: PKA module is reset"]
pub type PkaR = crate::BitReader;
#[doc = "Field `PKA` writer - 0: PKA module is not reset 1: PKA module is reset"]
pub type PkaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES` reader - 0: AES module is not reset 1: AES module is reset"]
pub type AesR = crate::BitReader;
#[doc = "Field `AES` writer - 0: AES module is not reset 1: AES module is reset"]
pub type AesW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: PKA module is not reset 1: PKA module is reset"]
    #[inline(always)]
    pub fn pka(&self) -> PkaR {
        PkaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: AES module is not reset 1: AES module is reset"]
    #[inline(always)]
    pub fn aes(&self) -> AesR {
        AesR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: PKA module is not reset 1: PKA module is reset"]
    #[inline(always)]
    #[must_use]
    pub fn pka(&mut self) -> PkaW<SrsecSpec> {
        PkaW::new(self, 0)
    }
    #[doc = "Bit 1 - 0: AES module is not reset 1: AES module is reset"]
    #[inline(always)]
    #[must_use]
    pub fn aes(&mut self) -> AesW<SrsecSpec> {
        AesW::new(self, 1)
    }
}
#[doc = "This register controls the reset for the security module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsec::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsec::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrsecSpec;
impl crate::RegisterSpec for SrsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srsec::R`](R) reader structure"]
impl crate::Readable for SrsecSpec {}
#[doc = "`write(|w| ..)` method takes [`srsec::W`](W) writer structure"]
impl crate::Writable for SrsecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRSEC to value 0"]
impl crate::Resettable for SrsecSpec {
    const RESET_VALUE: u32 = 0;
}
