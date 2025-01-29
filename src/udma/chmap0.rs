#[doc = "Register `CHMAP0` reader"]
pub type R = crate::R<Chmap0Spec>;
#[doc = "Register `CHMAP0` writer"]
pub type W = crate::W<Chmap0Spec>;
#[doc = "Field `CH0SEL` reader - uDMA channel 0 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch0selR = crate::FieldReader;
#[doc = "Field `CH0SEL` writer - uDMA channel 0 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch0selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH1SEL` reader - uDMA channel 1 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch1selR = crate::FieldReader;
#[doc = "Field `CH1SEL` writer - uDMA channel 1 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch1selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH2SEL` reader - uDMA channel 2 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch2selR = crate::FieldReader;
#[doc = "Field `CH2SEL` writer - uDMA channel 2 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch2selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH3SEL` reader - uDMA channel 3 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch3selR = crate::FieldReader;
#[doc = "Field `CH3SEL` writer - uDMA channel 3 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch3selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH4SEL` reader - uDMA channel 4 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch4selR = crate::FieldReader;
#[doc = "Field `CH4SEL` writer - uDMA channel 4 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch4selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH5SEL` reader - uDMA channel 5 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch5selR = crate::FieldReader;
#[doc = "Field `CH5SEL` writer - uDMA channel 5 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch5selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH6SEL` reader - uDMA channel 6 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch6selR = crate::FieldReader;
#[doc = "Field `CH6SEL` writer - uDMA channel 6 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch6selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH7SEL` reader - uDMA channel 7 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch7selR = crate::FieldReader;
#[doc = "Field `CH7SEL` writer - uDMA channel 7 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type Ch7selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - uDMA channel 0 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch0sel(&self) -> Ch0selR {
        Ch0selR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - uDMA channel 1 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch1sel(&self) -> Ch1selR {
        Ch1selR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - uDMA channel 2 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch2sel(&self) -> Ch2selR {
        Ch2selR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - uDMA channel 3 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch3sel(&self) -> Ch3selR {
        Ch3selR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - uDMA channel 4 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch4sel(&self) -> Ch4selR {
        Ch4selR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - uDMA channel 5 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch5sel(&self) -> Ch5selR {
        Ch5selR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - uDMA channel 6 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch6sel(&self) -> Ch6selR {
        Ch6selR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - uDMA channel 7 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch7sel(&self) -> Ch7selR {
        Ch7selR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - uDMA channel 0 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch0sel(&mut self) -> Ch0selW<Chmap0Spec> {
        Ch0selW::new(self, 0)
    }
    #[doc = "Bits 4:7 - uDMA channel 1 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch1sel(&mut self) -> Ch1selW<Chmap0Spec> {
        Ch1selW::new(self, 4)
    }
    #[doc = "Bits 8:11 - uDMA channel 2 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch2sel(&mut self) -> Ch2selW<Chmap0Spec> {
        Ch2selW::new(self, 8)
    }
    #[doc = "Bits 12:15 - uDMA channel 3 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch3sel(&mut self) -> Ch3selW<Chmap0Spec> {
        Ch3selW::new(self, 12)
    }
    #[doc = "Bits 16:19 - uDMA channel 4 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch4sel(&mut self) -> Ch4selW<Chmap0Spec> {
        Ch4selW::new(self, 16)
    }
    #[doc = "Bits 20:23 - uDMA channel 5 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch5sel(&mut self) -> Ch5selW<Chmap0Spec> {
        Ch5selW::new(self, 20)
    }
    #[doc = "Bits 24:27 - uDMA channel 6 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch6sel(&mut self) -> Ch6selW<Chmap0Spec> {
        Ch6selW::new(self, 24)
    }
    #[doc = "Bits 28:31 - uDMA channel 7 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch7sel(&mut self) -> Ch7selW<Chmap0Spec> {
        Ch7selW::new(self, 28)
    }
}
#[doc = "DMA channel map select 0 Each 4-bit field of the CHMAP0 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section.\n\nYou can [`read`](crate::Reg::read) this register and get [`chmap0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chmap0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chmap0Spec;
impl crate::RegisterSpec for Chmap0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chmap0::R`](R) reader structure"]
impl crate::Readable for Chmap0Spec {}
#[doc = "`write(|w| ..)` method takes [`chmap0::W`](W) writer structure"]
impl crate::Writable for Chmap0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHMAP0 to value 0"]
impl crate::Resettable for Chmap0Spec {
    const RESET_VALUE: u32 = 0;
}
