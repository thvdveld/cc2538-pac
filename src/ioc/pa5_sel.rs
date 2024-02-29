#[doc = "Register `PA5_SEL` reader"]
pub type R = crate::R<Pa5SelSpec>;
#[doc = "Register `PA5_SEL` writer"]
pub type W = crate::W<Pa5SelSpec>;
#[doc = "Field `PA5_sel` reader - Select one peripheral signal output for PA5."]
pub type Pa5SelR = crate::FieldReader;
#[doc = "Field `PA5_sel` writer - Select one peripheral signal output for PA5."]
pub type Pa5SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PA5."]
    #[inline(always)]
    pub fn pa5_sel(&self) -> Pa5SelR {
        Pa5SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PA5."]
    #[inline(always)]
    #[must_use]
    pub fn pa5_sel(&mut self) -> Pa5SelW<Pa5SelSpec> {
        Pa5SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PA5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa5_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa5_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pa5SelSpec;
impl crate::RegisterSpec for Pa5SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa5_sel::R`](R) reader structure"]
impl crate::Readable for Pa5SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pa5_sel::W`](W) writer structure"]
impl crate::Writable for Pa5SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PA5_SEL to value 0"]
impl crate::Resettable for Pa5SelSpec {
    const RESET_VALUE: u32 = 0;
}
