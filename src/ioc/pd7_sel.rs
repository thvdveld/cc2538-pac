#[doc = "Register `PD7_SEL` reader"]
pub type R = crate::R<Pd7SelSpec>;
#[doc = "Register `PD7_SEL` writer"]
pub type W = crate::W<Pd7SelSpec>;
#[doc = "Field `PD7_sel` reader - Select one peripheral signal output for PD7."]
pub type Pd7SelR = crate::FieldReader;
#[doc = "Field `PD7_sel` writer - Select one peripheral signal output for PD7."]
pub type Pd7SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PD7."]
    #[inline(always)]
    pub fn pd7_sel(&self) -> Pd7SelR {
        Pd7SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PD7."]
    #[inline(always)]
    pub fn pd7_sel(&mut self) -> Pd7SelW<Pd7SelSpec> {
        Pd7SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PD7\n\nYou can [`read`](crate::Reg::read) this register and get [`pd7_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd7_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pd7SelSpec;
impl crate::RegisterSpec for Pd7SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd7_sel::R`](R) reader structure"]
impl crate::Readable for Pd7SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pd7_sel::W`](W) writer structure"]
impl crate::Writable for Pd7SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PD7_SEL to value 0"]
impl crate::Resettable for Pd7SelSpec {
    const RESET_VALUE: u32 = 0;
}
