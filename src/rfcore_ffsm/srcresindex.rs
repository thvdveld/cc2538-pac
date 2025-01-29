#[doc = "Register `SRCRESINDEX` reader"]
pub type R = crate::R<SrcresindexSpec>;
#[doc = "Register `SRCRESINDEX` writer"]
pub type W = crate::W<SrcresindexSpec>;
#[doc = "Field `SRCRESINDEX` reader - The bit index of the least-significant entry (0-23 for short addresses or 0-11 for extended addresses) in SRCRESMASK, or 0x3F when there is no source match On a match, bit 5 is 0 when the match is on a short address and 1 when it is on an extended address. On a match, bit 6 is 1 when the conditions for automatic pending bit in acknowledgment have been met (see the description of SRCMATCH.AUTOPEND). The bit does not indicate if the acknowledgment is actually transmitted, and does not consider the PENDING_OR register bit and the SACK/SACKPEND/SNACK strobes."]
pub type SrcresindexR = crate::FieldReader;
#[doc = "Field `SRCRESINDEX` writer - The bit index of the least-significant entry (0-23 for short addresses or 0-11 for extended addresses) in SRCRESMASK, or 0x3F when there is no source match On a match, bit 5 is 0 when the match is on a short address and 1 when it is on an extended address. On a match, bit 6 is 1 when the conditions for automatic pending bit in acknowledgment have been met (see the description of SRCMATCH.AUTOPEND). The bit does not indicate if the acknowledgment is actually transmitted, and does not consider the PENDING_OR register bit and the SACK/SACKPEND/SNACK strobes."]
pub type SrcresindexW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The bit index of the least-significant entry (0-23 for short addresses or 0-11 for extended addresses) in SRCRESMASK, or 0x3F when there is no source match On a match, bit 5 is 0 when the match is on a short address and 1 when it is on an extended address. On a match, bit 6 is 1 when the conditions for automatic pending bit in acknowledgment have been met (see the description of SRCMATCH.AUTOPEND). The bit does not indicate if the acknowledgment is actually transmitted, and does not consider the PENDING_OR register bit and the SACK/SACKPEND/SNACK strobes."]
    #[inline(always)]
    pub fn srcresindex(&self) -> SrcresindexR {
        SrcresindexR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The bit index of the least-significant entry (0-23 for short addresses or 0-11 for extended addresses) in SRCRESMASK, or 0x3F when there is no source match On a match, bit 5 is 0 when the match is on a short address and 1 when it is on an extended address. On a match, bit 6 is 1 when the conditions for automatic pending bit in acknowledgment have been met (see the description of SRCMATCH.AUTOPEND). The bit does not indicate if the acknowledgment is actually transmitted, and does not consider the PENDING_OR register bit and the SACK/SACKPEND/SNACK strobes."]
    #[inline(always)]
    pub fn srcresindex(&mut self) -> SrcresindexW<SrcresindexSpec> {
        SrcresindexW::new(self, 0)
    }
}
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::Reg::read) this register and get [`srcresindex::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcresindex::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcresindexSpec;
impl crate::RegisterSpec for SrcresindexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcresindex::R`](R) reader structure"]
impl crate::Readable for SrcresindexSpec {}
#[doc = "`write(|w| ..)` method takes [`srcresindex::W`](W) writer structure"]
impl crate::Writable for SrcresindexSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCRESINDEX to value 0"]
impl crate::Resettable for SrcresindexSpec {
    const RESET_VALUE: u32 = 0;
}
