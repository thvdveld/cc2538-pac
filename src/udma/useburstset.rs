#[doc = "Register `USEBURSTSET` reader"]
pub type R = crate::R<UseburstsetSpec>;
#[doc = "Register `USEBURSTSET` writer"]
pub type W = crate::W<UseburstsetSpec>;
#[doc = "Field `SET` reader - Channel \\[n\\]
useburst set 0: uDMA channel \\[n\\]
responds to single or burst requests. 1: uDMA channel \\[n\\]
responds only to burst requests. Bit 0 corresponds to channel 0. This bit is automatically cleared as described above. A bit can also be manually cleared by setting the corresponding CLR\\[n\\]
bit in the DMAUSEBURSTCLR register."]
pub type SetR = crate::FieldReader<u32>;
#[doc = "Field `SET` writer - Channel \\[n\\]
useburst set 0: uDMA channel \\[n\\]
responds to single or burst requests. 1: uDMA channel \\[n\\]
responds only to burst requests. Bit 0 corresponds to channel 0. This bit is automatically cleared as described above. A bit can also be manually cleared by setting the corresponding CLR\\[n\\]
bit in the DMAUSEBURSTCLR register."]
pub type SetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
useburst set 0: uDMA channel \\[n\\]
responds to single or burst requests. 1: uDMA channel \\[n\\]
responds only to burst requests. Bit 0 corresponds to channel 0. This bit is automatically cleared as described above. A bit can also be manually cleared by setting the corresponding CLR\\[n\\]
bit in the DMAUSEBURSTCLR register."]
    #[inline(always)]
    pub fn set_(&self) -> SetR {
        SetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
useburst set 0: uDMA channel \\[n\\]
responds to single or burst requests. 1: uDMA channel \\[n\\]
responds only to burst requests. Bit 0 corresponds to channel 0. This bit is automatically cleared as described above. A bit can also be manually cleared by setting the corresponding CLR\\[n\\]
bit in the DMAUSEBURSTCLR register."]
    #[inline(always)]
    #[must_use]
    pub fn set_(&mut self) -> SetW<UseburstsetSpec> {
        SetW::new(self, 0)
    }
}
#[doc = "DMA channel useburst set Each bit of the USEBURSTSET register represents the corresponding uDMA channel. Setting a bit disables the channel single request input from generating requests, configuring the channel to only accept burst requests. Reading the register returns the status of USEBURST. If the amount of data to transfer is a multiple of the arbitration (burst) size, the corresponding SET\\[n\\]
bit is cleared after completing the final transfer. If there are fewer items remaining to transfer than the arbitration (burst) size, the uDMA controller automatically clears the corresponding SET\\[n\\]
bit, allowing the remaining items to transfer using single requests. To resume transfers using burst requests, the corresponding bit must be set again. A bit must not be set if the corresponding peripheral does not support the burst request model.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`useburstset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`useburstset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UseburstsetSpec;
impl crate::RegisterSpec for UseburstsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`useburstset::R`](R) reader structure"]
impl crate::Readable for UseburstsetSpec {}
#[doc = "`write(|w| ..)` method takes [`useburstset::W`](W) writer structure"]
impl crate::Writable for UseburstsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USEBURSTSET to value 0"]
impl crate::Resettable for UseburstsetSpec {
    const RESET_VALUE: u32 = 0;
}
