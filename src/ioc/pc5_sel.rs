#[doc = "Register `PC5_SEL` reader"]
pub type R = crate::R<PC5_SEL_SPEC>;
#[doc = "Register `PC5_SEL` writer"]
pub type W = crate::W<PC5_SEL_SPEC>;
#[doc = "Field `PC5_sel` reader - Select one peripheral signal output for PC5."]
pub type PC5_SEL_R = crate::FieldReader;
#[doc = "Field `PC5_sel` writer - Select one peripheral signal output for PC5."]
pub type PC5_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PC5."]
    #[inline(always)]
    pub fn pc5_sel(&self) -> PC5_SEL_R {
        PC5_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PC5."]
    #[inline(always)]
    #[must_use]
    pub fn pc5_sel(&mut self) -> PC5_SEL_W<PC5_SEL_SPEC> {
        PC5_SEL_W::new(self, 0)
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
#[doc = "Peripheral select control for PC5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc5_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc5_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PC5_SEL_SPEC;
impl crate::RegisterSpec for PC5_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc5_sel::R`](R) reader structure"]
impl crate::Readable for PC5_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pc5_sel::W`](W) writer structure"]
impl crate::Writable for PC5_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PC5_SEL to value 0"]
impl crate::Resettable for PC5_SEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
