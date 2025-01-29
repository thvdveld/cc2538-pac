#[doc = "Register `PC3_SEL` reader"]
pub type R = crate::R<Pc3SelSpec>;
#[doc = "Register `PC3_SEL` writer"]
pub type W = crate::W<Pc3SelSpec>;
#[doc = "Field `PC3_sel` reader - Select one peripheral signal output for PC3."]
pub type Pc3SelR = crate::FieldReader;
#[doc = "Field `PC3_sel` writer - Select one peripheral signal output for PC3."]
pub type Pc3SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PC3."]
    #[inline(always)]
    pub fn pc3_sel(&self) -> Pc3SelR {
        Pc3SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PC3."]
    #[inline(always)]
    pub fn pc3_sel(&mut self) -> Pc3SelW<Pc3SelSpec> {
        Pc3SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PC3\n\nYou can [`read`](crate::Reg::read) this register and get [`pc3_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc3_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pc3SelSpec;
impl crate::RegisterSpec for Pc3SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc3_sel::R`](R) reader structure"]
impl crate::Readable for Pc3SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pc3_sel::W`](W) writer structure"]
impl crate::Writable for Pc3SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PC3_SEL to value 0"]
impl crate::Resettable for Pc3SelSpec {
    const RESET_VALUE: u32 = 0;
}
