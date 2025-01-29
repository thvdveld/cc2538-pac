#[doc = "Register `PB1_SEL` reader"]
pub type R = crate::R<Pb1SelSpec>;
#[doc = "Register `PB1_SEL` writer"]
pub type W = crate::W<Pb1SelSpec>;
#[doc = "Field `PB1_sel` reader - Select one peripheral signal output for PB1."]
pub type Pb1SelR = crate::FieldReader;
#[doc = "Field `PB1_sel` writer - Select one peripheral signal output for PB1."]
pub type Pb1SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PB1."]
    #[inline(always)]
    pub fn pb1_sel(&self) -> Pb1SelR {
        Pb1SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PB1."]
    #[inline(always)]
    pub fn pb1_sel(&mut self) -> Pb1SelW<Pb1SelSpec> {
        Pb1SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PB1\n\nYou can [`read`](crate::Reg::read) this register and get [`pb1_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb1_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pb1SelSpec;
impl crate::RegisterSpec for Pb1SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb1_sel::R`](R) reader structure"]
impl crate::Readable for Pb1SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pb1_sel::W`](W) writer structure"]
impl crate::Writable for Pb1SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PB1_SEL to value 0"]
impl crate::Resettable for Pb1SelSpec {
    const RESET_VALUE: u32 = 0;
}
