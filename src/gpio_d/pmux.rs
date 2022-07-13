#[doc = "Register `PMUX` reader"]
pub struct R(crate::R<PMUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMUX` writer"]
pub struct W(crate::W<PMUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PMUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKOEN` reader - Clock out enable When this bit is set, the 32-kHz clock is routed to either PA\\[0\\]
or PB\\[7\\]
pins. PMUX.CKOPIN selects the pin to use. This overrides the current configuration setting for this pin. The pullup or pulldown is disabled and the direction is set to output for this pin."]
pub type CKOEN_R = crate::BitReader<bool>;
#[doc = "Field `CKOEN` writer - Clock out enable When this bit is set, the 32-kHz clock is routed to either PA\\[0\\]
or PB\\[7\\]
pins. PMUX.CKOPIN selects the pin to use. This overrides the current configuration setting for this pin. The pullup or pulldown is disabled and the direction is set to output for this pin."]
pub type CKOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMUX_SPEC, bool, O>;
#[doc = "Field `CKOPIN` reader - Decouple control pin select This control only has relevance when CKOEN is set. When 0, PA\\[0\\]
becomes the 32-kHz clock output. When 1, PB\\[7\\]
becomes the 32-kHz clock output."]
pub type CKOPIN_R = crate::BitReader<bool>;
#[doc = "Field `CKOPIN` writer - Decouple control pin select This control only has relevance when CKOEN is set. When 0, PA\\[0\\]
becomes the 32-kHz clock output. When 1, PB\\[7\\]
becomes the 32-kHz clock output."]
pub type CKOPIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMUX_SPEC, bool, O>;
#[doc = "Field `DCEN` reader - Decouple control enable When this bit is set, the on-die digital regulator status is routed to either PB\\[1\\]
or PB\\[0\\]
pins. PMUX.DCPIN selects the pin to use. This overrides the current configuration setting for this pin. The pullup or pulldown is disabled and the direction is set to output for this pin."]
pub type DCEN_R = crate::BitReader<bool>;
#[doc = "Field `DCEN` writer - Decouple control enable When this bit is set, the on-die digital regulator status is routed to either PB\\[1\\]
or PB\\[0\\]
pins. PMUX.DCPIN selects the pin to use. This overrides the current configuration setting for this pin. The pullup or pulldown is disabled and the direction is set to output for this pin."]
pub type DCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMUX_SPEC, bool, O>;
#[doc = "Field `DCPIN` reader - Decouple control pin select This control has relevance only when DCEN is set. When 0, PB\\[1\\]
becomes the on-die digital regulator status (1 indicates the on-die digital regulator is active); when 1, PB\\[0\\]
becomes the on-die digital regulator status. NOTE: PB\\[1\\]
and PB\\[0\\]
can also be controlled with other override features. In priority order for PB\\[1\\]: When POR/BOD test mode is active, PB\\[1\\]
becomes the active low brown-out detected indicator. When DCEN is set and DCPIN is not set, PB\\[1\\]
becomes the on-dir digital regulator status. In priority order for PB\\[0\\]: When POR/BOD test mode is active, PB\\[0\\]
becomes the power-on-reset indicator. When DCEN and DCPIN are set, PB\\[0\\]
becomes the on-die digital regulator status."]
pub type DCPIN_R = crate::BitReader<bool>;
#[doc = "Field `DCPIN` writer - Decouple control pin select This control has relevance only when DCEN is set. When 0, PB\\[1\\]
becomes the on-die digital regulator status (1 indicates the on-die digital regulator is active); when 1, PB\\[0\\]
becomes the on-die digital regulator status. NOTE: PB\\[1\\]
and PB\\[0\\]
can also be controlled with other override features. In priority order for PB\\[1\\]: When POR/BOD test mode is active, PB\\[1\\]
becomes the active low brown-out detected indicator. When DCEN is set and DCPIN is not set, PB\\[1\\]
becomes the on-dir digital regulator status. In priority order for PB\\[0\\]: When POR/BOD test mode is active, PB\\[0\\]
becomes the power-on-reset indicator. When DCEN and DCPIN are set, PB\\[0\\]
becomes the on-die digital regulator status."]
pub type DCPIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMUX_SPEC, bool, O>;
impl R {
    #[doc = "Bit 7 - Clock out enable When this bit is set, the 32-kHz clock is routed to either PA\\[0\\]
or PB\\[7\\]
pins. PMUX.CKOPIN selects the pin to use. This overrides the current configuration setting for this pin. The pullup or pulldown is disabled and the direction is set to output for this pin."]
    #[inline(always)]
    pub fn ckoen(&self) -> CKOEN_R {
        CKOEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 4 - Decouple control pin select This control only has relevance when CKOEN is set. When 0, PA\\[0\\]
becomes the 32-kHz clock output. When 1, PB\\[7\\]
becomes the 32-kHz clock output."]
    #[inline(always)]
    pub fn ckopin(&self) -> CKOPIN_R {
        CKOPIN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Decouple control enable When this bit is set, the on-die digital regulator status is routed to either PB\\[1\\]
or PB\\[0\\]
pins. PMUX.DCPIN selects the pin to use. This overrides the current configuration setting for this pin. The pullup or pulldown is disabled and the direction is set to output for this pin."]
    #[inline(always)]
    pub fn dcen(&self) -> DCEN_R {
        DCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
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
    pub fn dcpin(&self) -> DCPIN_R {
        DCPIN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Clock out enable When this bit is set, the 32-kHz clock is routed to either PA\\[0\\]
or PB\\[7\\]
pins. PMUX.CKOPIN selects the pin to use. This overrides the current configuration setting for this pin. The pullup or pulldown is disabled and the direction is set to output for this pin."]
    #[inline(always)]
    pub fn ckoen(&mut self) -> CKOEN_W<7> {
        CKOEN_W::new(self)
    }
    #[doc = "Bit 4 - Decouple control pin select This control only has relevance when CKOEN is set. When 0, PA\\[0\\]
becomes the 32-kHz clock output. When 1, PB\\[7\\]
becomes the 32-kHz clock output."]
    #[inline(always)]
    pub fn ckopin(&mut self) -> CKOPIN_W<4> {
        CKOPIN_W::new(self)
    }
    #[doc = "Bit 3 - Decouple control enable When this bit is set, the on-die digital regulator status is routed to either PB\\[1\\]
or PB\\[0\\]
pins. PMUX.DCPIN selects the pin to use. This overrides the current configuration setting for this pin. The pullup or pulldown is disabled and the direction is set to output for this pin."]
    #[inline(always)]
    pub fn dcen(&mut self) -> DCEN_W<3> {
        DCEN_W::new(self)
    }
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
    pub fn dcpin(&mut self) -> DCPIN_W<0> {
        DCPIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The PMUX register can be used to output external decouple control and clock_32k on I/O pins. Decouple control can be output on specific PB pins and clock_32k can be output on a specific PA or PB pin. These features override the current setting of the selected pin when enabled. The pin is set to output, pull-up and -down disabled, and analog mode disabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmux](index.html) module"]
pub struct PMUX_SPEC;
impl crate::RegisterSpec for PMUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmux::R](R) reader structure"]
impl crate::Readable for PMUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmux::W](W) writer structure"]
impl crate::Writable for PMUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMUX to value 0"]
impl crate::Resettable for PMUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
