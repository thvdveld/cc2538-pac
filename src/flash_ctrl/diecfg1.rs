#[doc = "Register `DIECFG1` reader"]
pub struct R(crate::R<DIECFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIECFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIECFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIECFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GPTM0_EN` reader - 1: GPTM0 is enabled. 0: GPTM0 is permanently disabled."]
pub type GPTM0_EN_R = crate::BitReader<bool>;
#[doc = "Field `GPTM1_EN` reader - 1: GPTM1 is enabled. 0: GPTM1 is permanently disabled."]
pub type GPTM1_EN_R = crate::BitReader<bool>;
#[doc = "Field `GPTM2_EN` reader - 1: GPTM2 is enabled. 0: GPTM2 is permanently disabled."]
pub type GPTM2_EN_R = crate::BitReader<bool>;
#[doc = "Field `GPTM3_EN` reader - 1: GPTM3 is enabled. 0: GPTM3 is permanently disabled."]
pub type GPTM3_EN_R = crate::BitReader<bool>;
#[doc = "Field `SSI0_EN` reader - 1: SSI0 is enabled. 0: SSI0 is permanently disabled."]
pub type SSI0_EN_R = crate::BitReader<bool>;
#[doc = "Field `SSI1_EN` reader - 1: SSI1 is enabled. 0: SSI1 is permanently disabled."]
pub type SSI1_EN_R = crate::BitReader<bool>;
#[doc = "Field `UART0_EN` reader - 1: UART0 is enabled. 0: UART0 is permanently disabled."]
pub type UART0_EN_R = crate::BitReader<bool>;
#[doc = "Field `UART1_EN` reader - 1: UART1 is enabled. 0: UART1 is permanently disabled."]
pub type UART1_EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C_EN` reader - 1: I2C is enabled. 0: I2C is permanently disabled."]
pub type I2C_EN_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - 1: GPTM0 is enabled. 0: GPTM0 is permanently disabled."]
    #[inline(always)]
    pub fn gptm0_en(&self) -> GPTM0_EN_R {
        GPTM0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: GPTM1 is enabled. 0: GPTM1 is permanently disabled."]
    #[inline(always)]
    pub fn gptm1_en(&self) -> GPTM1_EN_R {
        GPTM1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: GPTM2 is enabled. 0: GPTM2 is permanently disabled."]
    #[inline(always)]
    pub fn gptm2_en(&self) -> GPTM2_EN_R {
        GPTM2_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: GPTM3 is enabled. 0: GPTM3 is permanently disabled."]
    #[inline(always)]
    pub fn gptm3_en(&self) -> GPTM3_EN_R {
        GPTM3_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: SSI0 is enabled. 0: SSI0 is permanently disabled."]
    #[inline(always)]
    pub fn ssi0_en(&self) -> SSI0_EN_R {
        SSI0_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: SSI1 is enabled. 0: SSI1 is permanently disabled."]
    #[inline(always)]
    pub fn ssi1_en(&self) -> SSI1_EN_R {
        SSI1_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: UART0 is enabled. 0: UART0 is permanently disabled."]
    #[inline(always)]
    pub fn uart0_en(&self) -> UART0_EN_R {
        UART0_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 1: UART1 is enabled. 0: UART1 is permanently disabled."]
    #[inline(always)]
    pub fn uart1_en(&self) -> UART1_EN_R {
        UART1_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: I2C is enabled. 0: I2C is permanently disabled."]
    #[inline(always)]
    pub fn i2c_en(&self) -> I2C_EN_R {
        I2C_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diecfg1](index.html) module"]
pub struct DIECFG1_SPEC;
impl crate::RegisterSpec for DIECFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diecfg1::R](R) reader structure"]
impl crate::Readable for DIECFG1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIECFG1 to value 0"]
impl crate::Resettable for DIECFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
