#[doc = "Register `ATEST` reader"]
pub type R = crate::R<ATEST_SPEC>;
#[doc = "Register `ATEST` writer"]
pub type W = crate::W<ATEST_SPEC>;
#[doc = "Field `ATEST_CTRL` reader - Controls the analog test mode: 00 0000: Disabled 00 0001: Enables the temperature sensor (see also the CCTEST_TR0 register description). Other values reserved."]
pub type ATEST_CTRL_R = crate::FieldReader;
#[doc = "Field `ATEST_CTRL` writer - Controls the analog test mode: 00 0000: Disabled 00 0001: Enables the temperature sensor (see also the CCTEST_TR0 register description). Other values reserved."]
pub type ATEST_CTRL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Controls the analog test mode: 00 0000: Disabled 00 0001: Enables the temperature sensor (see also the CCTEST_TR0 register description). Other values reserved."]
    #[inline(always)]
    pub fn atest_ctrl(&self) -> ATEST_CTRL_R {
        ATEST_CTRL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Controls the analog test mode: 00 0000: Disabled 00 0001: Enables the temperature sensor (see also the CCTEST_TR0 register description). Other values reserved."]
    #[inline(always)]
    #[must_use]
    pub fn atest_ctrl(&mut self) -> ATEST_CTRL_W<ATEST_SPEC, 0> {
        ATEST_CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Analog test control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atest::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atest::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ATEST_SPEC;
impl crate::RegisterSpec for ATEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atest::R`](R) reader structure"]
impl crate::Readable for ATEST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`atest::W`](W) writer structure"]
impl crate::Writable for ATEST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ATEST to value 0"]
impl crate::Resettable for ATEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
