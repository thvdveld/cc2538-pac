#[doc = "Register `AGCCTRL1` reader"]
pub type R = crate::R<AGCCTRL1_SPEC>;
#[doc = "Register `AGCCTRL1` writer"]
pub type W = crate::W<AGCCTRL1_SPEC>;
#[doc = "Field `AGC_REF` reader - Target value for the AGC control loop, given in 1-dB steps"]
pub type AGC_REF_R = crate::FieldReader;
#[doc = "Field `AGC_REF` writer - Target value for the AGC control loop, given in 1-dB steps"]
pub type AGC_REF_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Target value for the AGC control loop, given in 1-dB steps"]
    #[inline(always)]
    pub fn agc_ref(&self) -> AGC_REF_R {
        AGC_REF_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Target value for the AGC control loop, given in 1-dB steps"]
    #[inline(always)]
    #[must_use]
    pub fn agc_ref(&mut self) -> AGC_REF_W<AGCCTRL1_SPEC> {
        AGC_REF_W::new(self, 0)
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
#[doc = "AGC reference level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agcctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agcctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AGCCTRL1_SPEC;
impl crate::RegisterSpec for AGCCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`agcctrl1::R`](R) reader structure"]
impl crate::Readable for AGCCTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`agcctrl1::W`](W) writer structure"]
impl crate::Writable for AGCCTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AGCCTRL1 to value 0"]
impl crate::Resettable for AGCCTRL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
