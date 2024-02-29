#[doc = "Register `PA4_SEL` reader"]
pub type R = crate::R<Pa4SelSpec>;
#[doc = "Register `PA4_SEL` writer"]
pub type W = crate::W<Pa4SelSpec>;
#[doc = "Field `PA4_sel` reader - Select one peripheral signal output for PA4."]
pub type Pa4SelR = crate::FieldReader;
#[doc = "Field `PA4_sel` writer - Select one peripheral signal output for PA4."]
pub type Pa4SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PA4."]
    #[inline(always)]
    pub fn pa4_sel(&self) -> Pa4SelR {
        Pa4SelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PA4."]
    #[inline(always)]
    #[must_use]
    pub fn pa4_sel(&mut self) -> Pa4SelW<Pa4SelSpec> {
        Pa4SelW::new(self, 0)
    }
}
#[doc = "Peripheral select control for PA4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa4_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa4_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pa4SelSpec;
impl crate::RegisterSpec for Pa4SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa4_sel::R`](R) reader structure"]
impl crate::Readable for Pa4SelSpec {}
#[doc = "`write(|w| ..)` method takes [`pa4_sel::W`](W) writer structure"]
impl crate::Writable for Pa4SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PA4_SEL to value 0"]
impl crate::Resettable for Pa4SelSpec {
    const RESET_VALUE: u32 = 0;
}
