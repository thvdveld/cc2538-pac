#[doc = "Register `CSPCTRL` reader"]
pub struct R(crate::R<CSPCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSPCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSPCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSPCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSPCTRL` writer"]
pub struct W(crate::W<CSPCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSPCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CSPCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSPCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCU_CTRL` reader - CSP MCU control input"]
pub type MCU_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `MCU_CTRL` writer - CSP MCU control input"]
pub type MCU_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSPCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CSP MCU control input"]
    #[inline(always)]
    pub fn mcu_ctrl(&self) -> MCU_CTRL_R {
        MCU_CTRL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CSP MCU control input"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_ctrl(&mut self) -> MCU_CTRL_W<0> {
        MCU_CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSP control bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cspctrl](index.html) module"]
pub struct CSPCTRL_SPEC;
impl crate::RegisterSpec for CSPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cspctrl::R](R) reader structure"]
impl crate::Readable for CSPCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cspctrl::W](W) writer structure"]
impl crate::Writable for CSPCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSPCTRL to value 0"]
impl crate::Resettable for CSPCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
