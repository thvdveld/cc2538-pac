#[doc = "Register `IEV` reader"]
pub struct R(crate::R<IEV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEV` writer"]
pub struct W(crate::W<IEV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEV_SPEC>;
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
impl From<crate::W<IEV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IEV` reader - Bits set: Rising edges or high levels on corresponding pin trigger interrupts Bits cleared: Falling edges or low levels on corresponding pin trigger interrupts"]
pub type IEV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IEV` writer - Bits set: Rising edges or high levels on corresponding pin trigger interrupts Bits cleared: Falling edges or low levels on corresponding pin trigger interrupts"]
pub type IEV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IEV_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Bits set: Rising edges or high levels on corresponding pin trigger interrupts Bits cleared: Falling edges or low levels on corresponding pin trigger interrupts"]
    #[inline(always)]
    pub fn iev(&self) -> IEV_R {
        IEV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bits set: Rising edges or high levels on corresponding pin trigger interrupts Bits cleared: Falling edges or low levels on corresponding pin trigger interrupts"]
    #[inline(always)]
    pub fn iev(&mut self) -> IEV_W<0> {
        IEV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The IEV register is the interrupt event register. Bits set to high in IEV configure the corresponding pin to detect rising edges or high levels, depending on the corresponding bit value in IS. Clearing a bit configures the pin to detect falling edges or low levels, depending on the corresponding bit value in IS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iev](index.html) module"]
pub struct IEV_SPEC;
impl crate::RegisterSpec for IEV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iev::R](R) reader structure"]
impl crate::Readable for IEV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iev::W](W) writer structure"]
impl crate::Writable for IEV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IEV to value 0"]
impl crate::Resettable for IEV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
