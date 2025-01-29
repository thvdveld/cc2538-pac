#[doc = "Register `PA0_SEL` reader"]
pub type R = crate::R<Pa0SelSpec>;
#[doc = "Register `PA0_SEL` writer"]
pub type W = crate::W<Pa0SelSpec>;
#[doc = "Field `PA0_sel` reader - Select one peripheral signal output for PA0."]
pub type Pa0SelR = crate::FieldReader;
#[doc = "Field `PA0_sel` writer - Select one peripheral signal output for PA0."]
pub type Pa0SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PA0."]
    #[inline(always)]
    pub fn pa0_sel(&self) -> Pa0SelR {
        Pa0SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PA0."]
    #[inline(always)]
    pub fn pa0_sel(&mut self) -> Pa0SelW<Pa0SelSpec> {
        Pa0SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PA0\n\nYou can [`read`](crate::Reg::read) this register and get [`pa0_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa0_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pa0SelSpec;
impl crate::RegisterSpec for Pa0SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa0_sel::R`](R) reader structure"]
impl crate::Readable for Pa0SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pa0_sel::W`](W) writer structure"]
impl crate::Writable for Pa0SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PA0_SEL to value 0"]
impl crate::Resettable for Pa0SelSpec {
    const RESET_VALUE: u32 = 0;
}
