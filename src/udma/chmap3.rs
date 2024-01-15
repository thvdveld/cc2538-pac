#[doc = "Register `CHMAP3` reader"]
pub type R = crate::R<CHMAP3_SPEC>;
#[doc = "Register `CHMAP3` writer"]
pub type W = crate::W<CHMAP3_SPEC>;
#[doc = "Field `CH24SEL` reader - uDMA channel 24 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH24SEL_R = crate::FieldReader;
#[doc = "Field `CH24SEL` writer - uDMA channel 24 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH24SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH25SEL` reader - uDMA channel 25 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH25SEL_R = crate::FieldReader;
#[doc = "Field `CH25SEL` writer - uDMA channel 25 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH25SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH26SEL` reader - uDMA channel 26 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH26SEL_R = crate::FieldReader;
#[doc = "Field `CH26SEL` writer - uDMA channel 26 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH26SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH27SEL` reader - uDMA channel 27 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH27SEL_R = crate::FieldReader;
#[doc = "Field `CH27SEL` writer - uDMA channel 27 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH27SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH28SEL` reader - uDMA channel 28 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH28SEL_R = crate::FieldReader;
#[doc = "Field `CH28SEL` writer - uDMA channel 28 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH28SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH29SEL` reader - uDMA channel 29 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH29SEL_R = crate::FieldReader;
#[doc = "Field `CH29SEL` writer - uDMA channel 29 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH29SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH30SEL` reader - uDMA channel 30 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH30SEL_R = crate::FieldReader;
#[doc = "Field `CH30SEL` writer - uDMA channel 30 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH30SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH31SEL` reader - uDMA channel 31 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH31SEL_R = crate::FieldReader;
#[doc = "Field `CH31SEL` writer - uDMA channel 31 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH31SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - uDMA channel 24 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch24sel(&self) -> CH24SEL_R {
        CH24SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - uDMA channel 25 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch25sel(&self) -> CH25SEL_R {
        CH25SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - uDMA channel 26 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch26sel(&self) -> CH26SEL_R {
        CH26SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - uDMA channel 27 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch27sel(&self) -> CH27SEL_R {
        CH27SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - uDMA channel 28 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch28sel(&self) -> CH28SEL_R {
        CH28SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - uDMA channel 29 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch29sel(&self) -> CH29SEL_R {
        CH29SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - uDMA channel 30 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch30sel(&self) -> CH30SEL_R {
        CH30SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - uDMA channel 31 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch31sel(&self) -> CH31SEL_R {
        CH31SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - uDMA channel 24 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch24sel(&mut self) -> CH24SEL_W<CHMAP3_SPEC> {
        CH24SEL_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - uDMA channel 25 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch25sel(&mut self) -> CH25SEL_W<CHMAP3_SPEC> {
        CH25SEL_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - uDMA channel 26 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch26sel(&mut self) -> CH26SEL_W<CHMAP3_SPEC> {
        CH26SEL_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - uDMA channel 27 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch27sel(&mut self) -> CH27SEL_W<CHMAP3_SPEC> {
        CH27SEL_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - uDMA channel 28 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch28sel(&mut self) -> CH28SEL_W<CHMAP3_SPEC> {
        CH28SEL_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - uDMA channel 29 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch29sel(&mut self) -> CH29SEL_W<CHMAP3_SPEC> {
        CH29SEL_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - uDMA channel 30 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch30sel(&mut self) -> CH30SEL_W<CHMAP3_SPEC> {
        CH30SEL_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - uDMA channel 31 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch31sel(&mut self) -> CH31SEL_W<CHMAP3_SPEC> {
        CH31SEL_W::new(self, 28)
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
#[doc = "DMA channel map select 3 Each 4-bit field of the CHMAP3 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chmap3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chmap3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHMAP3_SPEC;
impl crate::RegisterSpec for CHMAP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chmap3::R`](R) reader structure"]
impl crate::Readable for CHMAP3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chmap3::W`](W) writer structure"]
impl crate::Writable for CHMAP3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHMAP3 to value 0"]
impl crate::Resettable for CHMAP3_SPEC {
    const RESET_VALUE: u32 = 0;
}
