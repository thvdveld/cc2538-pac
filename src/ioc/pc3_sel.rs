#[doc = "Register `PC3_SEL` reader"]
pub type R = crate::R<PC3_SEL_SPEC>;
#[doc = "Register `PC3_SEL` writer"]
pub type W = crate::W<PC3_SEL_SPEC>;
#[doc = "Field `PC3_sel` reader - Select one peripheral signal output for PC3."]
pub type PC3_SEL_R = crate::FieldReader;
#[doc = "Field `PC3_sel` writer - Select one peripheral signal output for PC3."]
pub type PC3_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PC3."]
    #[inline(always)]
    pub fn pc3_sel(&self) -> PC3_SEL_R {
        PC3_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PC3."]
    #[inline(always)]
    #[must_use]
    pub fn pc3_sel(&mut self) -> PC3_SEL_W<PC3_SEL_SPEC> {
        PC3_SEL_W::new(self, 0)
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
#[doc = "Peripheral select control for PC3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc3_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc3_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PC3_SEL_SPEC;
impl crate::RegisterSpec for PC3_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc3_sel::R`](R) reader structure"]
impl crate::Readable for PC3_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pc3_sel::W`](W) writer structure"]
impl crate::Writable for PC3_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PC3_SEL to value 0"]
impl crate::Resettable for PC3_SEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
