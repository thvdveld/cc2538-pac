#[doc = "Register `CMPCTL` reader"]
pub type R = crate::R<CmpctlSpec>;
#[doc = "Register `CMPCTL` writer"]
pub type W = crate::W<CmpctlSpec>;
#[doc = "Field `OUTPUT` reader - Comparator output"]
pub type OutputR = crate::BitReader;
#[doc = "Field `EN` reader - Comparator enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Comparator enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator output"]
    #[inline(always)]
    pub fn output(&self) -> OutputR {
        OutputR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Comparator enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<CmpctlSpec> {
        EnW::new(self, 1)
    }
}
#[doc = "Analog comparator control and status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpctlSpec;
impl crate::RegisterSpec for CmpctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpctl::R`](R) reader structure"]
impl crate::Readable for CmpctlSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpctl::W`](W) writer structure"]
impl crate::Writable for CmpctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMPCTL to value 0"]
impl crate::Resettable for CmpctlSpec {
    const RESET_VALUE: u32 = 0;
}
