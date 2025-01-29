#[doc = "Register `PD5_SEL` reader"]
pub type R = crate::R<Pd5SelSpec>;
#[doc = "Register `PD5_SEL` writer"]
pub type W = crate::W<Pd5SelSpec>;
#[doc = "Field `PD5_sel` reader - Select one peripheral signal output for PD5."]
pub type Pd5SelR = crate::FieldReader;
#[doc = "Field `PD5_sel` writer - Select one peripheral signal output for PD5."]
pub type Pd5SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PD5."]
    #[inline(always)]
    pub fn pd5_sel(&self) -> Pd5SelR {
        Pd5SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PD5."]
    #[inline(always)]
    pub fn pd5_sel(&mut self) -> Pd5SelW<Pd5SelSpec> {
        Pd5SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PD5\n\nYou can [`read`](crate::Reg::read) this register and get [`pd5_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd5_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pd5SelSpec;
impl crate::RegisterSpec for Pd5SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd5_sel::R`](R) reader structure"]
impl crate::Readable for Pd5SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pd5_sel::W`](W) writer structure"]
impl crate::Writable for Pd5SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PD5_SEL to value 0"]
impl crate::Resettable for Pd5SelSpec {
    const RESET_VALUE: u32 = 0;
}
