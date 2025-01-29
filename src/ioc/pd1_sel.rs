#[doc = "Register `PD1_SEL` reader"]
pub type R = crate::R<Pd1SelSpec>;
#[doc = "Register `PD1_SEL` writer"]
pub type W = crate::W<Pd1SelSpec>;
#[doc = "Field `PD1_sel` reader - Select one peripheral signal output for PD1."]
pub type Pd1SelR = crate::FieldReader;
#[doc = "Field `PD1_sel` writer - Select one peripheral signal output for PD1."]
pub type Pd1SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PD1."]
    #[inline(always)]
    pub fn pd1_sel(&self) -> Pd1SelR {
        Pd1SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PD1."]
    #[inline(always)]
    pub fn pd1_sel(&mut self) -> Pd1SelW<Pd1SelSpec> {
        Pd1SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PD1\n\nYou can [`read`](crate::Reg::read) this register and get [`pd1_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd1_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pd1SelSpec;
impl crate::RegisterSpec for Pd1SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd1_sel::R`](R) reader structure"]
impl crate::Readable for Pd1SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pd1_sel::W`](W) writer structure"]
impl crate::Writable for Pd1SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PD1_SEL to value 0"]
impl crate::Resettable for Pd1SelSpec {
    const RESET_VALUE: u32 = 0;
}
