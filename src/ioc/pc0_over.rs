#[doc = "Register `PC0_OVER` reader"]
pub type R = crate::R<PC0_OVER_SPEC>;
#[doc = "Register `PC0_OVER` writer"]
pub type W = crate::W<PC0_OVER_SPEC>;
#[doc = "Field `PC0_over` reader - 0: output disable 1: oe - output enable"]
pub type PC0_OVER_R = crate::BitReader;
#[doc = "Field `PC0_over` writer - 0: output disable 1: oe - output enable"]
pub type PC0_OVER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 3 - 0: output disable 1: oe - output enable"]
    #[inline(always)]
    pub fn pc0_over(&self) -> PC0_OVER_R {
        PC0_OVER_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 0: output disable 1: oe - output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pc0_over(&mut self) -> PC0_OVER_W<PC0_OVER_SPEC, 3> {
        PC0_OVER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This is the overide configuration register for each pad. PC0 has high drive capability.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc0_over::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc0_over::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PC0_OVER_SPEC;
impl crate::RegisterSpec for PC0_OVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc0_over::R`](R) reader structure"]
impl crate::Readable for PC0_OVER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pc0_over::W`](W) writer structure"]
impl crate::Writable for PC0_OVER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PC0_OVER to value 0x04"]
impl crate::Resettable for PC0_OVER_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
