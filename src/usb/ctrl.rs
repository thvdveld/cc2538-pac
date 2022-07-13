#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLLOCKED` reader - PLL lock status. The PLL is locked when USB_CTRL.PLLLOCKED is 1."]
pub type PLLLOCKED_R = crate::BitReader<bool>;
#[doc = "Field `PLLEN` reader - 48 MHz USB PLL enable When this bit is set, the 48 MHz PLL is started. Software must avoid access to other USB registers before the PLL has locked; that is, USB_CTRL.PLLLOCKED is 1. This bit can be set only when USB_CTRL.USBEN is 1. The PLL must be disabled before entering PM1 when suspended, and must be re-enabled when resuming operation."]
pub type PLLEN_R = crate::BitReader<bool>;
#[doc = "Field `PLLEN` writer - 48 MHz USB PLL enable When this bit is set, the 48 MHz PLL is started. Software must avoid access to other USB registers before the PLL has locked; that is, USB_CTRL.PLLLOCKED is 1. This bit can be set only when USB_CTRL.USBEN is 1. The PLL must be disabled before entering PM1 when suspended, and must be re-enabled when resuming operation."]
pub type PLLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `USBEN` reader - USB enable The USB controller is reset when this bit is cleared"]
pub type USBEN_R = crate::BitReader<bool>;
#[doc = "Field `USBEN` writer - USB enable The USB controller is reset when this bit is cleared"]
pub type USBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 7 - PLL lock status. The PLL is locked when USB_CTRL.PLLLOCKED is 1."]
    #[inline(always)]
    pub fn plllocked(&self) -> PLLLOCKED_R {
        PLLLOCKED_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 1 - 48 MHz USB PLL enable When this bit is set, the 48 MHz PLL is started. Software must avoid access to other USB registers before the PLL has locked; that is, USB_CTRL.PLLLOCKED is 1. This bit can be set only when USB_CTRL.USBEN is 1. The PLL must be disabled before entering PM1 when suspended, and must be re-enabled when resuming operation."]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - USB enable The USB controller is reset when this bit is cleared"]
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 48 MHz USB PLL enable When this bit is set, the 48 MHz PLL is started. Software must avoid access to other USB registers before the PLL has locked; that is, USB_CTRL.PLLLOCKED is 1. This bit can be set only when USB_CTRL.USBEN is 1. The PLL must be disabled before entering PM1 when suspended, and must be re-enabled when resuming operation."]
    #[inline(always)]
    pub fn pllen(&mut self) -> PLLEN_W<1> {
        PLLEN_W::new(self)
    }
    #[doc = "Bit 0 - USB enable The USB controller is reset when this bit is cleared"]
    #[inline(always)]
    pub fn usben(&mut self) -> USBEN_W<0> {
        USBEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB peripheral control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
