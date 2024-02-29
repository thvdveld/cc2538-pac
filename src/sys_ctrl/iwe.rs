#[doc = "Register `IWE` reader"]
pub type R = crate::R<IweSpec>;
#[doc = "Register `IWE` writer"]
pub type W = crate::W<IweSpec>;
#[doc = "Field `PORT_A_IWE` reader - 1: Enable port A wake-up interrupt. 0: Disable port A wake-up interrupt."]
pub type PortAIweR = crate::BitReader;
#[doc = "Field `PORT_A_IWE` writer - 1: Enable port A wake-up interrupt. 0: Disable port A wake-up interrupt."]
pub type PortAIweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_B_IWE` reader - 1: Enable port B wake-up interrupt. 0: Disable port B wake-up interrupt."]
pub type PortBIweR = crate::BitReader;
#[doc = "Field `PORT_B_IWE` writer - 1: Enable port B wake-up interrupt. 0: Disable port B wake-up interrupt."]
pub type PortBIweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_C_IWE` reader - 1: Enable port C wake-up interrupt. 0: Disable port C wake-up interrupt."]
pub type PortCIweR = crate::BitReader;
#[doc = "Field `PORT_C_IWE` writer - 1: Enable port C wake-up interrupt. 0: Disable port C wake-up interrupt."]
pub type PortCIweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_D_IWE` reader - 1: Enable port D wake-up interrupt. 0: Disable port D wake-up interrupt."]
pub type PortDIweR = crate::BitReader;
#[doc = "Field `PORT_D_IWE` writer - 1: Enable port D wake-up interrupt. 0: Disable port D wake-up interrupt."]
pub type PortDIweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_IWE` reader - 1: Enable USB wake-up interrupt. 0: Disable USB wake-up interrupt."]
pub type UsbIweR = crate::BitReader;
#[doc = "Field `USB_IWE` writer - 1: Enable USB wake-up interrupt. 0: Disable USB wake-up interrupt."]
pub type UsbIweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SM_TIMER_IWE` reader - 1: Enable SM Timer wake-up interrupt. 0: Disable SM Timer wake-up interrupt."]
pub type SmTimerIweR = crate::BitReader;
#[doc = "Field `SM_TIMER_IWE` writer - 1: Enable SM Timer wake-up interrupt. 0: Disable SM Timer wake-up interrupt."]
pub type SmTimerIweW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: Enable port A wake-up interrupt. 0: Disable port A wake-up interrupt."]
    #[inline(always)]
    pub fn port_a_iwe(&self) -> PortAIweR {
        PortAIweR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Enable port B wake-up interrupt. 0: Disable port B wake-up interrupt."]
    #[inline(always)]
    pub fn port_b_iwe(&self) -> PortBIweR {
        PortBIweR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Enable port C wake-up interrupt. 0: Disable port C wake-up interrupt."]
    #[inline(always)]
    pub fn port_c_iwe(&self) -> PortCIweR {
        PortCIweR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Enable port D wake-up interrupt. 0: Disable port D wake-up interrupt."]
    #[inline(always)]
    pub fn port_d_iwe(&self) -> PortDIweR {
        PortDIweR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Enable USB wake-up interrupt. 0: Disable USB wake-up interrupt."]
    #[inline(always)]
    pub fn usb_iwe(&self) -> UsbIweR {
        UsbIweR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Enable SM Timer wake-up interrupt. 0: Disable SM Timer wake-up interrupt."]
    #[inline(always)]
    pub fn sm_timer_iwe(&self) -> SmTimerIweR {
        SmTimerIweR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Enable port A wake-up interrupt. 0: Disable port A wake-up interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn port_a_iwe(&mut self) -> PortAIweW<IweSpec> {
        PortAIweW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Enable port B wake-up interrupt. 0: Disable port B wake-up interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn port_b_iwe(&mut self) -> PortBIweW<IweSpec> {
        PortBIweW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: Enable port C wake-up interrupt. 0: Disable port C wake-up interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn port_c_iwe(&mut self) -> PortCIweW<IweSpec> {
        PortCIweW::new(self, 2)
    }
    #[doc = "Bit 3 - 1: Enable port D wake-up interrupt. 0: Disable port D wake-up interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn port_d_iwe(&mut self) -> PortDIweW<IweSpec> {
        PortDIweW::new(self, 3)
    }
    #[doc = "Bit 4 - 1: Enable USB wake-up interrupt. 0: Disable USB wake-up interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_iwe(&mut self) -> UsbIweW<IweSpec> {
        UsbIweW::new(self, 4)
    }
    #[doc = "Bit 5 - 1: Enable SM Timer wake-up interrupt. 0: Disable SM Timer wake-up interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sm_timer_iwe(&mut self) -> SmTimerIweW<IweSpec> {
        SmTimerIweW::new(self, 5)
    }
}
#[doc = "This register controls interrupt wake-up.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IweSpec;
impl crate::RegisterSpec for IweSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwe::R`](R) reader structure"]
impl crate::Readable for IweSpec {}
#[doc = "`write(|w| ..)` method takes [`iwe::W`](W) writer structure"]
impl crate::Writable for IweSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IWE to value 0"]
impl crate::Resettable for IweSpec {
    const RESET_VALUE: u32 = 0;
}
