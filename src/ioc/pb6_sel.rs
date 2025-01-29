#[doc = "Register `PB6_SEL` reader"]
pub type R = crate::R<Pb6SelSpec>;
#[doc = "Register `PB6_SEL` writer"]
pub type W = crate::W<Pb6SelSpec>;
#[doc = "Field `PB6_sel` reader - Select one peripheral signal output for PB6."]
pub type Pb6SelR = crate::FieldReader;
#[doc = "Field `PB6_sel` writer - Select one peripheral signal output for PB6."]
pub type Pb6SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PB6."]
    #[inline(always)]
    pub fn pb6_sel(&self) -> Pb6SelR {
        Pb6SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PB6."]
    #[inline(always)]
    pub fn pb6_sel(&mut self) -> Pb6SelW<Pb6SelSpec> {
        Pb6SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PB6\n\nYou can [`read`](crate::Reg::read) this register and get [`pb6_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb6_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pb6SelSpec;
impl crate::RegisterSpec for Pb6SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb6_sel::R`](R) reader structure"]
impl crate::Readable for Pb6SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pb6_sel::W`](W) writer structure"]
impl crate::Writable for Pb6SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PB6_SEL to value 0"]
impl crate::Resettable for Pb6SelSpec {
    const RESET_VALUE: u32 = 0;
}
