#[doc = "Register `I_MAP` reader"]
pub type R = crate::R<IMapSpec>;
#[doc = "Register `I_MAP` writer"]
pub type W = crate::W<IMapSpec>;
#[doc = "Field `ALTMAP` reader - 1: Select alternate interrupt map. 0: Select regular interrupt map. (See the ASD document for details.)"]
pub type AltmapR = crate::BitReader;
#[doc = "Field `ALTMAP` writer - 1: Select alternate interrupt map. 0: Select regular interrupt map. (See the ASD document for details.)"]
pub type AltmapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: Select alternate interrupt map. 0: Select regular interrupt map. (See the ASD document for details.)"]
    #[inline(always)]
    pub fn altmap(&self) -> AltmapR {
        AltmapR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Select alternate interrupt map. 0: Select regular interrupt map. (See the ASD document for details.)"]
    #[inline(always)]
    pub fn altmap(&mut self) -> AltmapW<IMapSpec> {
        AltmapW::new(self, 0)
    }
}
#[doc = "This register selects which interrupt map to be used.\n\nYou can [`read`](crate::Reg::read) this register and get [`i_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMapSpec;
impl crate::RegisterSpec for IMapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i_map::R`](R) reader structure"]
impl crate::Readable for IMapSpec {}
#[doc = "`write(|w| ..)` method takes [`i_map::W`](W) writer structure"]
impl crate::Writable for IMapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I_MAP to value 0"]
impl crate::Resettable for IMapSpec {
    const RESET_VALUE: u32 = 0;
}
