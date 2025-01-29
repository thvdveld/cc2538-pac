#[doc = "Register `SEQ_CTRL` reader"]
pub type R = crate::R<SeqCtrlSpec>;
#[doc = "Register `SEQ_CTRL` writer"]
pub type W = crate::W<SeqCtrlSpec>;
#[doc = "Field `SW_CONTROL_STATUS` reader - These bits can be used by software to trigger sequencer operations. External logic can set these bits by writing 1b, cannot reset them by writing 0b. The sequencer can reset these bits by writing 0b, cannot set them by writing 1b. Setting the run bit in PKA_FUNCTION together with a nonzero sequencer operations field automatically sets bit \\[0\\]
here. This field should always be written with zeroes and ignored when reading this register."]
pub type SwControlStatusR = crate::FieldReader;
#[doc = "Field `SW_CONTROL_STATUS` writer - These bits can be used by software to trigger sequencer operations. External logic can set these bits by writing 1b, cannot reset them by writing 0b. The sequencer can reset these bits by writing 0b, cannot set them by writing 1b. Setting the run bit in PKA_FUNCTION together with a nonzero sequencer operations field automatically sets bit \\[0\\]
here. This field should always be written with zeroes and ignored when reading this register."]
pub type SwControlStatusW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SEQUENCER_STATUS` reader - These read-only bits can be used by the sequencer to communicate status to the outside world. Bit \\[8\\]
is also used as sequencer interrupt, with the complement of this bit ORed into the run bit in PKA_FUNCTION. This field should always be written with zeroes and ignored when reading this register."]
pub type SequencerStatusR = crate::FieldReader;
#[doc = "Field `RESET` reader - Option program ROM: Reset value = 0. Read/Write, reset value 0b (ZERO). Writing 1b resets the sequencer, write to 0b to restart operations again. As the reset value is 0b, the sequencer will automatically start operations executing from program ROM. This bit should always be written with zero and ignored when reading this register. Option Program RAM: Reset value =1. Read/Write, reset value 1b (ONE). When 1b, the sequencer is held in a reset state and the PKA_PROGRAM area is accessible for loading the sequencer program (while the PKA_DATA_RAM is inaccessible), write to 0b to (re)start sequencer operations and disable PKA_PROGRAM area accessibility (also enables the PKA_DATA_RAM accesses). Resetting the sequencer (in order to load other firmware) should only be done when the PKA Engine is not performing any operations (i.e. the run bit in the PKA_FUNCTION register should be zero)."]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - Option program ROM: Reset value = 0. Read/Write, reset value 0b (ZERO). Writing 1b resets the sequencer, write to 0b to restart operations again. As the reset value is 0b, the sequencer will automatically start operations executing from program ROM. This bit should always be written with zero and ignored when reading this register. Option Program RAM: Reset value =1. Read/Write, reset value 1b (ONE). When 1b, the sequencer is held in a reset state and the PKA_PROGRAM area is accessible for loading the sequencer program (while the PKA_DATA_RAM is inaccessible), write to 0b to (re)start sequencer operations and disable PKA_PROGRAM area accessibility (also enables the PKA_DATA_RAM accesses). Resetting the sequencer (in order to load other firmware) should only be done when the PKA Engine is not performing any operations (i.e. the run bit in the PKA_FUNCTION register should be zero)."]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - These bits can be used by software to trigger sequencer operations. External logic can set these bits by writing 1b, cannot reset them by writing 0b. The sequencer can reset these bits by writing 0b, cannot set them by writing 1b. Setting the run bit in PKA_FUNCTION together with a nonzero sequencer operations field automatically sets bit \\[0\\]
here. This field should always be written with zeroes and ignored when reading this register."]
    #[inline(always)]
    pub fn sw_control_status(&self) -> SwControlStatusR {
        SwControlStatusR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - These read-only bits can be used by the sequencer to communicate status to the outside world. Bit \\[8\\]
is also used as sequencer interrupt, with the complement of this bit ORed into the run bit in PKA_FUNCTION. This field should always be written with zeroes and ignored when reading this register."]
    #[inline(always)]
    pub fn sequencer_status(&self) -> SequencerStatusR {
        SequencerStatusR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Option program ROM: Reset value = 0. Read/Write, reset value 0b (ZERO). Writing 1b resets the sequencer, write to 0b to restart operations again. As the reset value is 0b, the sequencer will automatically start operations executing from program ROM. This bit should always be written with zero and ignored when reading this register. Option Program RAM: Reset value =1. Read/Write, reset value 1b (ONE). When 1b, the sequencer is held in a reset state and the PKA_PROGRAM area is accessible for loading the sequencer program (while the PKA_DATA_RAM is inaccessible), write to 0b to (re)start sequencer operations and disable PKA_PROGRAM area accessibility (also enables the PKA_DATA_RAM accesses). Resetting the sequencer (in order to load other firmware) should only be done when the PKA Engine is not performing any operations (i.e. the run bit in the PKA_FUNCTION register should be zero)."]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - These bits can be used by software to trigger sequencer operations. External logic can set these bits by writing 1b, cannot reset them by writing 0b. The sequencer can reset these bits by writing 0b, cannot set them by writing 1b. Setting the run bit in PKA_FUNCTION together with a nonzero sequencer operations field automatically sets bit \\[0\\]
here. This field should always be written with zeroes and ignored when reading this register."]
    #[inline(always)]
    pub fn sw_control_status(&mut self) -> SwControlStatusW<SeqCtrlSpec> {
        SwControlStatusW::new(self, 0)
    }
    #[doc = "Bit 31 - Option program ROM: Reset value = 0. Read/Write, reset value 0b (ZERO). Writing 1b resets the sequencer, write to 0b to restart operations again. As the reset value is 0b, the sequencer will automatically start operations executing from program ROM. This bit should always be written with zero and ignored when reading this register. Option Program RAM: Reset value =1. Read/Write, reset value 1b (ONE). When 1b, the sequencer is held in a reset state and the PKA_PROGRAM area is accessible for loading the sequencer program (while the PKA_DATA_RAM is inaccessible), write to 0b to (re)start sequencer operations and disable PKA_PROGRAM area accessibility (also enables the PKA_DATA_RAM accesses). Resetting the sequencer (in order to load other firmware) should only be done when the PKA Engine is not performing any operations (i.e. the run bit in the PKA_FUNCTION register should be zero)."]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<SeqCtrlSpec> {
        ResetW::new(self, 31)
    }
}
#[doc = "PKA sequencer control and status register The sequencer is interfaced with the outside world through a single control and status register. With the exception of bit \\[31\\], the actual use of bits in the separate sub-fields of this register is determined by the sequencer firmware. This register need only be accessed when the sequencer program is stored in RAM. The reset value of the RESTE bit depends upon the option chosen for sequencer program storage.\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeqCtrlSpec;
impl crate::RegisterSpec for SeqCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq_ctrl::R`](R) reader structure"]
impl crate::Readable for SeqCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`seq_ctrl::W`](W) writer structure"]
impl crate::Writable for SeqCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEQ_CTRL to value 0"]
impl crate::Resettable for SeqCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
