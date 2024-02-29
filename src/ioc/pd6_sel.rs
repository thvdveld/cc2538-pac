#[doc = "Register `PD6_SEL` reader"]
pub type R = crate::R<Pd6SelSpec>;
#[doc = "Register `PD6_SEL` writer"]
pub type W = crate::W<Pd6SelSpec>;
#[doc = "Field `PD6_sel` reader - Select one peripheral signal output for PD6."]
pub type Pd6SelR = crate::FieldReader;
#[doc = "Field `PD6_sel` writer - Select one peripheral signal output for PD6."]
pub type Pd6SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PD6."]
    #[inline(always)]
    pub fn pd6_sel(&self) -> Pd6SelR {
        Pd6SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PD6."]
    #[inline(always)]
    #[must_use]
    pub fn pd6_sel(&mut self) -> Pd6SelW<Pd6SelSpec> {
        Pd6SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PD6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd6_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd6_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pd6SelSpec;
impl crate::RegisterSpec for Pd6SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd6_sel::R`](R) reader structure"]
impl crate::Readable for Pd6SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pd6_sel::W`](W) writer structure"]
impl crate::Writable for Pd6SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PD6_SEL to value 0"]
impl crate::Resettable for Pd6SelSpec {
    const RESET_VALUE: u32 = 0;
}
