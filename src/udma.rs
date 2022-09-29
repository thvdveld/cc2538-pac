#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA status The STAT register returns the status of the uDMA controller. This register cannot be read when the uDMA controller is in the reset state."]
    pub stat: STAT,
    #[doc = "0x04 - DMA configuration The CFG register controls the configuration of the uDMA controller."]
    pub cfg: CFG,
    #[doc = "0x08 - DMA channel control base pointer The CTLBASE register must be configured so that the base pointer points to a location in system memory. The amount of system memory that must be assigned to the uDMA controller depends on the number of uDMA channels used and whether the alternate channel control data structure is used. See Section 10.2.5 for details about the Channel Control Table. The base address must be aligned on a 1024-byte boundary. This register cannot be read when the uDMA controller is in the reset state."]
    pub ctlbase: CTLBASE,
    #[doc = "0x0c - DMA alternate channel control base pointer The ALTBASE register returns the base address of the alternate channel control data. This register removes the necessity for application software to calculate the base address of the alternate channel control structures. This register cannot be read when the uDMA controller is in the reset state."]
    pub altbase: ALTBASE,
    #[doc = "0x10 - DMA channel wait-on-request status This read-only register indicates that the uDMA channel is waiting on a request. A peripheral can hold off the uDMA from performing a single request until the peripheral is ready for a burst request to enhance the uDMA performance. The use of this feature is dependent on the design of the peripheral and is not controllable by software in any way. This register cannot be read when the uDMA controller is in the reset state."]
    pub waitstat: WAITSTAT,
    #[doc = "0x14 - DMA channel software request Each bit of the SWREQ register represents the corresponding uDMA channel. Setting a bit generates a request for the specified uDMA channel."]
    pub swreq: SWREQ,
    #[doc = "0x18 - DMA channel useburst set Each bit of the USEBURSTSET register represents the corresponding uDMA channel. Setting a bit disables the channel single request input from generating requests, configuring the channel to only accept burst requests. Reading the register returns the status of USEBURST. If the amount of data to transfer is a multiple of the arbitration (burst) size, the corresponding SET\\[n\\]
bit is cleared after completing the final transfer. If there are fewer items remaining to transfer than the arbitration (burst) size, the uDMA controller automatically clears the corresponding SET\\[n\\]
bit, allowing the remaining items to transfer using single requests. To resume transfers using burst requests, the corresponding bit must be set again. A bit must not be set if the corresponding peripheral does not support the burst request model."]
    pub useburstset: USEBURSTSET,
    #[doc = "0x1c - DMA channel useburst clear Each bit of the USEBURSTCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the USEBURSTSET register."]
    pub useburstclr: USEBURSTCLR,
    #[doc = "0x20 - DMA channel request mask set Each bit of the REQMASKSET register represents the corresponding uDMA channel. Setting a bit disables uDMA requests for the channel. Reading the register returns the request mask status. When a uDMA channel request is masked, that means the peripheral can no longer request uDMA transfers. The channel can then be used for software-initiated transfers."]
    pub reqmaskset: REQMASKSET,
    #[doc = "0x24 - DMA channel request mask clear Each bit of the REQMASKCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the REQMASKSET register."]
    pub reqmaskclr: REQMASKCLR,
    #[doc = "0x28 - DMA channel enable set Each bit of the ENASET register represents the corresponding uDMA channel. Setting a bit enables the corresponding uDMA channel. Reading the register returns the enable status of the channels. If a channel is enabled but the request mask is set (REQMASKSET), then the channel can be used for software-initiated transfers."]
    pub enaset: ENASET,
    #[doc = "0x2c - DMA channel enable clear Each bit of the ENACLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the ENASET register."]
    pub enaclr: ENACLR,
    #[doc = "0x30 - DMA channel primary alternate set Each bit of the ALTSET register represents the corresponding uDMA channel. Setting a bit configures the uDMA channel to use the alternate control data structure. Reading the register returns the status of which control data structure is in use for the corresponding uDMA channel."]
    pub altset: ALTSET,
    #[doc = "0x34 - DMA channel primary alternate clear Each bit of the ALTCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the ALTSET register."]
    pub altclr: ALTCLR,
    #[doc = "0x38 - DMA channel priority set Each bit of the PRIOSET register represents the corresponding uDMA channel. Setting a bit configures the uDMA channel to have a high priority level. Reading the register returns the status of the channel priority mask."]
    pub prioset: PRIOSET,
    #[doc = "0x3c - DMA channel priority clear Each bit of the DMAPRIOCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the PRIOSET register."]
    pub prioclr: PRIOCLR,
    _reserved16: [u8; 0x0c],
    #[doc = "0x4c - DMA bus error clear The ERRCLR register is used to read and clear the uDMA bus error status. The error status is set if the uDMA controller encountered a bus error while performing a transfer. If a bus error occurs on a channel, that channel is automatically disabled by the uDMA controller. The other channels are unaffected."]
    pub errclr: ERRCLR,
    _reserved17: [u8; 0x04b0],
    #[doc = "0x500 - DMA channel assignment Each bit of the CHASGN register represents the corresponding uDMA channel. Setting a bit selects the secondary channel assignment as specified in the section \"Channel Assignments\""]
    pub chasgn: CHASGN,
    #[doc = "0x504 - DMA channel interrupt status Each bit of the CHIS register represents the corresponding uDMA channel. A bit is set when that uDMA channel causes a completion interrupt. The bits are cleared by writing 1."]
    pub chis: CHIS,
    _reserved19: [u8; 0x08],
    #[doc = "0x510 - DMA channel map select 0 Each 4-bit field of the CHMAP0 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section."]
    pub chmap0: CHMAP0,
    #[doc = "0x514 - DMA channel map select 1 Each 4-bit field of the CHMAP1 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section."]
    pub chmap1: CHMAP1,
    #[doc = "0x518 - DMA channel map select 2 Each 4-bit field of the CHMAP2 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section."]
    pub chmap2: CHMAP2,
    #[doc = "0x51c - DMA channel map select 3 Each 4-bit field of the CHMAP3 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section."]
    pub chmap3: CHMAP3,
}
#[doc = "STAT (r) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "DMA status The STAT register returns the status of the uDMA controller. This register cannot be read when the uDMA controller is in the reset state."]
pub mod stat;
#[doc = "CFG (w) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "DMA configuration The CFG register controls the configuration of the uDMA controller."]
pub mod cfg;
#[doc = "CTLBASE (rw) register accessor: an alias for `Reg<CTLBASE_SPEC>`"]
pub type CTLBASE = crate::Reg<ctlbase::CTLBASE_SPEC>;
#[doc = "DMA channel control base pointer The CTLBASE register must be configured so that the base pointer points to a location in system memory. The amount of system memory that must be assigned to the uDMA controller depends on the number of uDMA channels used and whether the alternate channel control data structure is used. See Section 10.2.5 for details about the Channel Control Table. The base address must be aligned on a 1024-byte boundary. This register cannot be read when the uDMA controller is in the reset state."]
pub mod ctlbase;
#[doc = "ALTBASE (r) register accessor: an alias for `Reg<ALTBASE_SPEC>`"]
pub type ALTBASE = crate::Reg<altbase::ALTBASE_SPEC>;
#[doc = "DMA alternate channel control base pointer The ALTBASE register returns the base address of the alternate channel control data. This register removes the necessity for application software to calculate the base address of the alternate channel control structures. This register cannot be read when the uDMA controller is in the reset state."]
pub mod altbase;
#[doc = "WAITSTAT (r) register accessor: an alias for `Reg<WAITSTAT_SPEC>`"]
pub type WAITSTAT = crate::Reg<waitstat::WAITSTAT_SPEC>;
#[doc = "DMA channel wait-on-request status This read-only register indicates that the uDMA channel is waiting on a request. A peripheral can hold off the uDMA from performing a single request until the peripheral is ready for a burst request to enhance the uDMA performance. The use of this feature is dependent on the design of the peripheral and is not controllable by software in any way. This register cannot be read when the uDMA controller is in the reset state."]
pub mod waitstat;
#[doc = "SWREQ (w) register accessor: an alias for `Reg<SWREQ_SPEC>`"]
pub type SWREQ = crate::Reg<swreq::SWREQ_SPEC>;
#[doc = "DMA channel software request Each bit of the SWREQ register represents the corresponding uDMA channel. Setting a bit generates a request for the specified uDMA channel."]
pub mod swreq;
#[doc = "USEBURSTSET (rw) register accessor: an alias for `Reg<USEBURSTSET_SPEC>`"]
pub type USEBURSTSET = crate::Reg<useburstset::USEBURSTSET_SPEC>;
#[doc = "DMA channel useburst set Each bit of the USEBURSTSET register represents the corresponding uDMA channel. Setting a bit disables the channel single request input from generating requests, configuring the channel to only accept burst requests. Reading the register returns the status of USEBURST. If the amount of data to transfer is a multiple of the arbitration (burst) size, the corresponding SET\\[n\\]
bit is cleared after completing the final transfer. If there are fewer items remaining to transfer than the arbitration (burst) size, the uDMA controller automatically clears the corresponding SET\\[n\\]
bit, allowing the remaining items to transfer using single requests. To resume transfers using burst requests, the corresponding bit must be set again. A bit must not be set if the corresponding peripheral does not support the burst request model."]
pub mod useburstset;
#[doc = "USEBURSTCLR (w) register accessor: an alias for `Reg<USEBURSTCLR_SPEC>`"]
pub type USEBURSTCLR = crate::Reg<useburstclr::USEBURSTCLR_SPEC>;
#[doc = "DMA channel useburst clear Each bit of the USEBURSTCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the USEBURSTSET register."]
pub mod useburstclr;
#[doc = "REQMASKSET (rw) register accessor: an alias for `Reg<REQMASKSET_SPEC>`"]
pub type REQMASKSET = crate::Reg<reqmaskset::REQMASKSET_SPEC>;
#[doc = "DMA channel request mask set Each bit of the REQMASKSET register represents the corresponding uDMA channel. Setting a bit disables uDMA requests for the channel. Reading the register returns the request mask status. When a uDMA channel request is masked, that means the peripheral can no longer request uDMA transfers. The channel can then be used for software-initiated transfers."]
pub mod reqmaskset;
#[doc = "REQMASKCLR (w) register accessor: an alias for `Reg<REQMASKCLR_SPEC>`"]
pub type REQMASKCLR = crate::Reg<reqmaskclr::REQMASKCLR_SPEC>;
#[doc = "DMA channel request mask clear Each bit of the REQMASKCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the REQMASKSET register."]
pub mod reqmaskclr;
#[doc = "ENASET (rw) register accessor: an alias for `Reg<ENASET_SPEC>`"]
pub type ENASET = crate::Reg<enaset::ENASET_SPEC>;
#[doc = "DMA channel enable set Each bit of the ENASET register represents the corresponding uDMA channel. Setting a bit enables the corresponding uDMA channel. Reading the register returns the enable status of the channels. If a channel is enabled but the request mask is set (REQMASKSET), then the channel can be used for software-initiated transfers."]
pub mod enaset;
#[doc = "ENACLR (w) register accessor: an alias for `Reg<ENACLR_SPEC>`"]
pub type ENACLR = crate::Reg<enaclr::ENACLR_SPEC>;
#[doc = "DMA channel enable clear Each bit of the ENACLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the ENASET register."]
pub mod enaclr;
#[doc = "ALTSET (rw) register accessor: an alias for `Reg<ALTSET_SPEC>`"]
pub type ALTSET = crate::Reg<altset::ALTSET_SPEC>;
#[doc = "DMA channel primary alternate set Each bit of the ALTSET register represents the corresponding uDMA channel. Setting a bit configures the uDMA channel to use the alternate control data structure. Reading the register returns the status of which control data structure is in use for the corresponding uDMA channel."]
pub mod altset;
#[doc = "ALTCLR (w) register accessor: an alias for `Reg<ALTCLR_SPEC>`"]
pub type ALTCLR = crate::Reg<altclr::ALTCLR_SPEC>;
#[doc = "DMA channel primary alternate clear Each bit of the ALTCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the ALTSET register."]
pub mod altclr;
#[doc = "PRIOSET (rw) register accessor: an alias for `Reg<PRIOSET_SPEC>`"]
pub type PRIOSET = crate::Reg<prioset::PRIOSET_SPEC>;
#[doc = "DMA channel priority set Each bit of the PRIOSET register represents the corresponding uDMA channel. Setting a bit configures the uDMA channel to have a high priority level. Reading the register returns the status of the channel priority mask."]
pub mod prioset;
#[doc = "PRIOCLR (w) register accessor: an alias for `Reg<PRIOCLR_SPEC>`"]
pub type PRIOCLR = crate::Reg<prioclr::PRIOCLR_SPEC>;
#[doc = "DMA channel priority clear Each bit of the DMAPRIOCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the PRIOSET register."]
pub mod prioclr;
#[doc = "ERRCLR (rw) register accessor: an alias for `Reg<ERRCLR_SPEC>`"]
pub type ERRCLR = crate::Reg<errclr::ERRCLR_SPEC>;
#[doc = "DMA bus error clear The ERRCLR register is used to read and clear the uDMA bus error status. The error status is set if the uDMA controller encountered a bus error while performing a transfer. If a bus error occurs on a channel, that channel is automatically disabled by the uDMA controller. The other channels are unaffected."]
pub mod errclr;
#[doc = "CHASGN (rw) register accessor: an alias for `Reg<CHASGN_SPEC>`"]
pub type CHASGN = crate::Reg<chasgn::CHASGN_SPEC>;
#[doc = "DMA channel assignment Each bit of the CHASGN register represents the corresponding uDMA channel. Setting a bit selects the secondary channel assignment as specified in the section \"Channel Assignments\""]
pub mod chasgn;
#[doc = "CHIS (rw) register accessor: an alias for `Reg<CHIS_SPEC>`"]
pub type CHIS = crate::Reg<chis::CHIS_SPEC>;
#[doc = "DMA channel interrupt status Each bit of the CHIS register represents the corresponding uDMA channel. A bit is set when that uDMA channel causes a completion interrupt. The bits are cleared by writing 1."]
pub mod chis;
#[doc = "CHMAP0 (rw) register accessor: an alias for `Reg<CHMAP0_SPEC>`"]
pub type CHMAP0 = crate::Reg<chmap0::CHMAP0_SPEC>;
#[doc = "DMA channel map select 0 Each 4-bit field of the CHMAP0 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section."]
pub mod chmap0;
#[doc = "CHMAP1 (rw) register accessor: an alias for `Reg<CHMAP1_SPEC>`"]
pub type CHMAP1 = crate::Reg<chmap1::CHMAP1_SPEC>;
#[doc = "DMA channel map select 1 Each 4-bit field of the CHMAP1 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section."]
pub mod chmap1;
#[doc = "CHMAP2 (rw) register accessor: an alias for `Reg<CHMAP2_SPEC>`"]
pub type CHMAP2 = crate::Reg<chmap2::CHMAP2_SPEC>;
#[doc = "DMA channel map select 2 Each 4-bit field of the CHMAP2 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section."]
pub mod chmap2;
#[doc = "CHMAP3 (rw) register accessor: an alias for `Reg<CHMAP3_SPEC>`"]
pub type CHMAP3 = crate::Reg<chmap3::CHMAP3_SPEC>;
#[doc = "DMA channel map select 3 Each 4-bit field of the CHMAP3 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section."]
pub mod chmap3;
