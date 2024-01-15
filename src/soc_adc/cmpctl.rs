#[doc = "Register `CMPCTL` reader"]
pub type R = crate::R<CMPCTL_SPEC>;
#[doc = "Register `CMPCTL` writer"]
pub type W = crate::W<CMPCTL_SPEC>;
#[doc = "Field `OUTPUT` reader - Comparator output"]
pub type OUTPUT_R = crate::BitReader;
#[doc = "Field `EN` reader - Comparator enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Comparator enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator output"]
    #[inline(always)]
    pub fn output(&self) -> OUTPUT_R {
        OUTPUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Comparator enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CMPCTL_SPEC> {
        EN_W::new(self, 1)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Analog comparator control and status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPCTL_SPEC;
impl crate::RegisterSpec for CMPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpctl::R`](R) reader structure"]
impl crate::Readable for CMPCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmpctl::W`](W) writer structure"]
impl crate::Writable for CMPCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMPCTL to value 0"]
impl crate::Resettable for CMPCTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
