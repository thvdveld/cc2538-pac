#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `TAEN` reader - GPTM Timer A enable 0: Timer A is disabled. 1: Timer A is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
pub type TAEN_R = crate::BitReader;
#[doc = "Field `TAEN` writer - GPTM Timer A enable 0: Timer A is disabled. 1: Timer A is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
pub type TAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASTALL` reader - GPTM Timer A stall enable 0: Timer A continues counting while the processor is halted by the debugger. 1: Timer A freezes counting while the processor is halted by the debugger."]
pub type TASTALL_R = crate::BitReader;
#[doc = "Field `TASTALL` writer - GPTM Timer A stall enable 0: Timer A continues counting while the processor is halted by the debugger. 1: Timer A freezes counting while the processor is halted by the debugger."]
pub type TASTALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAEVENT` reader - GPTM Timer A event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
pub type TAEVENT_R = crate::FieldReader;
#[doc = "Field `TAEVENT` writer - GPTM Timer A event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
pub type TAEVENT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TAOTE` reader - GPTM Timer A output trigger enable 0: The ADC trigger of output Timer A is disabled. 1: The ADC trigger of output Timer A is enabled."]
pub type TAOTE_R = crate::BitReader;
#[doc = "Field `TAOTE` writer - GPTM Timer A output trigger enable 0: The ADC trigger of output Timer A is disabled. 1: The ADC trigger of output Timer A is enabled."]
pub type TAOTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAPWML` reader - GPTM Timer A PWM output level 0: Output is unaffected. 1: Output is inverted."]
pub type TAPWML_R = crate::BitReader;
#[doc = "Field `TAPWML` writer - GPTM Timer A PWM output level 0: Output is unaffected. 1: Output is inverted."]
pub type TAPWML_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBEN` reader - GPTM Timer B enable 0: Timer B is disabled. 1: Timer B is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
pub type TBEN_R = crate::BitReader;
#[doc = "Field `TBEN` writer - GPTM Timer B enable 0: Timer B is disabled. 1: Timer B is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
pub type TBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBSTALL` reader - GPTM Timer B stall enable 0: Timer B continues counting while the processor is halted by the debugger. 1: Timer B freezes counting while the processor is halted by the debugger."]
pub type TBSTALL_R = crate::BitReader;
#[doc = "Field `TBSTALL` writer - GPTM Timer B stall enable 0: Timer B continues counting while the processor is halted by the debugger. 1: Timer B freezes counting while the processor is halted by the debugger."]
pub type TBSTALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBEVENT` reader - GPTM Timer B event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
pub type TBEVENT_R = crate::FieldReader;
#[doc = "Field `TBEVENT` writer - GPTM Timer B event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
pub type TBEVENT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TBOTE` reader - GPTM Timer B output trigger enable 0: The ADC trigger of output Timer B is disabled. 1: The ADC trigger of output Timer B is enabled."]
pub type TBOTE_R = crate::BitReader;
#[doc = "Field `TBOTE` writer - GPTM Timer B output trigger enable 0: The ADC trigger of output Timer B is disabled. 1: The ADC trigger of output Timer B is enabled."]
pub type TBOTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBPWML` reader - GPTM Timer B PWM output level 0: Output is unaffected. 1: Output is inverted."]
pub type TBPWML_R = crate::BitReader;
#[doc = "Field `TBPWML` writer - GPTM Timer B PWM output level 0: Output is unaffected. 1: Output is inverted."]
pub type TBPWML_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn taen(&mut self) -> TAEN_W<CTL_SPEC> {
        TAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPTM Timer A stall enable 0: Timer A continues counting while the processor is halted by the debugger. 1: Timer A freezes counting while the processor is halted by the debugger."]
    #[inline(always)]
    #[must_use]
    pub fn tastall(&mut self) -> TASTALL_W<CTL_SPEC> {
        TASTALL_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - GPTM Timer A event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
    #[inline(always)]
    #[must_use]
    pub fn taevent(&mut self) -> TAEVENT_W<CTL_SPEC> {
        TAEVENT_W::new(self, 2)
    }
    #[doc = "Bit 5 - GPTM Timer A output trigger enable 0: The ADC trigger of output Timer A is disabled. 1: The ADC trigger of output Timer A is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn taote(&mut self) -> TAOTE_W<CTL_SPEC> {
        TAOTE_W::new(self, 5)
    }
    #[doc = "Bit 6 - GPTM Timer A PWM output level 0: Output is unaffected. 1: Output is inverted."]
    #[inline(always)]
    #[must_use]
    pub fn tapwml(&mut self) -> TAPWML_W<CTL_SPEC> {
        TAPWML_W::new(self, 6)
    }
    #[doc = "Bit 8 - GPTM Timer B enable 0: Timer B is disabled. 1: Timer B is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
    #[inline(always)]
    #[must_use]
    pub fn tben(&mut self) -> TBEN_W<CTL_SPEC> {
        TBEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - GPTM Timer B stall enable 0: Timer B continues counting while the processor is halted by the debugger. 1: Timer B freezes counting while the processor is halted by the debugger."]
    #[inline(always)]
    #[must_use]
    pub fn tbstall(&mut self) -> TBSTALL_W<CTL_SPEC> {
        TBSTALL_W::new(self, 9)
    }
    #[doc = "Bits 10:11 - GPTM Timer B event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
    #[inline(always)]
    #[must_use]
    pub fn tbevent(&mut self) -> TBEVENT_W<CTL_SPEC> {
        TBEVENT_W::new(self, 10)
    }
    #[doc = "Bit 13 - GPTM Timer B output trigger enable 0: The ADC trigger of output Timer B is disabled. 1: The ADC trigger of output Timer B is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn tbote(&mut self) -> TBOTE_W<CTL_SPEC> {
        TBOTE_W::new(self, 13)
    }
    #[doc = "Bit 14 - GPTM Timer B PWM output level 0: Output is unaffected. 1: Output is inverted."]
    #[inline(always)]
    #[must_use]
    pub fn tbpwml(&mut self) -> TBPWML_W<CTL_SPEC> {
        TBPWML_W::new(self, 14)
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
#[doc = "GPTM control This register is used alongside the CFG and TnMR registers to fine-tune the timer configuration, and to enable other features such as timer stall.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
