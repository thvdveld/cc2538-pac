#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `GPTMCFG` reader - GPTM configuration The GPTMCFG values are defined as follows: 0x0: 32-bit timer configuration. 0x1: 32-bit real-time clock 0x2: Reserved 0x3: Reserved 0x4: 16-bit timer configuration. The function is controlled by bits \\[1:0\\]
of GPTMTAMR and GPTMTBMR. 0x5-0x7: Reserved"]
pub type GptmcfgR = crate::FieldReader;
#[doc = "Field `GPTMCFG` writer - GPTM configuration The GPTMCFG values are defined as follows: 0x0: 32-bit timer configuration. 0x1: 32-bit real-time clock 0x2: Reserved 0x3: Reserved 0x4: 16-bit timer configuration. The function is controlled by bits \\[1:0\\]
of GPTMTAMR and GPTMTBMR. 0x5-0x7: Reserved"]
pub type GptmcfgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - GPTM configuration The GPTMCFG values are defined as follows: 0x0: 32-bit timer configuration. 0x1: 32-bit real-time clock 0x2: Reserved 0x3: Reserved 0x4: 16-bit timer configuration. The function is controlled by bits \\[1:0\\]
of GPTMTAMR and GPTMTBMR. 0x5-0x7: Reserved"]
    #[inline(always)]
    pub fn gptmcfg(&self) -> GptmcfgR {
        GptmcfgR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - GPTM configuration The GPTMCFG values are defined as follows: 0x0: 32-bit timer configuration. 0x1: 32-bit real-time clock 0x2: Reserved 0x3: Reserved 0x4: 16-bit timer configuration. The function is controlled by bits \\[1:0\\]
of GPTMTAMR and GPTMTBMR. 0x5-0x7: Reserved"]
    #[inline(always)]
    pub fn gptmcfg(&mut self) -> GptmcfgW<CfgSpec> {
        GptmcfgW::new(self, 0)
    }
}
#[doc = "GPTM configuration This register configures the global operation of the GPTM. The value written to this register determines whether the GPTM is in 32-bit mode (concatenated timers) or in 16-bit mode (individual, split timers).\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0;
}
