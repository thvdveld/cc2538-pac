#[doc = "Register `PD2_SEL` reader"]
pub type R = crate::R<Pd2SelSpec>;
#[doc = "Register `PD2_SEL` writer"]
pub type W = crate::W<Pd2SelSpec>;
#[doc = "Field `PD2_sel` reader - Select one peripheral signal output for PD2."]
pub type Pd2SelR = crate::FieldReader;
#[doc = "Field `PD2_sel` writer - Select one peripheral signal output for PD2."]
pub type Pd2SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PD2."]
    #[inline(always)]
    pub fn pd2_sel(&self) -> Pd2SelR {
        Pd2SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PD2."]
    #[inline(always)]
    pub fn pd2_sel(&mut self) -> Pd2SelW<Pd2SelSpec> {
        Pd2SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PD2\n\nYou can [`read`](crate::Reg::read) this register and get [`pd2_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd2_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pd2SelSpec;
impl crate::RegisterSpec for Pd2SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd2_sel::R`](R) reader structure"]
impl crate::Readable for Pd2SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pd2_sel::W`](W) writer structure"]
impl crate::Writable for Pd2SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PD2_SEL to value 0"]
impl crate::Resettable for Pd2SelSpec {
    const RESET_VALUE: u32 = 0;
}
