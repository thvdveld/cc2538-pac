#[doc = "Register `SA` reader"]
pub struct R(crate::R<SA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SA` writer"]
pub struct W(crate::W<SA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SA_SPEC>;
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
impl From<crate::W<SA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RS` reader - Receive and send The R/S bit specifies if the next operation is a receive (high) or transmit (low). 0: Transmit 1: Receive"]
pub type RS_R = crate::BitReader<bool>;
#[doc = "Field `RS` writer - Receive and send The R/S bit specifies if the next operation is a receive (high) or transmit (low). 0: Transmit 1: Receive"]
pub type RS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SA_SPEC, bool, O>;
#[doc = "Field `SA` reader - I2C slave address"]
pub type SA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SA` writer - I2C slave address"]
pub type SA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SA_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - Receive and send The R/S bit specifies if the next operation is a receive (high) or transmit (low). 0: Transmit 1: Receive"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - I2C slave address"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Receive and send The R/S bit specifies if the next operation is a receive (high) or transmit (low). 0: Transmit 1: Receive"]
    #[inline(always)]
    #[must_use]
    pub fn rs(&mut self) -> RS_W<0> {
        RS_W::new(self)
    }
    #[doc = "Bits 1:7 - I2C slave address"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<1> {
        SA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C master slave address This register consists of eight bits, seven address bits (A6-A0), and a receive and send bit, which determines if the next operation is a receive (high) or transmit (low).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sa](index.html) module"]
pub struct SA_SPEC;
impl crate::RegisterSpec for SA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sa::R](R) reader structure"]
impl crate::Readable for SA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sa::W](W) writer structure"]
impl crate::Writable for SA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SA to value 0"]
impl crate::Resettable for SA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
