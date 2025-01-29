#[doc = "Register `PB7_SEL` reader"]
pub type R = crate::R<Pb7SelSpec>;
#[doc = "Register `PB7_SEL` writer"]
pub type W = crate::W<Pb7SelSpec>;
#[doc = "Field `PB7_sel` reader - Select one peripheral signal output for PB7."]
pub type Pb7SelR = crate::FieldReader;
#[doc = "Field `PB7_sel` writer - Select one peripheral signal output for PB7."]
pub type Pb7SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PB7."]
    #[inline(always)]
    pub fn pb7_sel(&self) -> Pb7SelR {
        Pb7SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PB7."]
    #[inline(always)]
    pub fn pb7_sel(&mut self) -> Pb7SelW<Pb7SelSpec> {
        Pb7SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PB7\n\nYou can [`read`](crate::Reg::read) this register and get [`pb7_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb7_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pb7SelSpec;
impl crate::RegisterSpec for Pb7SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb7_sel::R`](R) reader structure"]
impl crate::Readable for Pb7SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pb7_sel::W`](W) writer structure"]
impl crate::Writable for Pb7SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PB7_SEL to value 0"]
impl crate::Resettable for Pb7SelSpec {
    const RESET_VALUE: u32 = 0;
}
