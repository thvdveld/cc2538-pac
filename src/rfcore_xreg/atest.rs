#[doc = "Register `ATEST` reader"]
pub type R = crate::R<AtestSpec>;
#[doc = "Register `ATEST` writer"]
pub type W = crate::W<AtestSpec>;
#[doc = "Field `ATEST_CTRL` reader - Controls the analog test mode: 00 0000: Disabled 00 0001: Enables the temperature sensor (see also the CCTEST_TR0 register description). Other values reserved."]
pub type AtestCtrlR = crate::FieldReader;
#[doc = "Field `ATEST_CTRL` writer - Controls the analog test mode: 00 0000: Disabled 00 0001: Enables the temperature sensor (see also the CCTEST_TR0 register description). Other values reserved."]
pub type AtestCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Controls the analog test mode: 00 0000: Disabled 00 0001: Enables the temperature sensor (see also the CCTEST_TR0 register description). Other values reserved."]
    #[inline(always)]
    pub fn atest_ctrl(&self) -> AtestCtrlR {
        AtestCtrlR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Controls the analog test mode: 00 0000: Disabled 00 0001: Enables the temperature sensor (see also the CCTEST_TR0 register description). Other values reserved."]
    #[inline(always)]
    #[must_use]
    pub fn atest_ctrl(&mut self) -> AtestCtrlW<AtestSpec> {
        AtestCtrlW::new(self, 0)
    }
}
#[doc = "Analog test control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atest::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atest::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtestSpec;
impl crate::RegisterSpec for AtestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atest::R`](R) reader structure"]
impl crate::Readable for AtestSpec {}
#[doc = "`write(|w| ..)` method takes [`atest::W`](W) writer structure"]
impl crate::Writable for AtestSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATEST to value 0"]
impl crate::Resettable for AtestSpec {
    const RESET_VALUE: u32 = 0;
}
