#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPTMCFG` reader - GPTM configuration The GPTMCFG values are defined as follows: 0x0: 32-bit timer configuration. 0x1: 32-bit real-time clock 0x2: Reserved 0x3: Reserved 0x4: 16-bit timer configuration. The function is controlled by bits \\[1:0\\]
of GPTMTAMR and GPTMTBMR. 0x5-0x7: Reserved"]
pub type GPTMCFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPTMCFG` writer - GPTM configuration The GPTMCFG values are defined as follows: 0x0: 32-bit timer configuration. 0x1: 32-bit real-time clock 0x2: Reserved 0x3: Reserved 0x4: 16-bit timer configuration. The function is controlled by bits \\[1:0\\]
of GPTMTAMR and GPTMTBMR. 0x5-0x7: Reserved"]
pub type GPTMCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - GPTM configuration The GPTMCFG values are defined as follows: 0x0: 32-bit timer configuration. 0x1: 32-bit real-time clock 0x2: Reserved 0x3: Reserved 0x4: 16-bit timer configuration. The function is controlled by bits \\[1:0\\]
of GPTMTAMR and GPTMTBMR. 0x5-0x7: Reserved"]
    #[inline(always)]
    pub fn gptmcfg(&self) -> GPTMCFG_R {
        GPTMCFG_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - GPTM configuration The GPTMCFG values are defined as follows: 0x0: 32-bit timer configuration. 0x1: 32-bit real-time clock 0x2: Reserved 0x3: Reserved 0x4: 16-bit timer configuration. The function is controlled by bits \\[1:0\\]
of GPTMTAMR and GPTMTBMR. 0x5-0x7: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn gptmcfg(&mut self) -> GPTMCFG_W<0> {
        GPTMCFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM configuration This register configures the global operation of the GPTM. The value written to this register determines whether the GPTM is in 32-bit mode (concatenated timers) or in 16-bit mode (individual, split timers).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
