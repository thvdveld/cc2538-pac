#[doc = "Register `IWE` reader"]
pub type R = crate::R<IWE_SPEC>;
#[doc = "Register `IWE` writer"]
pub type W = crate::W<IWE_SPEC>;
#[doc = "Field `PORT_A_IWE` reader - 1: Enable port A wake-up interrupt. 0: Disable port A wake-up interrupt."]
pub type PORT_A_IWE_R = crate::BitReader;
#[doc = "Field `PORT_A_IWE` writer - 1: Enable port A wake-up interrupt. 0: Disable port A wake-up interrupt."]
pub type PORT_A_IWE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PORT_B_IWE` reader - 1: Enable port B wake-up interrupt. 0: Disable port B wake-up interrupt."]
pub type PORT_B_IWE_R = crate::BitReader;
#[doc = "Field `PORT_B_IWE` writer - 1: Enable port B wake-up interrupt. 0: Disable port B wake-up interrupt."]
pub type PORT_B_IWE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PORT_C_IWE` reader - 1: Enable port C wake-up interrupt. 0: Disable port C wake-up interrupt."]
pub type PORT_C_IWE_R = crate::BitReader;
#[doc = "Field `PORT_C_IWE` writer - 1: Enable port C wake-up interrupt. 0: Disable port C wake-up interrupt."]
pub type PORT_C_IWE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PORT_D_IWE` reader - 1: Enable port D wake-up interrupt. 0: Disable port D wake-up interrupt."]
pub type PORT_D_IWE_R = crate::BitReader;
#[doc = "Field `PORT_D_IWE` writer - 1: Enable port D wake-up interrupt. 0: Disable port D wake-up interrupt."]
pub type PORT_D_IWE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_IWE` reader - 1: Enable USB wake-up interrupt. 0: Disable USB wake-up interrupt."]
pub type USB_IWE_R = crate::BitReader;
#[doc = "Field `USB_IWE` writer - 1: Enable USB wake-up interrupt. 0: Disable USB wake-up interrupt."]
pub type USB_IWE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SM_TIMER_IWE` reader - 1: Enable SM Timer wake-up interrupt. 0: Disable SM Timer wake-up interrupt."]
pub type SM_TIMER_IWE_R = crate::BitReader;
#[doc = "Field `SM_TIMER_IWE` writer - 1: Enable SM Timer wake-up interrupt. 0: Disable SM Timer wake-up interrupt."]
pub type SM_TIMER_IWE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 1: Enable port A wake-up interrupt. 0: Disable port A wake-up interrupt."]
    #[inline(always)]
    pub fn port_a_iwe(&self) -> PORT_A_IWE_R {
        PORT_A_IWE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Enable port B wake-up interrupt. 0: Disable port B wake-up interrupt."]
    #[inline(always)]
    pub fn port_b_iwe(&self) -> PORT_B_IWE_R {
        PORT_B_IWE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Enable port C wake-up interrupt. 0: Disable port C wake-up interrupt."]
    #[inline(always)]
    pub fn port_c_iwe(&self) -> PORT_C_IWE_R {
        PORT_C_IWE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Enable port D wake-up interrupt. 0: Disable port D wake-up interrupt."]
    #[inline(always)]
    pub fn port_d_iwe(&self) -> PORT_D_IWE_R {
        PORT_D_IWE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Enable USB wake-up interrupt. 0: Disable USB wake-up interrupt."]
    #[inline(always)]
    pub fn usb_iwe(&self) -> USB_IWE_R {
        USB_IWE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Enable SM Timer wake-up interrupt. 0: Disable SM Timer wake-up interrupt."]
    #[inline(always)]
    pub fn sm_timer_iwe(&self) -> SM_TIMER_IWE_R {
        SM_TIMER_IWE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Enable port A wake-up interrupt. 0: Disable port A wake-up interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn port_a_iwe(&mut self) -> PORT_A_IWE_W<IWE_SPEC, 0> {
        PORT_A_IWE_W::new(self)
    }
    #[doc = "Bit 1 - 1: Enable port B wake-up interrupt. 0: Disable port B wake-up interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn port_b_iwe(&mut self) -> PORT_B_IWE_W<IWE_SPEC, 1> {
        PORT_B_IWE_W::new(self)
    }
    #[doc = "Bit 2 - 1: Enable port C wake-up interrupt. 0: Disable port C wake-up interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn port_c_iwe(&mut self) -> PORT_C_IWE_W<IWE_SPEC, 2> {
        PORT_C_IWE_W::new(self)
    }
    #[doc = "Bit 3 - 1: Enable port D wake-up interrupt. 0: Disable port D wake-up interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn port_d_iwe(&mut self) -> PORT_D_IWE_W<IWE_SPEC, 3> {
        PORT_D_IWE_W::new(self)
    }
    #[doc = "Bit 4 - 1: Enable USB wake-up interrupt. 0: Disable USB wake-up interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_iwe(&mut self) -> USB_IWE_W<IWE_SPEC, 4> {
        USB_IWE_W::new(self)
    }
    #[doc = "Bit 5 - 1: Enable SM Timer wake-up interrupt. 0: Disable SM Timer wake-up interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sm_timer_iwe(&mut self) -> SM_TIMER_IWE_W<IWE_SPEC, 5> {
        SM_TIMER_IWE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This register controls interrupt wake-up.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IWE_SPEC;
impl crate::RegisterSpec for IWE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwe::R`](R) reader structure"]
impl crate::Readable for IWE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iwe::W`](W) writer structure"]
impl crate::Writable for IWE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IWE to value 0"]
impl crate::Resettable for IWE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
