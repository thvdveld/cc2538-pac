#[doc = "Register `PA2_SEL` reader"]
pub type R = crate::R<PA2_SEL_SPEC>;
#[doc = "Register `PA2_SEL` writer"]
pub type W = crate::W<PA2_SEL_SPEC>;
#[doc = "Field `PA2_sel` reader - Select one peripheral signal output for PA2."]
pub type PA2_SEL_R = crate::FieldReader;
#[doc = "Field `PA2_sel` writer - Select one peripheral signal output for PA2."]
pub type PA2_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PA2."]
    #[inline(always)]
    pub fn pa2_sel(&self) -> PA2_SEL_R {
        PA2_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PA2."]
    #[inline(always)]
    #[must_use]
    pub fn pa2_sel(&mut self) -> PA2_SEL_W<PA2_SEL_SPEC> {
        PA2_SEL_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Peripheral select control for PA2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa2_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa2_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PA2_SEL_SPEC;
impl crate::RegisterSpec for PA2_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa2_sel::R`](R) reader structure"]
impl crate::Readable for PA2_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pa2_sel::W`](W) writer structure"]
impl crate::Writable for PA2_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PA2_SEL to value 0"]
impl crate::Resettable for PA2_SEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
