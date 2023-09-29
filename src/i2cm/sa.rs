#[doc = "Register `SA` reader"]
pub type R = crate::R<SA_SPEC>;
#[doc = "Register `SA` writer"]
pub type W = crate::W<SA_SPEC>;
#[doc = "Field `RS` reader - Receive and send The R/S bit specifies if the next operation is a receive (high) or transmit (low). 0: Transmit 1: Receive"]
pub type RS_R = crate::BitReader;
#[doc = "Field `RS` writer - Receive and send The R/S bit specifies if the next operation is a receive (high) or transmit (low). 0: Transmit 1: Receive"]
pub type RS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SA` reader - I2C slave address"]
pub type SA_R = crate::FieldReader;
#[doc = "Field `SA` writer - I2C slave address"]
pub type SA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
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
    pub fn rs(&mut self) -> RS_W<SA_SPEC, 0> {
        RS_W::new(self)
    }
    #[doc = "Bits 1:7 - I2C slave address"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<SA_SPEC, 1> {
        SA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2C master slave address This register consists of eight bits, seven address bits (A6-A0), and a receive and send bit, which determines if the next operation is a receive (high) or transmit (low).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SA_SPEC;
impl crate::RegisterSpec for SA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sa::R`](R) reader structure"]
impl crate::Readable for SA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sa::W`](W) writer structure"]
impl crate::Writable for SA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SA to value 0"]
impl crate::Resettable for SA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
