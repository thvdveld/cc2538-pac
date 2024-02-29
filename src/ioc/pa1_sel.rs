#[doc = "Register `PA1_SEL` reader"]
pub type R = crate::R<Pa1SelSpec>;
#[doc = "Register `PA1_SEL` writer"]
pub type W = crate::W<Pa1SelSpec>;
#[doc = "Field `PA1_sel` reader - Select one peripheral signal output for PA1."]
pub type Pa1SelR = crate::FieldReader;
#[doc = "Field `PA1_sel` writer - Select one peripheral signal output for PA1."]
pub type Pa1SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PA1."]
    #[inline(always)]
    pub fn pa1_sel(&self) -> Pa1SelR {
        Pa1SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PA1."]
    #[inline(always)]
    #[must_use]
    pub fn pa1_sel(&mut self) -> Pa1SelW<Pa1SelSpec> {
        Pa1SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PA1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa1_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa1_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pa1SelSpec;
impl crate::RegisterSpec for Pa1SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa1_sel::R`](R) reader structure"]
impl crate::Readable for Pa1SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pa1_sel::W`](W) writer structure"]
impl crate::Writable for Pa1SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PA1_SEL to value 0"]
impl crate::Resettable for Pa1SelSpec {
    const RESET_VALUE: u32 = 0;
}
