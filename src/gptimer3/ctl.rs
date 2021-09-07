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
#[doc = "Field `TBPWML` reader - GPTM Timer B PWM output level 0: Output is unaffected. 1: Output is inverted."]
pub struct TBPWML_R(crate::FieldReader<bool, bool>);
impl TBPWML_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBPWML_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBPWML_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBPWML` writer - GPTM Timer B PWM output level 0: Output is unaffected. 1: Output is inverted."]
pub struct TBPWML_W<'a> {
    w: &'a mut W,
}
impl<'a> TBPWML_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `TBOTE` reader - GPTM Timer B output trigger enable 0: The ADC trigger of output Timer B is disabled. 1: The ADC trigger of output Timer B is enabled."]
pub struct TBOTE_R(crate::FieldReader<bool, bool>);
impl TBOTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBOTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBOTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBOTE` writer - GPTM Timer B output trigger enable 0: The ADC trigger of output Timer B is disabled. 1: The ADC trigger of output Timer B is enabled."]
pub struct TBOTE_W<'a> {
    w: &'a mut W,
}
impl<'a> TBOTE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `TBEVENT` reader - GPTM Timer B event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
pub struct TBEVENT_R(crate::FieldReader<u8, u8>);
impl TBEVENT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TBEVENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBEVENT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBEVENT` writer - GPTM Timer B event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
pub struct TBEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> TBEVENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `TBSTALL` reader - GPTM Timer B stall enable 0: Timer B continues counting while the processor is halted by the debugger. 1: Timer B freezes counting while the processor is halted by the debugger."]
pub struct TBSTALL_R(crate::FieldReader<bool, bool>);
impl TBSTALL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBSTALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBSTALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBSTALL` writer - GPTM Timer B stall enable 0: Timer B continues counting while the processor is halted by the debugger. 1: Timer B freezes counting while the processor is halted by the debugger."]
pub struct TBSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> TBSTALL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `TBEN` reader - GPTM Timer B enable 0: Timer B is disabled. 1: Timer B is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
pub struct TBEN_R(crate::FieldReader<bool, bool>);
impl TBEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBEN` writer - GPTM Timer B enable 0: Timer B is disabled. 1: Timer B is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
pub struct TBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TBEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `TAPWML` reader - GPTM Timer A PWM output level 0: Output is unaffected. 1: Output is inverted."]
pub struct TAPWML_R(crate::FieldReader<bool, bool>);
impl TAPWML_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAPWML_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAPWML_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAPWML` writer - GPTM Timer A PWM output level 0: Output is unaffected. 1: Output is inverted."]
pub struct TAPWML_W<'a> {
    w: &'a mut W,
}
impl<'a> TAPWML_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `TAOTE` reader - GPTM Timer A output trigger enable 0: The ADC trigger of output Timer A is disabled. 1: The ADC trigger of output Timer A is enabled."]
pub struct TAOTE_R(crate::FieldReader<bool, bool>);
impl TAOTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAOTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAOTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAOTE` writer - GPTM Timer A output trigger enable 0: The ADC trigger of output Timer A is disabled. 1: The ADC trigger of output Timer A is enabled."]
pub struct TAOTE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAOTE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `TAEVENT` reader - GPTM Timer A event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
pub struct TAEVENT_R(crate::FieldReader<u8, u8>);
impl TAEVENT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAEVENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAEVENT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAEVENT` writer - GPTM Timer A event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
pub struct TAEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> TAEVENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `TASTALL` reader - GPTM Timer A stall enable 0: Timer A continues counting while the processor is halted by the debugger. 1: Timer A freezes counting while the processor is halted by the debugger."]
pub struct TASTALL_R(crate::FieldReader<bool, bool>);
impl TASTALL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TASTALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TASTALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TASTALL` writer - GPTM Timer A stall enable 0: Timer A continues counting while the processor is halted by the debugger. 1: Timer A freezes counting while the processor is halted by the debugger."]
pub struct TASTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> TASTALL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `TAEN` reader - GPTM Timer A enable 0: Timer A is disabled. 1: Timer A is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
pub struct TAEN_R(crate::FieldReader<bool, bool>);
impl TAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAEN` writer - GPTM Timer A enable 0: Timer A is disabled. 1: Timer A is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
pub struct TAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TAEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 14 - GPTM Timer B PWM output level 0: Output is unaffected. 1: Output is inverted."]
    #[inline(always)]
    pub fn tbpwml(&self) -> TBPWML_R {
        TBPWML_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - GPTM Timer B output trigger enable 0: The ADC trigger of output Timer B is disabled. 1: The ADC trigger of output Timer B is enabled."]
    #[inline(always)]
    pub fn tbote(&self) -> TBOTE_R {
        TBOTE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - GPTM Timer B event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
    #[inline(always)]
    pub fn tbevent(&self) -> TBEVENT_R {
        TBEVENT_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9 - GPTM Timer B stall enable 0: Timer B continues counting while the processor is halted by the debugger. 1: Timer B freezes counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn tbstall(&self) -> TBSTALL_R {
        TBSTALL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B enable 0: Timer B is disabled. 1: Timer B is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
    #[inline(always)]
    pub fn tben(&self) -> TBEN_R {
        TBEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPTM Timer A PWM output level 0: Output is unaffected. 1: Output is inverted."]
    #[inline(always)]
    pub fn tapwml(&self) -> TAPWML_R {
        TAPWML_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPTM Timer A output trigger enable 0: The ADC trigger of output Timer A is disabled. 1: The ADC trigger of output Timer A is enabled."]
    #[inline(always)]
    pub fn taote(&self) -> TAOTE_R {
        TAOTE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - GPTM Timer A event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
    #[inline(always)]
    pub fn taevent(&self) -> TAEVENT_R {
        TAEVENT_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - GPTM Timer A stall enable 0: Timer A continues counting while the processor is halted by the debugger. 1: Timer A freezes counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn tastall(&self) -> TASTALL_R {
        TASTALL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - GPTM Timer A enable 0: Timer A is disabled. 1: Timer A is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
    #[inline(always)]
    pub fn taen(&self) -> TAEN_R {
        TAEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - GPTM Timer B PWM output level 0: Output is unaffected. 1: Output is inverted."]
    #[inline(always)]
    pub fn tbpwml(&mut self) -> TBPWML_W {
        TBPWML_W { w: self }
    }
    #[doc = "Bit 13 - GPTM Timer B output trigger enable 0: The ADC trigger of output Timer B is disabled. 1: The ADC trigger of output Timer B is enabled."]
    #[inline(always)]
    pub fn tbote(&mut self) -> TBOTE_W {
        TBOTE_W { w: self }
    }
    #[doc = "Bits 10:11 - GPTM Timer B event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
    #[inline(always)]
    pub fn tbevent(&mut self) -> TBEVENT_W {
        TBEVENT_W { w: self }
    }
    #[doc = "Bit 9 - GPTM Timer B stall enable 0: Timer B continues counting while the processor is halted by the debugger. 1: Timer B freezes counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn tbstall(&mut self) -> TBSTALL_W {
        TBSTALL_W { w: self }
    }
    #[doc = "Bit 8 - GPTM Timer B enable 0: Timer B is disabled. 1: Timer B is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
    #[inline(always)]
    pub fn tben(&mut self) -> TBEN_W {
        TBEN_W { w: self }
    }
    #[doc = "Bit 6 - GPTM Timer A PWM output level 0: Output is unaffected. 1: Output is inverted."]
    #[inline(always)]
    pub fn tapwml(&mut self) -> TAPWML_W {
        TAPWML_W { w: self }
    }
    #[doc = "Bit 5 - GPTM Timer A output trigger enable 0: The ADC trigger of output Timer A is disabled. 1: The ADC trigger of output Timer A is enabled."]
    #[inline(always)]
    pub fn taote(&mut self) -> TAOTE_W {
        TAOTE_W { w: self }
    }
    #[doc = "Bits 2:3 - GPTM Timer A event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
    #[inline(always)]
    pub fn taevent(&mut self) -> TAEVENT_W {
        TAEVENT_W { w: self }
    }
    #[doc = "Bit 1 - GPTM Timer A stall enable 0: Timer A continues counting while the processor is halted by the debugger. 1: Timer A freezes counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn tastall(&mut self) -> TASTALL_W {
        TASTALL_W { w: self }
    }
    #[doc = "Bit 0 - GPTM Timer A enable 0: Timer A is disabled. 1: Timer A is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
    #[inline(always)]
    pub fn taen(&mut self) -> TAEN_W {
        TAEN_W { w: self }
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
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
