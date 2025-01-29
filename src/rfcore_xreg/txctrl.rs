#[doc = "Register `TXCTRL` reader"]
pub type R = crate::R<TxctrlSpec>;
#[doc = "Register `TXCTRL` writer"]
pub type W = crate::W<TxctrlSpec>;
#[doc = "Field `TXMIX_CURRENT` reader - Transmit mixers core current Current increases with increasing setting."]
pub type TxmixCurrentR = crate::FieldReader;
#[doc = "Field `TXMIX_CURRENT` writer - Transmit mixers core current Current increases with increasing setting."]
pub type TxmixCurrentW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DAC_DC` reader - Adjusts the DC level to the TX mixer."]
pub type DacDcR = crate::FieldReader;
#[doc = "Field `DAC_DC` writer - Adjusts the DC level to the TX mixer."]
pub type DacDcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DAC_CURR` reader - Change the current in the DAC."]
pub type DacCurrR = crate::FieldReader;
#[doc = "Field `DAC_CURR` writer - Change the current in the DAC."]
pub type DacCurrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - Transmit mixers core current Current increases with increasing setting."]
    #[inline(always)]
    pub fn txmix_current(&self) -> TxmixCurrentR {
        TxmixCurrentR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Adjusts the DC level to the TX mixer."]
    #[inline(always)]
    pub fn dac_dc(&self) -> DacDcR {
        DacDcR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Change the current in the DAC."]
    #[inline(always)]
    pub fn dac_curr(&self) -> DacCurrR {
        DacCurrR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Transmit mixers core current Current increases with increasing setting."]
    #[inline(always)]
    pub fn txmix_current(&mut self) -> TxmixCurrentW<TxctrlSpec> {
        TxmixCurrentW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Adjusts the DC level to the TX mixer."]
    #[inline(always)]
    pub fn dac_dc(&mut self) -> DacDcW<TxctrlSpec> {
        DacDcW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Change the current in the DAC."]
    #[inline(always)]
    pub fn dac_curr(&mut self) -> DacCurrW<TxctrlSpec> {
        DacCurrW::new(self, 4)
    }
}
#[doc = "Controls the TX settings\n\nYou can [`read`](crate::Reg::read) this register and get [`txctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxctrlSpec;
impl crate::RegisterSpec for TxctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txctrl::R`](R) reader structure"]
impl crate::Readable for TxctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`txctrl::W`](W) writer structure"]
impl crate::Writable for TxctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXCTRL to value 0"]
impl crate::Resettable for TxctrlSpec {
    const RESET_VALUE: u32 = 0;
}
