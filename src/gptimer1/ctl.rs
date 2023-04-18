#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAEN` reader - GPTM Timer A enable 0: Timer A is disabled. 1: Timer A is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
pub type TAEN_R = crate::BitReader<bool>;
#[doc = "Field `TAEN` writer - GPTM Timer A enable 0: Timer A is disabled. 1: Timer A is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
pub type TAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `TASTALL` reader - GPTM Timer A stall enable 0: Timer A continues counting while the processor is halted by the debugger. 1: Timer A freezes counting while the processor is halted by the debugger."]
pub type TASTALL_R = crate::BitReader<bool>;
#[doc = "Field `TASTALL` writer - GPTM Timer A stall enable 0: Timer A continues counting while the processor is halted by the debugger. 1: Timer A freezes counting while the processor is halted by the debugger."]
pub type TASTALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `TAEVENT` reader - GPTM Timer A event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
pub type TAEVENT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TAEVENT` writer - GPTM Timer A event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
pub type TAEVENT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `TAOTE` reader - GPTM Timer A output trigger enable 0: The ADC trigger of output Timer A is disabled. 1: The ADC trigger of output Timer A is enabled."]
pub type TAOTE_R = crate::BitReader<bool>;
#[doc = "Field `TAOTE` writer - GPTM Timer A output trigger enable 0: The ADC trigger of output Timer A is disabled. 1: The ADC trigger of output Timer A is enabled."]
pub type TAOTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `TAPWML` reader - GPTM Timer A PWM output level 0: Output is unaffected. 1: Output is inverted."]
pub type TAPWML_R = crate::BitReader<bool>;
#[doc = "Field `TAPWML` writer - GPTM Timer A PWM output level 0: Output is unaffected. 1: Output is inverted."]
pub type TAPWML_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `TBEN` reader - GPTM Timer B enable 0: Timer B is disabled. 1: Timer B is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
pub type TBEN_R = crate::BitReader<bool>;
#[doc = "Field `TBEN` writer - GPTM Timer B enable 0: Timer B is disabled. 1: Timer B is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
pub type TBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `TBSTALL` reader - GPTM Timer B stall enable 0: Timer B continues counting while the processor is halted by the debugger. 1: Timer B freezes counting while the processor is halted by the debugger."]
pub type TBSTALL_R = crate::BitReader<bool>;
#[doc = "Field `TBSTALL` writer - GPTM Timer B stall enable 0: Timer B continues counting while the processor is halted by the debugger. 1: Timer B freezes counting while the processor is halted by the debugger."]
pub type TBSTALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `TBEVENT` reader - GPTM Timer B event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
pub type TBEVENT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TBEVENT` writer - GPTM Timer B event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
pub type TBEVENT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `TBOTE` reader - GPTM Timer B output trigger enable 0: The ADC trigger of output Timer B is disabled. 1: The ADC trigger of output Timer B is enabled."]
pub type TBOTE_R = crate::BitReader<bool>;
#[doc = "Field `TBOTE` writer - GPTM Timer B output trigger enable 0: The ADC trigger of output Timer B is disabled. 1: The ADC trigger of output Timer B is enabled."]
pub type TBOTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `TBPWML` reader - GPTM Timer B PWM output level 0: Output is unaffected. 1: Output is inverted."]
pub type TBPWML_R = crate::BitReader<bool>;
#[doc = "Field `TBPWML` writer - GPTM Timer B PWM output level 0: Output is unaffected. 1: Output is inverted."]
pub type TBPWML_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - GPTM Timer A enable 0: Timer A is disabled. 1: Timer A is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
    #[inline(always)]
    pub fn taen(&self) -> TAEN_R {
        TAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPTM Timer A stall enable 0: Timer A continues counting while the processor is halted by the debugger. 1: Timer A freezes counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn tastall(&self) -> TASTALL_R {
        TASTALL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - GPTM Timer A event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
    #[inline(always)]
    pub fn taevent(&self) -> TAEVENT_R {
        TAEVENT_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - GPTM Timer A output trigger enable 0: The ADC trigger of output Timer A is disabled. 1: The ADC trigger of output Timer A is enabled."]
    #[inline(always)]
    pub fn taote(&self) -> TAOTE_R {
        TAOTE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPTM Timer A PWM output level 0: Output is unaffected. 1: Output is inverted."]
    #[inline(always)]
    pub fn tapwml(&self) -> TAPWML_R {
        TAPWML_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B enable 0: Timer B is disabled. 1: Timer B is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
    #[inline(always)]
    pub fn tben(&self) -> TBEN_R {
        TBEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B stall enable 0: Timer B continues counting while the processor is halted by the debugger. 1: Timer B freezes counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn tbstall(&self) -> TBSTALL_R {
        TBSTALL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - GPTM Timer B event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
    #[inline(always)]
    pub fn tbevent(&self) -> TBEVENT_R {
        TBEVENT_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 13 - GPTM Timer B output trigger enable 0: The ADC trigger of output Timer B is disabled. 1: The ADC trigger of output Timer B is enabled."]
    #[inline(always)]
    pub fn tbote(&self) -> TBOTE_R {
        TBOTE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPTM Timer B PWM output level 0: Output is unaffected. 1: Output is inverted."]
    #[inline(always)]
    pub fn tbpwml(&self) -> TBPWML_R {
        TBPWML_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPTM Timer A enable 0: Timer A is disabled. 1: Timer A is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
    #[inline(always)]
    #[must_use]
    pub fn taen(&mut self) -> TAEN_W<0> {
        TAEN_W::new(self)
    }
    #[doc = "Bit 1 - GPTM Timer A stall enable 0: Timer A continues counting while the processor is halted by the debugger. 1: Timer A freezes counting while the processor is halted by the debugger."]
    #[inline(always)]
    #[must_use]
    pub fn tastall(&mut self) -> TASTALL_W<1> {
        TASTALL_W::new(self)
    }
    #[doc = "Bits 2:3 - GPTM Timer A event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
    #[inline(always)]
    #[must_use]
    pub fn taevent(&mut self) -> TAEVENT_W<2> {
        TAEVENT_W::new(self)
    }
    #[doc = "Bit 5 - GPTM Timer A output trigger enable 0: The ADC trigger of output Timer A is disabled. 1: The ADC trigger of output Timer A is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn taote(&mut self) -> TAOTE_W<5> {
        TAOTE_W::new(self)
    }
    #[doc = "Bit 6 - GPTM Timer A PWM output level 0: Output is unaffected. 1: Output is inverted."]
    #[inline(always)]
    #[must_use]
    pub fn tapwml(&mut self) -> TAPWML_W<6> {
        TAPWML_W::new(self)
    }
    #[doc = "Bit 8 - GPTM Timer B enable 0: Timer B is disabled. 1: Timer B is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
    #[inline(always)]
    #[must_use]
    pub fn tben(&mut self) -> TBEN_W<8> {
        TBEN_W::new(self)
    }
    #[doc = "Bit 9 - GPTM Timer B stall enable 0: Timer B continues counting while the processor is halted by the debugger. 1: Timer B freezes counting while the processor is halted by the debugger."]
    #[inline(always)]
    #[must_use]
    pub fn tbstall(&mut self) -> TBSTALL_W<9> {
        TBSTALL_W::new(self)
    }
    #[doc = "Bits 10:11 - GPTM Timer B event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
    #[inline(always)]
    #[must_use]
    pub fn tbevent(&mut self) -> TBEVENT_W<10> {
        TBEVENT_W::new(self)
    }
    #[doc = "Bit 13 - GPTM Timer B output trigger enable 0: The ADC trigger of output Timer B is disabled. 1: The ADC trigger of output Timer B is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn tbote(&mut self) -> TBOTE_W<13> {
        TBOTE_W::new(self)
    }
    #[doc = "Bit 14 - GPTM Timer B PWM output level 0: Output is unaffected. 1: Output is inverted."]
    #[inline(always)]
    #[must_use]
    pub fn tbpwml(&mut self) -> TBPWML_W<14> {
        TBPWML_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM control This register is used alongside the CFG and TnMR registers to fine-tune the timer configuration, and to enable other features such as timer stall.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
