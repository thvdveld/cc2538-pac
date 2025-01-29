#[doc = "Register `OPTIONS` reader"]
pub type R = crate::R<OptionsSpec>;
#[doc = "Field `PKCP_CONFIGURATION` reader - Value 1 indicates a PKCP with a 16x16 multiplier, value 2 indicates a PKCP with a 32x32 multiplier, other values reserved. Note: Reset value is undefined."]
pub type PkcpConfigurationR = crate::FieldReader;
#[doc = "Field `LNME_CONFIGURATION` reader - Value 0 indicates NO LNME, value 1 indicates one standard LNME (with alpha = 32, beta = 8); other values reserved. Note: Reset value is undefined"]
pub type LnmeConfigurationR = crate::FieldReader;
#[doc = "Field `SEQUENCER_CONFIGURATION` reader - Value 1 indicates a standard sequencer; other values are reserved."]
pub type SequencerConfigurationR = crate::FieldReader;
#[doc = "Field `PROGRAM_RAM` reader - Value 1b indicates sequencer program storage in RAM, value 0b in ROM. Note: Reset value is undefined"]
pub type ProgramRamR = crate::BitReader;
#[doc = "Field `PROTECTION_OPTION` reader - Value 0 indicates no additional protection against side channel attacks, value 1 indicates the SCAP option, value 3 indicates the PROT option; other values are reserved. Note: Reset value is undefined"]
pub type ProtectionOptionR = crate::FieldReader;
#[doc = "Field `INT_MASKING` reader - Value 0b indicates that the main interrupt output (bit \\[1\\]
of the interrupts output bus) is the direct complement of the run bit in the PKA_CONTROL register, value 1b indicates that interrupt masking logic is present for this output. Note: Reset value is undefined"]
pub type IntMaskingR = crate::BitReader;
#[doc = "Field `MMM3A` reader - Reserved for a future functional extension to the LNME Always 0b"]
pub type Mmm3aR = crate::BitReader;
#[doc = "Field `FIRST_LNME_NR_OF_PES` reader - Number of processing elements in the pipeline of the first LNME Should be ignored if LNME configuration is 0. Note: Reset value is undefined."]
pub type FirstLnmeNrOfPesR = crate::FieldReader;
#[doc = "Field `FIRST_LNME_FIFO_DEPTH` reader - Number of words in the first LNME's FIFO RAM Should be ignored if LNME configuration is 0. The contents of this field indicate the actual depth as selected by the LNME FIFO RAM size strap input, fifo_size_sel. Note: Reset value is undefined"]
pub type FirstLnmeFifoDepthR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Value 1 indicates a PKCP with a 16x16 multiplier, value 2 indicates a PKCP with a 32x32 multiplier, other values reserved. Note: Reset value is undefined."]
    #[inline(always)]
    pub fn pkcp_configuration(&self) -> PkcpConfigurationR {
        PkcpConfigurationR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Value 0 indicates NO LNME, value 1 indicates one standard LNME (with alpha = 32, beta = 8); other values reserved. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn lnme_configuration(&self) -> LnmeConfigurationR {
        LnmeConfigurationR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:6 - Value 1 indicates a standard sequencer; other values are reserved."]
    #[inline(always)]
    pub fn sequencer_configuration(&self) -> SequencerConfigurationR {
        SequencerConfigurationR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Value 1b indicates sequencer program storage in RAM, value 0b in ROM. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn program_ram(&self) -> ProgramRamR {
        ProgramRamR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Value 0 indicates no additional protection against side channel attacks, value 1 indicates the SCAP option, value 3 indicates the PROT option; other values are reserved. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn protection_option(&self) -> ProtectionOptionR {
        ProtectionOptionR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Value 0b indicates that the main interrupt output (bit \\[1\\]
of the interrupts output bus) is the direct complement of the run bit in the PKA_CONTROL register, value 1b indicates that interrupt masking logic is present for this output. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn int_masking(&self) -> IntMaskingR {
        IntMaskingR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved for a future functional extension to the LNME Always 0b"]
    #[inline(always)]
    pub fn mmm3a(&self) -> Mmm3aR {
        Mmm3aR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Number of processing elements in the pipeline of the first LNME Should be ignored if LNME configuration is 0. Note: Reset value is undefined."]
    #[inline(always)]
    pub fn first_lnme_nr_of_pes(&self) -> FirstLnmeNrOfPesR {
        FirstLnmeNrOfPesR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:31 - Number of words in the first LNME's FIFO RAM Should be ignored if LNME configuration is 0. The contents of this field indicate the actual depth as selected by the LNME FIFO RAM size strap input, fifo_size_sel. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn first_lnme_fifo_depth(&self) -> FirstLnmeFifoDepthR {
        FirstLnmeFifoDepthR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "PKA hardware options register This register provides the host with a means to determine the hardware configuration implemented in this PKA engine, focused on options that have an effect on software interacting with the module. Note: (32 x (1st LNME nr. of PEs + 1st LNME FIFO RAM depth - 10)) equals the maximum modulus vector length (in bits) that can be handled by the modular exponentiation and ECC operations executed on a PKA engine that includes an LNME.\n\nYou can [`read`](crate::Reg::read) this register and get [`options::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OptionsSpec;
impl crate::RegisterSpec for OptionsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`options::R`](R) reader structure"]
impl crate::Readable for OptionsSpec {}
#[doc = "`reset()` method sets OPTIONS to value 0"]
impl crate::Resettable for OptionsSpec {
    const RESET_VALUE: u32 = 0;
}
