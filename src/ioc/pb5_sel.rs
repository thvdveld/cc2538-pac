#[doc = "Register `PB5_SEL` reader"]
pub type R = crate::R<Pb5SelSpec>;
#[doc = "Register `PB5_SEL` writer"]
pub type W = crate::W<Pb5SelSpec>;
#[doc = "Field `PB5_sel` reader - Select one peripheral signal output for PB5."]
pub type Pb5SelR = crate::FieldReader;
#[doc = "Field `PB5_sel` writer - Select one peripheral signal output for PB5."]
pub type Pb5SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PB5."]
    #[inline(always)]
    pub fn pb5_sel(&self) -> Pb5SelR {
        Pb5SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PB5."]
    #[inline(always)]
    pub fn pb5_sel(&mut self) -> Pb5SelW<Pb5SelSpec> {
        Pb5SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PB5\n\nYou can [`read`](crate::Reg::read) this register and get [`pb5_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb5_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pb5SelSpec;
impl crate::RegisterSpec for Pb5SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb5_sel::R`](R) reader structure"]
impl crate::Readable for Pb5SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pb5_sel::W`](W) writer structure"]
impl crate::Writable for Pb5SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PB5_SEL to value 0"]
impl crate::Resettable for Pb5SelSpec {
    const RESET_VALUE: u32 = 0;
}
