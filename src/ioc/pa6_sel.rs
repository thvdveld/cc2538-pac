#[doc = "Register `PA6_SEL` reader"]
pub type R = crate::R<Pa6SelSpec>;
#[doc = "Register `PA6_SEL` writer"]
pub type W = crate::W<Pa6SelSpec>;
#[doc = "Field `PA6_sel` reader - Select one peripheral signal output for PA6."]
pub type Pa6SelR = crate::FieldReader;
#[doc = "Field `PA6_sel` writer - Select one peripheral signal output for PA6."]
pub type Pa6SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PA6."]
    #[inline(always)]
    pub fn pa6_sel(&self) -> Pa6SelR {
        Pa6SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PA6."]
    #[inline(always)]
    pub fn pa6_sel(&mut self) -> Pa6SelW<Pa6SelSpec> {
        Pa6SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PA6\n\nYou can [`read`](crate::Reg::read) this register and get [`pa6_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa6_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pa6SelSpec;
impl crate::RegisterSpec for Pa6SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa6_sel::R`](R) reader structure"]
impl crate::Readable for Pa6SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pa6_sel::W`](W) writer structure"]
impl crate::Writable for Pa6SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PA6_SEL to value 0"]
impl crate::Resettable for Pa6SelSpec {
    const RESET_VALUE: u32 = 0;
}
