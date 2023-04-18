#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR` writer"]
pub struct W(crate::W<IMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR_SPEC>;
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
impl From<crate::W<IMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATAIM` reader - Data interrupt mask 1: The data received or data requested interrupt is sent to the interrupt controller when the DATARIS bit in the I2CSRIS register is set. 0: The DATARIS interrupt is surpressed and not sent to the interrupt controller."]
pub type DATAIM_R = crate::BitReader<bool>;
#[doc = "Field `DATAIM` writer - Data interrupt mask 1: The data received or data requested interrupt is sent to the interrupt controller when the DATARIS bit in the I2CSRIS register is set. 0: The DATARIS interrupt is surpressed and not sent to the interrupt controller."]
pub type DATAIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `STARTIM` reader - Start condition interrupt mask 1: The START condition interrupt is sent to the interrupt controller when the STARTRIS bit in the I2CSRIS register is set. 0: The STARTRIS interrupt is supressed and not sent to the interrupt controller."]
pub type STARTIM_R = crate::BitReader<bool>;
#[doc = "Field `STOPIM` reader - Stop condition interrupt mask 1: The STOP condition interrupt is sent to the interrupt controller when the STOPRIS bit in the I2CSRIS register is set. 0: The STOPRIS interrupt is supressed and not sent to the interrupt controller."]
pub type STOPIM_R = crate::BitReader<bool>;
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
    pub fn dataim(&mut self) -> DATAIM_W<0> {
        DATAIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C slave interrupt mask This register controls whether a raw interrupt is promoted to a controller interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr::W](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
