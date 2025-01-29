#[doc = "Register `PC0_SEL` reader"]
pub type R = crate::R<Pc0SelSpec>;
#[doc = "Register `PC0_SEL` writer"]
pub type W = crate::W<Pc0SelSpec>;
#[doc = "Field `PC0_sel` reader - Select one peripheral signal output for PC0."]
pub type Pc0SelR = crate::FieldReader;
#[doc = "Field `PC0_sel` writer - Select one peripheral signal output for PC0."]
pub type Pc0SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PC0."]
    #[inline(always)]
    pub fn pc0_sel(&self) -> Pc0SelR {
        Pc0SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PC0."]
    #[inline(always)]
    pub fn pc0_sel(&mut self) -> Pc0SelW<Pc0SelSpec> {
        Pc0SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PC0\n\nYou can [`read`](crate::Reg::read) this register and get [`pc0_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc0_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pc0SelSpec;
impl crate::RegisterSpec for Pc0SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc0_sel::R`](R) reader structure"]
impl crate::Readable for Pc0SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pc0_sel::W`](W) writer structure"]
impl crate::Writable for Pc0SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PC0_SEL to value 0"]
impl crate::Resettable for Pc0SelSpec {
    const RESET_VALUE: u32 = 0;
}
