#[doc = "Register `CLD` reader"]
pub type R = crate::R<CLD_SPEC>;
#[doc = "Register `CLD` writer"]
pub type W = crate::W<CLD_SPEC>;
#[doc = "Field `EN` reader - 0: CLD is disabled. 1: CLD is enabled. Writing to this register shall be ignored if VALID = 0"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - 0: CLD is disabled. 1: CLD is enabled. Writing to this register shall be ignored if VALID = 0"]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VALID` reader - 0: CLD status in always-on domain is not equal to status in the EN register. 1: CLD status in always-on domain and EN register are equal."]
pub type VALID_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 0: CLD is disabled. 1: CLD is enabled. Writing to this register shall be ignored if VALID = 0"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 0: CLD status in always-on domain is not equal to status in the EN register. 1: CLD status in always-on domain and EN register are equal."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: CLD is disabled. 1: CLD is enabled. Writing to this register shall be ignored if VALID = 0"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CLD_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This register controls the clock loss detection feature.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cld::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cld::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLD_SPEC;
impl crate::RegisterSpec for CLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cld::R`](R) reader structure"]
impl crate::Readable for CLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cld::W`](W) writer structure"]
impl crate::Writable for CLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLD to value 0"]
impl crate::Resettable for CLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
