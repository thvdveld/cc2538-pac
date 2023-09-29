#[doc = "Register `IMR` reader"]
pub type R = crate::R<IMR_SPEC>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<IMR_SPEC>;
#[doc = "Field `DATAIM` reader - Data interrupt mask 1: The data received or data requested interrupt is sent to the interrupt controller when the DATARIS bit in the I2CSRIS register is set. 0: The DATARIS interrupt is surpressed and not sent to the interrupt controller."]
pub type DATAIM_R = crate::BitReader;
#[doc = "Field `DATAIM` writer - Data interrupt mask 1: The data received or data requested interrupt is sent to the interrupt controller when the DATARIS bit in the I2CSRIS register is set. 0: The DATARIS interrupt is surpressed and not sent to the interrupt controller."]
pub type DATAIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STARTIM` reader - Start condition interrupt mask 1: The START condition interrupt is sent to the interrupt controller when the STARTRIS bit in the I2CSRIS register is set. 0: The STARTRIS interrupt is supressed and not sent to the interrupt controller."]
pub type STARTIM_R = crate::BitReader;
#[doc = "Field `STOPIM` reader - Stop condition interrupt mask 1: The STOP condition interrupt is sent to the interrupt controller when the STOPRIS bit in the I2CSRIS register is set. 0: The STOPRIS interrupt is supressed and not sent to the interrupt controller."]
pub type STOPIM_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data interrupt mask 1: The data received or data requested interrupt is sent to the interrupt controller when the DATARIS bit in the I2CSRIS register is set. 0: The DATARIS interrupt is surpressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn dataim(&self) -> DATAIM_R {
        DATAIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start condition interrupt mask 1: The START condition interrupt is sent to the interrupt controller when the STARTRIS bit in the I2CSRIS register is set. 0: The STARTRIS interrupt is supressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn startim(&self) -> STARTIM_R {
        STARTIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop condition interrupt mask 1: The STOP condition interrupt is sent to the interrupt controller when the STOPRIS bit in the I2CSRIS register is set. 0: The STOPRIS interrupt is supressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn stopim(&self) -> STOPIM_R {
        STOPIM_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data interrupt mask 1: The data received or data requested interrupt is sent to the interrupt controller when the DATARIS bit in the I2CSRIS register is set. 0: The DATARIS interrupt is surpressed and not sent to the interrupt controller."]
    #[inline(always)]
    #[must_use]
    pub fn dataim(&mut self) -> DATAIM_W<IMR_SPEC, 0> {
        DATAIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2C slave interrupt mask This register controls whether a raw interrupt is promoted to a controller interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for IMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
