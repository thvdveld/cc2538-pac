#[doc = "Register `PMUX` reader"]
pub type R = crate::R<PmuxSpec>;
#[doc = "Register `PMUX` writer"]
pub type W = crate::W<PmuxSpec>;
#[doc = "Field `DCPIN` reader - Decouple control pin select This control has relevance only when DCEN is set. When 0, PB\\[1\\]
becomes the on-die digital regulator status (1 indicates the on-die digital regulator is active); when 1, PB\\[0\\]
becomes the on-die digital regulator status. NOTE: PB\\[1\\]
and PB\\[0\\]
can also be controlled with other override features. In priority order for PB\\[1\\]: When POR/BOD test mode is active, PB\\[1\\]
becomes the active low brown-out detected indicator. When DCEN is set and DCPIN is not set, PB\\[1\\]
becomes the on-dir digital regulator status. In priority order for PB\\[0\\]: When POR/BOD test mode is active, PB\\[0\\]
becomes the power-on-reset indicator. When DCEN and DCPIN are set, PB\\[0\\]
becomes the on-die digital regulator status."]
pub type DcpinR = crate::BitReader;
#[doc = "Field `DCPIN` writer - Decouple control pin select This control has relevance only when DCEN is set. When 0, PB\\[1\\]
becomes the on-die digital regulator status (1 indicates the on-die digital regulator is active); when 1, PB\\[0\\]
becomes the on-die digital regulator status. NOTE: PB\\[1\\]
and PB\\[0\\]
can also be controlled with other override features. In priority order for PB\\[1\\]: When POR/BOD test mode is active, PB\\[1\\]
becomes the active low brown-out detected indicator. When DCEN is set and DCPIN is not set, PB\\[1\\]
becomes the on-dir digital regulator status. In priority order for PB\\[0\\]: When POR/BOD test mode is active, PB\\[0\\]
becomes the power-on-reset indicator. When DCEN and DCPIN are set, PB\\[0\\]
becomes the on-die digital regulator status."]
pub type DcpinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCEN` reader - Decouple control enable When this bit is set, the on-die digital regulator status is routed to either PB\\[1\\]
or PB\\[0\\]
pins. PMUX.DCPIN selects the pin to use. This overrides the current configuration setting for this pin. The pullup or pulldown is disabled and the direction is set to output for this pin."]
pub type DcenR = crate::BitReader;
#[doc = "Field `DCEN` writer - Decouple control enable When this bit is set, the on-die digital regulator status is routed to either PB\\[1\\]
or PB\\[0\\]
pins. PMUX.DCPIN selects the pin to use. This overrides the current configuration setting for this pin. The pullup or pulldown is disabled and the direction is set to output for this pin."]
pub type DcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKOPIN` reader - Decouple control pin select This control only has relevance when CKOEN is set. When 0, PA\\[0\\]
becomes the 32-kHz clock output. When 1, PB\\[7\\]
becomes the 32-kHz clock output."]
pub type CkopinR = crate::BitReader;
#[doc = "Field `CKOPIN` writer - Decouple control pin select This control only has relevance when CKOEN is set. When 0, PA\\[0\\]
becomes the 32-kHz clock output. When 1, PB\\[7\\]
becomes the 32-kHz clock output."]
pub type CkopinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKOEN` reader - Clock out enable When this bit is set, the 32-kHz clock is routed to either PA\\[0\\]
or PB\\[7\\]
pins. PMUX.CKOPIN selects the pin to use. This overrides the current configuration setting for this pin. The pullup or pulldown is disabled and the direction is set to output for this pin."]
pub type CkoenR = crate::BitReader;
#[doc = "Field `CKOEN` writer - Clock out enable When this bit is set, the 32-kHz clock is routed to either PA\\[0\\]
or PB\\[7\\]
pins. PMUX.CKOPIN selects the pin to use. This overrides the current configuration setting for this pin. The pullup or pulldown is disabled and the direction is set to output for this pin."]
pub type CkoenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Decouple control pin select This control has relevance only when DCEN is set. When 0, PB\\[1\\]
becomes the on-die digital regulator status (1 indicates the on-die digital regulator is active); when 1, PB\\[0\\]
becomes the on-die digital regulator status. NOTE: PB\\[1\\]
and PB\\[0\\]
can also be controlled with other override features. In priority order for PB\\[1\\]: When POR/BOD test mode is active, PB\\[1\\]
becomes the active low brown-out detected indicator. When DCEN is set and DCPIN is not set, PB\\[1\\]
becomes the on-dir digital regulator status. In priority order for PB\\[0\\]: When POR/BOD test mode is active, PB\\[0\\]
becomes the power-on-reset indicator. When DCEN and DCPIN are set, PB\\[0\\]
becomes the on-die digital regulator status."]
    #[inline(always)]
    pub fn dcpin(&self) -> DcpinR {
        DcpinR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Decouple control enable When this bit is set, the on-die digital regulator status is routed to either PB\\[1\\]
or PB\\[0\\]
pins. PMUX.DCPIN selects the pin to use. This overrides the current configuration setting for this pin. The pullup or pulldown is disabled and the direction is set to output for this pin."]
    #[inline(always)]
    pub fn dcen(&self) -> DcenR {
        DcenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Decouple control pin select This control only has relevance when CKOEN is set. When 0, PA\\[0\\]
becomes the 32-kHz clock output. When 1, PB\\[7\\]
becomes the 32-kHz clock output."]
    #[inline(always)]
    pub fn ckopin(&self) -> CkopinR {
        CkopinR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock out enable When this bit is set, the 32-kHz clock is routed to either PA\\[0\\]
or PB\\[7\\]
pins. PMUX.CKOPIN selects the pin to use. This overrides the current configuration setting for this pin. The pullup or pulldown is disabled and the direction is set to output for this pin."]
    #[inline(always)]
    pub fn ckoen(&self) -> CkoenR {
        CkoenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Decouple control pin select This control has relevance only when DCEN is set. When 0, PB\\[1\\]
becomes the on-die digital regulator status (1 indicates the on-die digital regulator is active); when 1, PB\\[0\\]
becomes the on-die digital regulator status. NOTE: PB\\[1\\]
and PB\\[0\\]
can also be controlled with other override features. In priority order for PB\\[1\\]: When POR/BOD test mode is active, PB\\[1\\]
becomes the active low brown-out detected indicator. When DCEN is set and DCPIN is not set, PB\\[1\\]
becomes the on-dir digital regulator status. In priority order for PB\\[0\\]: When POR/BOD test mode is active, PB\\[0\\]
becomes the power-on-reset indicator. When DCEN and DCPIN are set, PB\\[0\\]
becomes the on-die digital regulator status."]
    #[inline(always)]
    pub fn dcpin(&mut self) -> DcpinW<PmuxSpec> {
        DcpinW::new(self, 0)
    }
    #[doc = "Bit 3 - Decouple control enable When this bit is set, the on-die digital regulator status is routed to either PB\\[1\\]
or PB\\[0\\]
pins. PMUX.DCPIN selects the pin to use. This overrides the current configuration setting for this pin. The pullup or pulldown is disabled and the direction is set to output for this pin."]
    #[inline(always)]
    pub fn dcen(&mut self) -> DcenW<PmuxSpec> {
        DcenW::new(self, 3)
    }
    #[doc = "Bit 4 - Decouple control pin select This control only has relevance when CKOEN is set. When 0, PA\\[0\\]
becomes the 32-kHz clock output. When 1, PB\\[7\\]
becomes the 32-kHz clock output."]
    #[inline(always)]
    pub fn ckopin(&mut self) -> CkopinW<PmuxSpec> {
        CkopinW::new(self, 4)
    }
    #[doc = "Bit 7 - Clock out enable When this bit is set, the 32-kHz clock is routed to either PA\\[0\\]
or PB\\[7\\]
pins. PMUX.CKOPIN selects the pin to use. This overrides the current configuration setting for this pin. The pullup or pulldown is disabled and the direction is set to output for this pin."]
    #[inline(always)]
    pub fn ckoen(&mut self) -> CkoenW<PmuxSpec> {
        CkoenW::new(self, 7)
    }
}
#[doc = "The PMUX register can be used to output external decouple control and clock_32k on I/O pins. Decouple control can be output on specific PB pins and clock_32k can be output on a specific PA or PB pin. These features override the current setting of the selected pin when enabled. The pin is set to output, pull-up and -down disabled, and analog mode disabled.\n\nYou can [`read`](crate::Reg::read) this register and get [`pmux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuxSpec;
impl crate::RegisterSpec for PmuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmux::R`](R) reader structure"]
impl crate::Readable for PmuxSpec {}
#[doc = "`write(|w| ..)` method takes [`pmux::W`](W) writer structure"]
impl crate::Writable for PmuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUX to value 0"]
impl crate::Resettable for PmuxSpec {
    const RESET_VALUE: u32 = 0;
}
