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
#[doc = "Field `STOPIM` reader - Stop condition interrupt mask 1: The STOP condition interrupt is sent to the interrupt controller when the STOPRIS bit in the I2CSRIS register is set. 0: The STOPRIS interrupt is supressed and not sent to the interrupt controller."]
pub struct STOPIM_R(crate::FieldReader<bool, bool>);
impl STOPIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STOPIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOPIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STARTIM` reader - Start condition interrupt mask 1: The START condition interrupt is sent to the interrupt controller when the STARTRIS bit in the I2CSRIS register is set. 0: The STARTRIS interrupt is supressed and not sent to the interrupt controller."]
pub struct STARTIM_R(crate::FieldReader<bool, bool>);
impl STARTIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STARTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STARTIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAIM` reader - Data interrupt mask 1: The data received or data requested interrupt is sent to the interrupt controller when the DATARIS bit in the I2CSRIS register is set. 0: The DATARIS interrupt is surpressed and not sent to the interrupt controller."]
pub struct DATAIM_R(crate::FieldReader<bool, bool>);
impl DATAIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATAIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAIM` writer - Data interrupt mask 1: The data received or data requested interrupt is sent to the interrupt controller when the DATARIS bit in the I2CSRIS register is set. 0: The DATARIS interrupt is surpressed and not sent to the interrupt controller."]
pub struct DATAIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAIM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Stop condition interrupt mask 1: The STOP condition interrupt is sent to the interrupt controller when the STOPRIS bit in the I2CSRIS register is set. 0: The STOPRIS interrupt is supressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn stopim(&self) -> STOPIM_R {
        STOPIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start condition interrupt mask 1: The START condition interrupt is sent to the interrupt controller when the STARTRIS bit in the I2CSRIS register is set. 0: The STARTRIS interrupt is supressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn startim(&self) -> STARTIM_R {
        STARTIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Data interrupt mask 1: The data received or data requested interrupt is sent to the interrupt controller when the DATARIS bit in the I2CSRIS register is set. 0: The DATARIS interrupt is surpressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn dataim(&self) -> DATAIM_R {
        DATAIM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data interrupt mask 1: The data received or data requested interrupt is sent to the interrupt controller when the DATARIS bit in the I2CSRIS register is set. 0: The DATARIS interrupt is surpressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn dataim(&mut self) -> DATAIM_W {
        DATAIM_W { w: self }
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
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
