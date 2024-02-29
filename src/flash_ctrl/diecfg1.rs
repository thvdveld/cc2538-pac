#[doc = "Register `DIECFG1` reader"]
pub type R = crate::R<Diecfg1Spec>;
#[doc = "Field `GPTM0_EN` reader - 1: GPTM0 is enabled. 0: GPTM0 is permanently disabled."]
pub type Gptm0EnR = crate::BitReader;
#[doc = "Field `GPTM1_EN` reader - 1: GPTM1 is enabled. 0: GPTM1 is permanently disabled."]
pub type Gptm1EnR = crate::BitReader;
#[doc = "Field `GPTM2_EN` reader - 1: GPTM2 is enabled. 0: GPTM2 is permanently disabled."]
pub type Gptm2EnR = crate::BitReader;
#[doc = "Field `GPTM3_EN` reader - 1: GPTM3 is enabled. 0: GPTM3 is permanently disabled."]
pub type Gptm3EnR = crate::BitReader;
#[doc = "Field `SSI0_EN` reader - 1: SSI0 is enabled. 0: SSI0 is permanently disabled."]
pub type Ssi0EnR = crate::BitReader;
#[doc = "Field `SSI1_EN` reader - 1: SSI1 is enabled. 0: SSI1 is permanently disabled."]
pub type Ssi1EnR = crate::BitReader;
#[doc = "Field `UART0_EN` reader - 1: UART0 is enabled. 0: UART0 is permanently disabled."]
pub type Uart0EnR = crate::BitReader;
#[doc = "Field `UART1_EN` reader - 1: UART1 is enabled. 0: UART1 is permanently disabled."]
pub type Uart1EnR = crate::BitReader;
#[doc = "Field `I2C_EN` reader - 1: I2C is enabled. 0: I2C is permanently disabled."]
pub type I2cEnR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1: GPTM0 is enabled. 0: GPTM0 is permanently disabled."]
    #[inline(always)]
    pub fn gptm0_en(&self) -> Gptm0EnR {
        Gptm0EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: GPTM1 is enabled. 0: GPTM1 is permanently disabled."]
    #[inline(always)]
    pub fn gptm1_en(&self) -> Gptm1EnR {
        Gptm1EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: GPTM2 is enabled. 0: GPTM2 is permanently disabled."]
    #[inline(always)]
    pub fn gptm2_en(&self) -> Gptm2EnR {
        Gptm2EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: GPTM3 is enabled. 0: GPTM3 is permanently disabled."]
    #[inline(always)]
    pub fn gptm3_en(&self) -> Gptm3EnR {
        Gptm3EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: SSI0 is enabled. 0: SSI0 is permanently disabled."]
    #[inline(always)]
    pub fn ssi0_en(&self) -> Ssi0EnR {
        Ssi0EnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: SSI1 is enabled. 0: SSI1 is permanently disabled."]
    #[inline(always)]
    pub fn ssi1_en(&self) -> Ssi1EnR {
        Ssi1EnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: UART0 is enabled. 0: UART0 is permanently disabled."]
    #[inline(always)]
    pub fn uart0_en(&self) -> Uart0EnR {
        Uart0EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 1: UART1 is enabled. 0: UART1 is permanently disabled."]
    #[inline(always)]
    pub fn uart1_en(&self) -> Uart1EnR {
        Uart1EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: I2C is enabled. 0: I2C is permanently disabled."]
    #[inline(always)]
    pub fn i2c_en(&self) -> I2cEnR {
        I2cEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diecfg1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diecfg1Spec;
impl crate::RegisterSpec for Diecfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diecfg1::R`](R) reader structure"]
impl crate::Readable for Diecfg1Spec {}
#[doc = "`reset()` method sets DIECFG1 to value 0"]
impl crate::Resettable for Diecfg1Spec {
    const RESET_VALUE: u32 = 0;
}
