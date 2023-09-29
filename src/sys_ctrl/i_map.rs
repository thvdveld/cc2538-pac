#[doc = "Register `I_MAP` reader"]
pub type R = crate::R<I_MAP_SPEC>;
#[doc = "Register `I_MAP` writer"]
pub type W = crate::W<I_MAP_SPEC>;
#[doc = "Field `ALTMAP` reader - 1: Select alternate interrupt map. 0: Select regular interrupt map. (See the ASD document for details.)"]
pub type ALTMAP_R = crate::BitReader;
#[doc = "Field `ALTMAP` writer - 1: Select alternate interrupt map. 0: Select regular interrupt map. (See the ASD document for details.)"]
pub type ALTMAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 1: Select alternate interrupt map. 0: Select regular interrupt map. (See the ASD document for details.)"]
    #[inline(always)]
    pub fn altmap(&self) -> ALTMAP_R {
        ALTMAP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Select alternate interrupt map. 0: Select regular interrupt map. (See the ASD document for details.)"]
    #[inline(always)]
    #[must_use]
    pub fn altmap(&mut self) -> ALTMAP_W<I_MAP_SPEC, 0> {
        ALTMAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This register selects which interrupt map to be used.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I_MAP_SPEC;
impl crate::RegisterSpec for I_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i_map::R`](R) reader structure"]
impl crate::Readable for I_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i_map::W`](W) writer structure"]
impl crate::Writable for I_MAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I_MAP to value 0"]
impl crate::Resettable for I_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
