#[doc = "Register `SA` reader"]
pub type R = crate::R<SaSpec>;
#[doc = "Register `SA` writer"]
pub type W = crate::W<SaSpec>;
#[doc = "Field `RS` reader - Receive and send The R/S bit specifies if the next operation is a receive (high) or transmit (low). 0: Transmit 1: Receive"]
pub type RsR = crate::BitReader;
#[doc = "Field `RS` writer - Receive and send The R/S bit specifies if the next operation is a receive (high) or transmit (low). 0: Transmit 1: Receive"]
pub type RsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SA` reader - I2C slave address"]
pub type SaR = crate::FieldReader;
#[doc = "Field `SA` writer - I2C slave address"]
pub type SaW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Receive and send The R/S bit specifies if the next operation is a receive (high) or transmit (low). 0: Transmit 1: Receive"]
    #[inline(always)]
    pub fn rs(&self) -> RsR {
        RsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - I2C slave address"]
    #[inline(always)]
    pub fn sa(&self) -> SaR {
        SaR::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Receive and send The R/S bit specifies if the next operation is a receive (high) or transmit (low). 0: Transmit 1: Receive"]
    #[inline(always)]
    #[must_use]
    pub fn rs(&mut self) -> RsW<SaSpec> {
        RsW::new(self, 0)
    }
    #[doc = "Bits 1:7 - I2C slave address"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SaW<SaSpec> {
        SaW::new(self, 1)
    }
}
#[doc = "I2C master slave address This register consists of eight bits, seven address bits (A6-A0), and a receive and send bit, which determines if the next operation is a receive (high) or transmit (low).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaSpec;
impl crate::RegisterSpec for SaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sa::R`](R) reader structure"]
impl crate::Readable for SaSpec {}
#[doc = "`write(|w| ..)` method takes [`sa::W`](W) writer structure"]
impl crate::Writable for SaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SA to value 0"]
impl crate::Resettable for SaSpec {
    const RESET_VALUE: u32 = 0;
}
