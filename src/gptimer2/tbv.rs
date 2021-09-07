#[doc = "Register `TBV` reader"]
pub struct R(crate::R<TBV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBV` writer"]
pub struct W(crate::W<TBV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBV_SPEC>;
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
impl From<crate::W<TBV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRE` reader - GPTM Timer B prescale register (16-bit mode)"]
pub struct PRE_R(crate::FieldReader<u8, u8>);
impl PRE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBV` reader - GPTM Timer B register"]
pub struct TBV_R(crate::FieldReader<u16, u16>);
impl TBV_R {
    pub(crate) fn new(bits: u16) -> Self {
        TBV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBV` writer - GPTM Timer B register"]
pub struct TBV_W<'a> {
    w: &'a mut W,
}
impl<'a> TBV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - GPTM Timer B prescale register (16-bit mode)"]
    #[inline(always)]
    pub fn pre(&self) -> PRE_R {
        PRE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:15 - GPTM Timer B register"]
    #[inline(always)]
    pub fn tbv(&self) -> TBV_R {
        TBV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPTM Timer B register"]
    #[inline(always)]
    pub fn tbv(&mut self) -> TBV_W {
        TBV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM Timer B value When read, this register shows the current, free-running value of Timer B in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry. When written, the value written into this register is loaded into the TBR register on the next clock cycle. When a GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAV register. Reads from this register return the current free-running value of Timer B. In a 16-bit mode, bits \\[15:0\\]
contain the value of the counter and bits \\[23:16\\]
contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in input edge count, input edge time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in \\[23:16\\]
is a true prescaler, meaning bits \\[23:16\\]
count down before decrementing the value in bits \\[15:0\\]. The prescaler its \\[31:24\\]
always read as 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbv](index.html) module"]
pub struct TBV_SPEC;
impl crate::RegisterSpec for TBV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbv::R](R) reader structure"]
impl crate::Readable for TBV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbv::W](W) writer structure"]
impl crate::Writable for TBV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TBV to value 0"]
impl crate::Resettable for TBV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
