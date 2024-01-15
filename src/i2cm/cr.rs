#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `LPBK` reader - I2C loopback 1: The controller in a test mode loopback configuration. 0: Normal operation"]
pub type LPBK_R = crate::BitReader;
#[doc = "Field `LPBK` writer - I2C loopback 1: The controller in a test mode loopback configuration. 0: Normal operation"]
pub type LPBK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFE` reader - I2C master function enable 1: Master mode is enabled. 0: Master mode is disabled."]
pub type MFE_R = crate::BitReader;
#[doc = "Field `MFE` writer - I2C master function enable 1: Master mode is enabled. 0: Master mode is disabled."]
pub type MFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFE` reader - I2C slave function enable 1: Slave mode is enabled. 0: Slave mode is disabled."]
pub type SFE_R = crate::BitReader;
#[doc = "Field `SFE` writer - I2C slave function enable 1: Slave mode is enabled. 0: Slave mode is disabled."]
pub type SFE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C loopback 1: The controller in a test mode loopback configuration. 0: Normal operation"]
    #[inline(always)]
    pub fn lpbk(&self) -> LPBK_R {
        LPBK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - I2C master function enable 1: Master mode is enabled. 0: Master mode is disabled."]
    #[inline(always)]
    pub fn mfe(&self) -> MFE_R {
        MFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C slave function enable 1: Slave mode is enabled. 0: Slave mode is disabled."]
    #[inline(always)]
    pub fn sfe(&self) -> SFE_R {
        SFE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C loopback 1: The controller in a test mode loopback configuration. 0: Normal operation"]
    #[inline(always)]
    #[must_use]
    pub fn lpbk(&mut self) -> LPBK_W<CR_SPEC> {
        LPBK_W::new(self, 0)
    }
    #[doc = "Bit 4 - I2C master function enable 1: Master mode is enabled. 0: Master mode is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn mfe(&mut self) -> MFE_W<CR_SPEC> {
        MFE_W::new(self, 4)
    }
    #[doc = "Bit 5 - I2C slave function enable 1: Slave mode is enabled. 0: Slave mode is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn sfe(&mut self) -> SFE_W<CR_SPEC> {
        SFE_W::new(self, 5)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2C master configuration This register configures the mode (master or slave) and sets the interface for test mode loopback.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: u32 = 0;
}
