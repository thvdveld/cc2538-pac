#[doc = "Register `PD0_SEL` reader"]
pub type R = crate::R<Pd0SelSpec>;
#[doc = "Register `PD0_SEL` writer"]
pub type W = crate::W<Pd0SelSpec>;
#[doc = "Field `PD0_sel` reader - Select one peripheral signal output for PD0."]
pub type Pd0SelR = crate::FieldReader;
#[doc = "Field `PD0_sel` writer - Select one peripheral signal output for PD0."]
pub type Pd0SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PD0."]
    #[inline(always)]
    pub fn pd0_sel(&self) -> Pd0SelR {
        Pd0SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PD0."]
    #[inline(always)]
    pub fn pd0_sel(&mut self) -> Pd0SelW<Pd0SelSpec> {
        Pd0SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PD0\n\nYou can [`read`](crate::Reg::read) this register and get [`pd0_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd0_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pd0SelSpec;
impl crate::RegisterSpec for Pd0SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd0_sel::R`](R) reader structure"]
impl crate::Readable for Pd0SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pd0_sel::W`](W) writer structure"]
impl crate::Writable for Pd0SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PD0_SEL to value 0"]
impl crate::Resettable for Pd0SelSpec {
    const RESET_VALUE: u32 = 0;
}
