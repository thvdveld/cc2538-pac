#[doc = "Register `PA2_SEL` reader"]
pub type R = crate::R<Pa2SelSpec>;
#[doc = "Register `PA2_SEL` writer"]
pub type W = crate::W<Pa2SelSpec>;
#[doc = "Field `PA2_sel` reader - Select one peripheral signal output for PA2."]
pub type Pa2SelR = crate::FieldReader;
#[doc = "Field `PA2_sel` writer - Select one peripheral signal output for PA2."]
pub type Pa2SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PA2."]
    #[inline(always)]
    pub fn pa2_sel(&self) -> Pa2SelR {
        Pa2SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PA2."]
    #[inline(always)]
    pub fn pa2_sel(&mut self) -> Pa2SelW<Pa2SelSpec> {
        Pa2SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PA2\n\nYou can [`read`](crate::Reg::read) this register and get [`pa2_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa2_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pa2SelSpec;
impl crate::RegisterSpec for Pa2SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa2_sel::R`](R) reader structure"]
impl crate::Readable for Pa2SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pa2_sel::W`](W) writer structure"]
impl crate::Writable for Pa2SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PA2_SEL to value 0"]
impl crate::Resettable for Pa2SelSpec {
    const RESET_VALUE: u32 = 0;
}
