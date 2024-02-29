#[doc = "Register `PA7_SEL` reader"]
pub type R = crate::R<Pa7SelSpec>;
#[doc = "Register `PA7_SEL` writer"]
pub type W = crate::W<Pa7SelSpec>;
#[doc = "Field `PA7_sel` reader - Select one peripheral signal output for PA7."]
pub type Pa7SelR = crate::FieldReader;
#[doc = "Field `PA7_sel` writer - Select one peripheral signal output for PA7."]
pub type Pa7SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PA7."]
    #[inline(always)]
    pub fn pa7_sel(&self) -> Pa7SelR {
        Pa7SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PA7."]
    #[inline(always)]
    #[must_use]
    pub fn pa7_sel(&mut self) -> Pa7SelW<Pa7SelSpec> {
        Pa7SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PA7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa7_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa7_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pa7SelSpec;
impl crate::RegisterSpec for Pa7SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa7_sel::R`](R) reader structure"]
impl crate::Readable for Pa7SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pa7_sel::W`](W) writer structure"]
impl crate::Writable for Pa7SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PA7_SEL to value 0"]
impl crate::Resettable for Pa7SelSpec {
    const RESET_VALUE: u32 = 0;
}
