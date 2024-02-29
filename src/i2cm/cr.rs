#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `LPBK` reader - I2C loopback 1: The controller in a test mode loopback configuration. 0: Normal operation"]
pub type LpbkR = crate::BitReader;
#[doc = "Field `LPBK` writer - I2C loopback 1: The controller in a test mode loopback configuration. 0: Normal operation"]
pub type LpbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFE` reader - I2C master function enable 1: Master mode is enabled. 0: Master mode is disabled."]
pub type MfeR = crate::BitReader;
#[doc = "Field `MFE` writer - I2C master function enable 1: Master mode is enabled. 0: Master mode is disabled."]
pub type MfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFE` reader - I2C slave function enable 1: Slave mode is enabled. 0: Slave mode is disabled."]
pub type SfeR = crate::BitReader;
#[doc = "Field `SFE` writer - I2C slave function enable 1: Slave mode is enabled. 0: Slave mode is disabled."]
pub type SfeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C loopback 1: The controller in a test mode loopback configuration. 0: Normal operation"]
    #[inline(always)]
    pub fn lpbk(&self) -> LpbkR {
        LpbkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - I2C master function enable 1: Master mode is enabled. 0: Master mode is disabled."]
    #[inline(always)]
    pub fn mfe(&self) -> MfeR {
        MfeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C slave function enable 1: Slave mode is enabled. 0: Slave mode is disabled."]
    #[inline(always)]
    pub fn sfe(&self) -> SfeR {
        SfeR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C loopback 1: The controller in a test mode loopback configuration. 0: Normal operation"]
    #[inline(always)]
    #[must_use]
    pub fn lpbk(&mut self) -> LpbkW<CrSpec> {
        LpbkW::new(self, 0)
    }
    #[doc = "Bit 4 - I2C master function enable 1: Master mode is enabled. 0: Master mode is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn mfe(&mut self) -> MfeW<CrSpec> {
        MfeW::new(self, 4)
    }
    #[doc = "Bit 5 - I2C slave function enable 1: Slave mode is enabled. 0: Slave mode is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn sfe(&mut self) -> SfeW<CrSpec> {
        SfeW::new(self, 5)
    }
}
#[doc = "I2C master configuration This register configures the mode (master or slave) and sets the interface for test mode loopback.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0;
}
