#[doc = "Register `DIECFG0` reader"]
pub struct R(crate::R<DIECFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIECFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIECFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIECFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHIPID` reader - Register copy of configuration bits Three clock cycles after reset is released, this bit field is equal to the field with the same name in the information page."]
pub struct CHIPID_R(crate::FieldReader<u16, u16>);
impl CHIPID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CHIPID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHIPID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_SEL_GATE_EN_N` reader - Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
pub struct CLK_SEL_GATE_EN_N_R(crate::FieldReader<bool, bool>);
impl CLK_SEL_GATE_EN_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_SEL_GATE_EN_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SEL_GATE_EN_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_SIZE` reader - Register copy of configuration bits Three clock cycles after reset is released, this bit field is equal to the field with the same name in the information page."]
pub struct SRAM_SIZE_R(crate::FieldReader<u8, u8>);
impl SRAM_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRAM_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_SIZE` reader - Register copy of configuration bits Three clock cycles after reset is released, this bit field is equal to the field with the same name in the information page."]
pub struct FLASH_SIZE_R(crate::FieldReader<u8, u8>);
impl FLASH_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLASH_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_ENABLE` reader - Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
pub struct USB_ENABLE_R(crate::FieldReader<bool, bool>);
impl USB_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASS_ERASE_ENABLE` reader - Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
pub struct MASS_ERASE_ENABLE_R(crate::FieldReader<bool, bool>);
impl MASS_ERASE_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASS_ERASE_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASS_ERASE_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_FWT_N` reader - Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
pub struct LOCK_FWT_N_R(crate::FieldReader<bool, bool>);
impl LOCK_FWT_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_FWT_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_FWT_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_IP_N` reader - Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
pub struct LOCK_IP_N_R(crate::FieldReader<bool, bool>);
impl LOCK_IP_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_IP_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_IP_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 16:31 - Register copy of configuration bits Three clock cycles after reset is released, this bit field is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn chipid(&self) -> CHIPID_R {
        CHIPID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 10 - Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn clk_sel_gate_en_n(&self) -> CLK_SEL_GATE_EN_N_R {
        CLK_SEL_GATE_EN_N_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 7:9 - Register copy of configuration bits Three clock cycles after reset is released, this bit field is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn sram_size(&self) -> SRAM_SIZE_R {
        SRAM_SIZE_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Register copy of configuration bits Three clock cycles after reset is released, this bit field is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn flash_size(&self) -> FLASH_SIZE_R {
        FLASH_SIZE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3 - Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn usb_enable(&self) -> USB_ENABLE_R {
        USB_ENABLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn mass_erase_enable(&self) -> MASS_ERASE_ENABLE_R {
        MASS_ERASE_ENABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn lock_fwt_n(&self) -> LOCK_FWT_N_R {
        LOCK_FWT_N_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn lock_ip_n(&self) -> LOCK_IP_N_R {
        LOCK_IP_N_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diecfg0](index.html) module"]
pub struct DIECFG0_SPEC;
impl crate::RegisterSpec for DIECFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diecfg0::R](R) reader structure"]
impl crate::Readable for DIECFG0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIECFG0 to value 0"]
impl crate::Resettable for DIECFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
