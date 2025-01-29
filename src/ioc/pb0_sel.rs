#[doc = "Register `PB0_SEL` reader"]
pub type R = crate::R<Pb0SelSpec>;
#[doc = "Register `PB0_SEL` writer"]
pub type W = crate::W<Pb0SelSpec>;
#[doc = "Field `PB0_sel` reader - Select one peripheral signal output for PB0."]
pub type Pb0SelR = crate::FieldReader;
#[doc = "Field `PB0_sel` writer - Select one peripheral signal output for PB0."]
pub type Pb0SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PB0."]
    #[inline(always)]
    pub fn pb0_sel(&self) -> Pb0SelR {
        Pb0SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PB0."]
    #[inline(always)]
    pub fn pb0_sel(&mut self) -> Pb0SelW<Pb0SelSpec> {
        Pb0SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PB0\n\nYou can [`read`](crate::Reg::read) this register and get [`pb0_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb0_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pb0SelSpec;
impl crate::RegisterSpec for Pb0SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb0_sel::R`](R) reader structure"]
impl crate::Readable for Pb0SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pb0_sel::W`](W) writer structure"]
impl crate::Writable for Pb0SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PB0_SEL to value 0"]
impl crate::Resettable for Pb0SelSpec {
    const RESET_VALUE: u32 = 0;
}
