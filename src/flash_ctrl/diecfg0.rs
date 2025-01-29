#[doc = "Register `DIECFG0` reader"]
pub type R = crate::R<Diecfg0Spec>;
#[doc = "Field `LOCK_IP_N` reader - Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
pub type LockIpNR = crate::BitReader;
#[doc = "Field `LOCK_FWT_N` reader - Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
pub type LockFwtNR = crate::BitReader;
#[doc = "Field `MASS_ERASE_ENABLE` reader - Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
pub type MassEraseEnableR = crate::BitReader;
#[doc = "Field `USB_ENABLE` reader - Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
pub type UsbEnableR = crate::BitReader;
#[doc = "Field `FLASH_SIZE` reader - Register copy of configuration bits Three clock cycles after reset is released, this bit field is equal to the field with the same name in the information page."]
pub type FlashSizeR = crate::FieldReader;
#[doc = "Field `SRAM_SIZE` reader - Register copy of configuration bits Three clock cycles after reset is released, this bit field is equal to the field with the same name in the information page."]
pub type SramSizeR = crate::FieldReader;
#[doc = "Field `CLK_SEL_GATE_EN_N` reader - Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
pub type ClkSelGateEnNR = crate::BitReader;
#[doc = "Field `CHIPID` reader - Register copy of configuration bits Three clock cycles after reset is released, this bit field is equal to the field with the same name in the information page."]
pub type ChipidR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn lock_ip_n(&self) -> LockIpNR {
        LockIpNR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn lock_fwt_n(&self) -> LockFwtNR {
        LockFwtNR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn mass_erase_enable(&self) -> MassEraseEnableR {
        MassEraseEnableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn usb_enable(&self) -> UsbEnableR {
        UsbEnableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Register copy of configuration bits Three clock cycles after reset is released, this bit field is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn flash_size(&self) -> FlashSizeR {
        FlashSizeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - Register copy of configuration bits Three clock cycles after reset is released, this bit field is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn sram_size(&self) -> SramSizeR {
        SramSizeR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn clk_sel_gate_en_n(&self) -> ClkSelGateEnNR {
        ClkSelGateEnNR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Register copy of configuration bits Three clock cycles after reset is released, this bit field is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn chipid(&self) -> ChipidR {
        ChipidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538.\n\nYou can [`read`](crate::Reg::read) this register and get [`diecfg0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diecfg0Spec;
impl crate::RegisterSpec for Diecfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diecfg0::R`](R) reader structure"]
impl crate::Readable for Diecfg0Spec {}
#[doc = "`reset()` method sets DIECFG0 to value 0"]
impl crate::Resettable for Diecfg0Spec {
    const RESET_VALUE: u32 = 0;
}
