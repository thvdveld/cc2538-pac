#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `TAEN` reader - GPTM Timer A enable 0: Timer A is disabled. 1: Timer A is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
pub type TaenR = crate::BitReader;
#[doc = "Field `TAEN` writer - GPTM Timer A enable 0: Timer A is disabled. 1: Timer A is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
pub type TaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASTALL` reader - GPTM Timer A stall enable 0: Timer A continues counting while the processor is halted by the debugger. 1: Timer A freezes counting while the processor is halted by the debugger."]
pub type TastallR = crate::BitReader;
#[doc = "Field `TASTALL` writer - GPTM Timer A stall enable 0: Timer A continues counting while the processor is halted by the debugger. 1: Timer A freezes counting while the processor is halted by the debugger."]
pub type TastallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAEVENT` reader - GPTM Timer A event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
pub type TaeventR = crate::FieldReader;
#[doc = "Field `TAEVENT` writer - GPTM Timer A event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
pub type TaeventW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TAOTE` reader - GPTM Timer A output trigger enable 0: The ADC trigger of output Timer A is disabled. 1: The ADC trigger of output Timer A is enabled."]
pub type TaoteR = crate::BitReader;
#[doc = "Field `TAOTE` writer - GPTM Timer A output trigger enable 0: The ADC trigger of output Timer A is disabled. 1: The ADC trigger of output Timer A is enabled."]
pub type TaoteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAPWML` reader - GPTM Timer A PWM output level 0: Output is unaffected. 1: Output is inverted."]
pub type TapwmlR = crate::BitReader;
#[doc = "Field `TAPWML` writer - GPTM Timer A PWM output level 0: Output is unaffected. 1: Output is inverted."]
pub type TapwmlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBEN` reader - GPTM Timer B enable 0: Timer B is disabled. 1: Timer B is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
pub type TbenR = crate::BitReader;
#[doc = "Field `TBEN` writer - GPTM Timer B enable 0: Timer B is disabled. 1: Timer B is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
pub type TbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBSTALL` reader - GPTM Timer B stall enable 0: Timer B continues counting while the processor is halted by the debugger. 1: Timer B freezes counting while the processor is halted by the debugger."]
pub type TbstallR = crate::BitReader;
#[doc = "Field `TBSTALL` writer - GPTM Timer B stall enable 0: Timer B continues counting while the processor is halted by the debugger. 1: Timer B freezes counting while the processor is halted by the debugger."]
pub type TbstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBEVENT` reader - GPTM Timer B event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
pub type TbeventR = crate::FieldReader;
#[doc = "Field `TBEVENT` writer - GPTM Timer B event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
pub type TbeventW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TBOTE` reader - GPTM Timer B output trigger enable 0: The ADC trigger of output Timer B is disabled. 1: The ADC trigger of output Timer B is enabled."]
pub type TboteR = crate::BitReader;
#[doc = "Field `TBOTE` writer - GPTM Timer B output trigger enable 0: The ADC trigger of output Timer B is disabled. 1: The ADC trigger of output Timer B is enabled."]
pub type TboteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBPWML` reader - GPTM Timer B PWM output level 0: Output is unaffected. 1: Output is inverted."]
pub type TbpwmlR = crate::BitReader;
#[doc = "Field `TBPWML` writer - GPTM Timer B PWM output level 0: Output is unaffected. 1: Output is inverted."]
pub type TbpwmlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPTM Timer A enable 0: Timer A is disabled. 1: Timer A is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
    #[inline(always)]
    pub fn taen(&self) -> TaenR {
        TaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPTM Timer A stall enable 0: Timer A continues counting while the processor is halted by the debugger. 1: Timer A freezes counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn tastall(&self) -> TastallR {
        TastallR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - GPTM Timer A event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
    #[inline(always)]
    pub fn taevent(&self) -> TaeventR {
        TaeventR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - GPTM Timer A output trigger enable 0: The ADC trigger of output Timer A is disabled. 1: The ADC trigger of output Timer A is enabled."]
    #[inline(always)]
    pub fn taote(&self) -> TaoteR {
        TaoteR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPTM Timer A PWM output level 0: Output is unaffected. 1: Output is inverted."]
    #[inline(always)]
    pub fn tapwml(&self) -> TapwmlR {
        TapwmlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B enable 0: Timer B is disabled. 1: Timer B is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
    #[inline(always)]
    pub fn tben(&self) -> TbenR {
        TbenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B stall enable 0: Timer B continues counting while the processor is halted by the debugger. 1: Timer B freezes counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn tbstall(&self) -> TbstallR {
        TbstallR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - GPTM Timer B event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
    #[inline(always)]
    pub fn tbevent(&self) -> TbeventR {
        TbeventR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 13 - GPTM Timer B output trigger enable 0: The ADC trigger of output Timer B is disabled. 1: The ADC trigger of output Timer B is enabled."]
    #[inline(always)]
    pub fn tbote(&self) -> TboteR {
        TboteR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPTM Timer B PWM output level 0: Output is unaffected. 1: Output is inverted."]
    #[inline(always)]
    pub fn tbpwml(&self) -> TbpwmlR {
        TbpwmlR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPTM Timer A enable 0: Timer A is disabled. 1: Timer A is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
    #[inline(always)]
    pub fn taen(&mut self) -> TaenW<CtlSpec> {
        TaenW::new(self, 0)
    }
    #[doc = "Bit 1 - GPTM Timer A stall enable 0: Timer A continues counting while the processor is halted by the debugger. 1: Timer A freezes counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn tastall(&mut self) -> TastallW<CtlSpec> {
        TastallW::new(self, 1)
    }
    #[doc = "Bits 2:3 - GPTM Timer A event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
    #[inline(always)]
    pub fn taevent(&mut self) -> TaeventW<CtlSpec> {
        TaeventW::new(self, 2)
    }
    #[doc = "Bit 5 - GPTM Timer A output trigger enable 0: The ADC trigger of output Timer A is disabled. 1: The ADC trigger of output Timer A is enabled."]
    #[inline(always)]
    pub fn taote(&mut self) -> TaoteW<CtlSpec> {
        TaoteW::new(self, 5)
    }
    #[doc = "Bit 6 - GPTM Timer A PWM output level 0: Output is unaffected. 1: Output is inverted."]
    #[inline(always)]
    pub fn tapwml(&mut self) -> TapwmlW<CtlSpec> {
        TapwmlW::new(self, 6)
    }
    #[doc = "Bit 8 - GPTM Timer B enable 0: Timer B is disabled. 1: Timer B is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
    #[inline(always)]
    pub fn tben(&mut self) -> TbenW<CtlSpec> {
        TbenW::new(self, 8)
    }
    #[doc = "Bit 9 - GPTM Timer B stall enable 0: Timer B continues counting while the processor is halted by the debugger. 1: Timer B freezes counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn tbstall(&mut self) -> TbstallW<CtlSpec> {
        TbstallW::new(self, 9)
    }
    #[doc = "Bits 10:11 - GPTM Timer B event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
    #[inline(always)]
    pub fn tbevent(&mut self) -> TbeventW<CtlSpec> {
        TbeventW::new(self, 10)
    }
    #[doc = "Bit 13 - GPTM Timer B output trigger enable 0: The ADC trigger of output Timer B is disabled. 1: The ADC trigger of output Timer B is enabled."]
    #[inline(always)]
    pub fn tbote(&mut self) -> TboteW<CtlSpec> {
        TboteW::new(self, 13)
    }
    #[doc = "Bit 14 - GPTM Timer B PWM output level 0: Output is unaffected. 1: Output is inverted."]
    #[inline(always)]
    pub fn tbpwml(&mut self) -> TbpwmlW<CtlSpec> {
        TbpwmlW::new(self, 14)
    }
}
#[doc = "GPTM control This register is used alongside the CFG and TnMR registers to fine-tune the timer configuration, and to enable other features such as timer stall.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
