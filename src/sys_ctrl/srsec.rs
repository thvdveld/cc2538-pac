#[doc = "Register `SRSEC` reader"]
pub type R = crate::R<SRSEC_SPEC>;
#[doc = "Register `SRSEC` writer"]
pub type W = crate::W<SRSEC_SPEC>;
#[doc = "Field `PKA` reader - 0: PKA module is not reset 1: PKA module is reset"]
pub type PKA_R = crate::BitReader;
#[doc = "Field `PKA` writer - 0: PKA module is not reset 1: PKA module is reset"]
pub type PKA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AES` reader - 0: AES module is not reset 1: AES module is reset"]
pub type AES_R = crate::BitReader;
#[doc = "Field `AES` writer - 0: AES module is not reset 1: AES module is reset"]
pub type AES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 0: PKA module is not reset 1: PKA module is reset"]
    #[inline(always)]
    pub fn pka(&self) -> PKA_R {
        PKA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: AES module is not reset 1: AES module is reset"]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: PKA module is not reset 1: PKA module is reset"]
    #[inline(always)]
    #[must_use]
    pub fn pka(&mut self) -> PKA_W<SRSEC_SPEC, 0> {
        PKA_W::new(self)
    }
    #[doc = "Bit 1 - 0: AES module is not reset 1: AES module is reset"]
    #[inline(always)]
    #[must_use]
    pub fn aes(&mut self) -> AES_W<SRSEC_SPEC, 1> {
        AES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This register controls the reset for the security module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsec::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsec::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRSEC_SPEC;
impl crate::RegisterSpec for SRSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srsec::R`](R) reader structure"]
impl crate::Readable for SRSEC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srsec::W`](W) writer structure"]
impl crate::Writable for SRSEC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRSEC to value 0"]
impl crate::Resettable for SRSEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
