#[doc = "Register `PA3_SEL` reader"]
pub type R = crate::R<Pa3SelSpec>;
#[doc = "Register `PA3_SEL` writer"]
pub type W = crate::W<Pa3SelSpec>;
#[doc = "Field `PA3_sel` reader - Select one peripheral signal output for PA3."]
pub type Pa3SelR = crate::FieldReader;
#[doc = "Field `PA3_sel` writer - Select one peripheral signal output for PA3."]
pub type Pa3SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PA3."]
    #[inline(always)]
    pub fn pa3_sel(&self) -> Pa3SelR {
        Pa3SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PA3."]
    #[inline(always)]
    #[must_use]
    pub fn pa3_sel(&mut self) -> Pa3SelW<Pa3SelSpec> {
        Pa3SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PA3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa3_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa3_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pa3SelSpec;
impl crate::RegisterSpec for Pa3SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa3_sel::R`](R) reader structure"]
impl crate::Readable for Pa3SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pa3_sel::W`](W) writer structure"]
impl crate::Writable for Pa3SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PA3_SEL to value 0"]
impl crate::Resettable for Pa3SelSpec {
    const RESET_VALUE: u32 = 0;
}
