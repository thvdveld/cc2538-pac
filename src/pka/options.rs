#[doc = "Register `OPTIONS` reader"]
pub struct R(crate::R<OPTIONS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTIONS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTIONS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTIONS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIRST_LNME_FIFO_DEPTH` reader - Number of words in the first LNME's FIFO RAM Should be ignored if LNME configuration is 0. The contents of this field indicate the actual depth as selected by the LNME FIFO RAM size strap input, fifo_size_sel. Note: Reset value is undefined"]
pub type FIRST_LNME_FIFO_DEPTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIRST_LNME_NR_OF_PES` reader - Number of processing elements in the pipeline of the first LNME Should be ignored if LNME configuration is 0. Note: Reset value is undefined."]
pub type FIRST_LNME_NR_OF_PES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MMM3A` reader - Reserved for a future functional extension to the LNME Always 0b"]
pub type MMM3A_R = crate::BitReader<bool>;
#[doc = "Field `INT_MASKING` reader - Value 0b indicates that the main interrupt output (bit \\[1\\]
of the interrupts output bus) is the direct complement of the run bit in the PKA_CONTROL register, value 1b indicates that interrupt masking logic is present for this output. Note: Reset value is undefined"]
pub type INT_MASKING_R = crate::BitReader<bool>;
#[doc = "Field `PROTECTION_OPTION` reader - Value 0 indicates no additional protection against side channel attacks, value 1 indicates the SCAP option, value 3 indicates the PROT option; other values are reserved. Note: Reset value is undefined"]
pub type PROTECTION_OPTION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PROGRAM_RAM` reader - Value 1b indicates sequencer program storage in RAM, value 0b in ROM. Note: Reset value is undefined"]
pub type PROGRAM_RAM_R = crate::BitReader<bool>;
#[doc = "Field `SEQUENCER_CONFIGURATION` reader - Value 1 indicates a standard sequencer; other values are reserved."]
pub type SEQUENCER_CONFIGURATION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LNME_CONFIGURATION` reader - Value 0 indicates NO LNME, value 1 indicates one standard LNME (with alpha = 32, beta = 8); other values reserved. Note: Reset value is undefined"]
pub type LNME_CONFIGURATION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PKCP_CONFIGURATION` reader - Value 1 indicates a PKCP with a 16x16 multiplier, value 2 indicates a PKCP with a 32x32 multiplier, other values reserved. Note: Reset value is undefined."]
pub type PKCP_CONFIGURATION_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 24:31 - Number of words in the first LNME's FIFO RAM Should be ignored if LNME configuration is 0. The contents of this field indicate the actual depth as selected by the LNME FIFO RAM size strap input, fifo_size_sel. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn first_lnme_fifo_depth(&self) -> FIRST_LNME_FIFO_DEPTH_R {
        FIRST_LNME_FIFO_DEPTH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:21 - Number of processing elements in the pipeline of the first LNME Should be ignored if LNME configuration is 0. Note: Reset value is undefined."]
    #[inline(always)]
    pub fn first_lnme_nr_of_pes(&self) -> FIRST_LNME_NR_OF_PES_R {
        FIRST_LNME_NR_OF_PES_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 12 - Reserved for a future functional extension to the LNME Always 0b"]
    #[inline(always)]
    pub fn mmm3a(&self) -> MMM3A_R {
        MMM3A_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Value 0b indicates that the main interrupt output (bit \\[1\\]
of the interrupts output bus) is the direct complement of the run bit in the PKA_CONTROL register, value 1b indicates that interrupt masking logic is present for this output. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn int_masking(&self) -> INT_MASKING_R {
        INT_MASKING_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Value 0 indicates no additional protection against side channel attacks, value 1 indicates the SCAP option, value 3 indicates the PROT option; other values are reserved. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn protection_option(&self) -> PROTECTION_OPTION_R {
        PROTECTION_OPTION_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 7 - Value 1b indicates sequencer program storage in RAM, value 0b in ROM. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn program_ram(&self) -> PROGRAM_RAM_R {
        PROGRAM_RAM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Value 1 indicates a standard sequencer; other values are reserved."]
    #[inline(always)]
    pub fn sequencer_configuration(&self) -> SEQUENCER_CONFIGURATION_R {
        SEQUENCER_CONFIGURATION_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 2:4 - Value 0 indicates NO LNME, value 1 indicates one standard LNME (with alpha = 32, beta = 8); other values reserved. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn lnme_configuration(&self) -> LNME_CONFIGURATION_R {
        LNME_CONFIGURATION_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 0:1 - Value 1 indicates a PKCP with a 16x16 multiplier, value 2 indicates a PKCP with a 32x32 multiplier, other values reserved. Note: Reset value is undefined."]
    #[inline(always)]
    pub fn pkcp_configuration(&self) -> PKCP_CONFIGURATION_R {
        PKCP_CONFIGURATION_R::new((self.bits & 3) as u8)
    }
}
#[doc = "PKA hardware options register This register provides the host with a means to determine the hardware configuration implemented in this PKA engine, focused on options that have an effect on software interacting with the module. Note: (32 x (1st LNME nr. of PEs + 1st LNME FIFO RAM depth - 10)) equals the maximum modulus vector length (in bits) that can be handled by the modular exponentiation and ECC operations executed on a PKA engine that includes an LNME.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [options](index.html) module"]
pub struct OPTIONS_SPEC;
impl crate::RegisterSpec for OPTIONS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [options::R](R) reader structure"]
impl crate::Readable for OPTIONS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OPTIONS to value 0"]
impl crate::Resettable for OPTIONS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
