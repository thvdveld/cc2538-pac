#[doc = "Register `PB3_SEL` reader"]
pub type R = crate::R<Pb3SelSpec>;
#[doc = "Register `PB3_SEL` writer"]
pub type W = crate::W<Pb3SelSpec>;
#[doc = "Field `PB3_sel` reader - Select one peripheral signal output for PB3."]
pub type Pb3SelR = crate::FieldReader;
#[doc = "Field `PB3_sel` writer - Select one peripheral signal output for PB3."]
pub type Pb3SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PB3."]
    #[inline(always)]
    pub fn pb3_sel(&self) -> Pb3SelR {
        Pb3SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PB3."]
    #[inline(always)]
    #[must_use]
    pub fn pb3_sel(&mut self) -> Pb3SelW<Pb3SelSpec> {
        Pb3SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PB3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb3_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb3_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pb3SelSpec;
impl crate::RegisterSpec for Pb3SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb3_sel::R`](R) reader structure"]
impl crate::Readable for Pb3SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pb3_sel::W`](W) writer structure"]
impl crate::Writable for Pb3SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PB3_SEL to value 0"]
impl crate::Resettable for Pb3SelSpec {
    const RESET_VALUE: u32 = 0;
}
