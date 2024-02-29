#[doc = "Register `CLD` reader"]
pub type R = crate::R<CldSpec>;
#[doc = "Register `CLD` writer"]
pub type W = crate::W<CldSpec>;
#[doc = "Field `EN` reader - 0: CLD is disabled. 1: CLD is enabled. Writing to this register shall be ignored if VALID = 0"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 0: CLD is disabled. 1: CLD is enabled. Writing to this register shall be ignored if VALID = 0"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VALID` reader - 0: CLD status in always-on domain is not equal to status in the EN register. 1: CLD status in always-on domain and EN register are equal."]
pub type ValidR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 0: CLD is disabled. 1: CLD is enabled. Writing to this register shall be ignored if VALID = 0"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 0: CLD status in always-on domain is not equal to status in the EN register. 1: CLD status in always-on domain and EN register are equal."]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: CLD is disabled. 1: CLD is enabled. Writing to this register shall be ignored if VALID = 0"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<CldSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "This register controls the clock loss detection feature.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cld::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cld::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CldSpec;
impl crate::RegisterSpec for CldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cld::R`](R) reader structure"]
impl crate::Readable for CldSpec {}
#[doc = "`write(|w| ..)` method takes [`cld::W`](W) writer structure"]
impl crate::Writable for CldSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLD to value 0"]
impl crate::Resettable for CldSpec {
    const RESET_VALUE: u32 = 0;
}
