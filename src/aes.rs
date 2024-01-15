#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    dmac_ch0_ctrl: DMAC_CH0_CTRL,
    dmac_ch0_extaddr: DMAC_CH0_EXTADDR,
    _reserved2: [u8; 0x04],
    dmac_ch0_dmalength: DMAC_CH0_DMALENGTH,
    _reserved3: [u8; 0x08],
    dmac_status: DMAC_STATUS,
    dmac_swres: DMAC_SWRES,
    dmac_ch1_ctrl: DMAC_CH1_CTRL,
    dmac_ch1_extaddr: DMAC_CH1_EXTADDR,
    _reserved7: [u8; 0x04],
    dmac_ch1_dmalength: DMAC_CH1_DMALENGTH,
    _reserved8: [u8; 0x48],
    dmac_mst_runparams: DMAC_MST_RUNPARAMS,
    dmac_persr: DMAC_PERSR,
    _reserved10: [u8; 0x78],
    dmac_options: DMAC_OPTIONS,
    dmac_version: DMAC_VERSION,
    _reserved12: [u8; 0x0300],
    key_store_write_area: KEY_STORE_WRITE_AREA,
    key_store_written_area: KEY_STORE_WRITTEN_AREA,
    key_store_size: KEY_STORE_SIZE,
    key_store_read_area: KEY_STORE_READ_AREA,
    _reserved16: [u8; 0xf0],
    aes_key2_0: AES_KEY2_0,
    aes_key2_1: AES_KEY2_1,
    aes_key2_2: AES_KEY2_2,
    aes_key2_3: AES_KEY2_3,
    aes_key3_0: AES_KEY3_0,
    aes_key3_1: AES_KEY3_1,
    aes_key3_2: AES_KEY3_2,
    aes_key3_3: AES_KEY3_3,
    _reserved24: [u8; 0x20],
    aes_iv_0: AES_IV_0,
    aes_iv_1: AES_IV_1,
    aes_iv_2: AES_IV_2,
    aes_iv_3: AES_IV_3,
    aes_ctrl: AES_CTRL,
    aes_c_length_0: AES_C_LENGTH_0,
    aes_c_length_1: AES_C_LENGTH_1,
    aes_auth_length: AES_AUTH_LENGTH,
    aes_data_in_out_0: AES_DATA_IN_OUT_0,
    aes_data_in_out_1: AES_DATA_IN_OUT_1,
    aes_data_in_out_2: AES_DATA_IN_OUT_2,
    aes_data_in_out_3: AES_DATA_IN_OUT_3,
    aes_tag_out_0: AES_TAG_OUT_0,
    aes_tag_out_1: AES_TAG_OUT_1,
    aes_tag_out_2: AES_TAG_OUT_2,
    aes_tag_out_3: AES_TAG_OUT_3,
    _reserved40: [u8; 0x80],
    hash_data_in_0: HASH_DATA_IN_0,
    hash_data_in_1: HASH_DATA_IN_1,
    hash_data_in_2: HASH_DATA_IN_2,
    hash_data_in_3: HASH_DATA_IN_3,
    hash_data_in_4: HASH_DATA_IN_4,
    hash_data_in_5: HASH_DATA_IN_5,
    hash_data_in_6: HASH_DATA_IN_6,
    hash_data_in_7: HASH_DATA_IN_7,
    hash_data_in_8: HASH_DATA_IN_8,
    hash_data_in_9: HASH_DATA_IN_9,
    hash_data_in_10: HASH_DATA_IN_10,
    hash_data_in_11: HASH_DATA_IN_11,
    hash_data_in_12: HASH_DATA_IN_12,
    hash_data_in_13: HASH_DATA_IN_13,
    hash_data_in_14: HASH_DATA_IN_14,
    hash_data_in_15: HASH_DATA_IN_15,
    hash_io_buf_ctrl: HASH_IO_BUF_CTRL,
    hash_mode_in: HASH_MODE_IN,
    hash_length_in_l: HASH_LENGTH_IN_L,
    hash_length_in_h: HASH_LENGTH_IN_H,
    hash_digest_a: HASH_DIGEST_A,
    hash_digest_b: HASH_DIGEST_B,
    hash_digest_c: HASH_DIGEST_C,
    hash_digest_d: HASH_DIGEST_D,
    hash_digest_e: HASH_DIGEST_E,
    hash_digest_f: HASH_DIGEST_F,
    hash_digest_g: HASH_DIGEST_G,
    hash_digest_h: HASH_DIGEST_H,
    _reserved68: [u8; 0x90],
    ctrl_alg_sel: CTRL_ALG_SEL,
    ctrl_prot_en: CTRL_PROT_EN,
    _reserved70: [u8; 0x38],
    ctrl_sw_reset: CTRL_SW_RESET,
    _reserved71: [u8; 0x3c],
    ctrl_int_cfg: CTRL_INT_CFG,
    ctrl_int_en: CTRL_INT_EN,
    ctrl_int_clr: CTRL_INT_CLR,
    ctrl_int_set: CTRL_INT_SET,
    ctrl_int_stat: CTRL_INT_STAT,
    _reserved76: [u8; 0x64],
    ctrl_options: CTRL_OPTIONS,
    ctrl_version: CTRL_VERSION,
}
impl RegisterBlock {
    #[doc = "0x00 - Channel control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
    #[inline(always)]
    pub const fn dmac_ch0_ctrl(&self) -> &DMAC_CH0_CTRL {
        &self.dmac_ch0_ctrl
    }
    #[doc = "0x04 - Channel external address"]
    #[inline(always)]
    pub const fn dmac_ch0_extaddr(&self) -> &DMAC_CH0_EXTADDR {
        &self.dmac_ch0_extaddr
    }
    #[doc = "0x0c - Channel DMA length"]
    #[inline(always)]
    pub const fn dmac_ch0_dmalength(&self) -> &DMAC_CH0_DMALENGTH {
        &self.dmac_ch0_dmalength
    }
    #[doc = "0x18 - DMAC status This register provides the actual state of each DMA channel. It also reports port errors in case these were received by the master interface module during the data transfer."]
    #[inline(always)]
    pub const fn dmac_status(&self) -> &DMAC_STATUS {
        &self.dmac_status
    }
    #[doc = "0x1c - DMAC software reset register Software reset is used to reset the DMAC to stop all transfers and clears the port error status register. After the software reset is performed, all the channels are disabled and no new requests are performed by the channels. The DMAC waits for the existing (active) requests to finish and accordingly sets the DMAC status registers."]
    #[inline(always)]
    pub const fn dmac_swres(&self) -> &DMAC_SWRES {
        &self.dmac_swres
    }
    #[doc = "0x20 - Channel control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
    #[inline(always)]
    pub const fn dmac_ch1_ctrl(&self) -> &DMAC_CH1_CTRL {
        &self.dmac_ch1_ctrl
    }
    #[doc = "0x24 - Channel external address"]
    #[inline(always)]
    pub const fn dmac_ch1_extaddr(&self) -> &DMAC_CH1_EXTADDR {
        &self.dmac_ch1_extaddr
    }
    #[doc = "0x2c - Channel DMA length"]
    #[inline(always)]
    pub const fn dmac_ch1_dmalength(&self) -> &DMAC_CH1_DMALENGTH {
        &self.dmac_ch1_dmalength
    }
    #[doc = "0x78 - DMAC master run-time parameters This register defines all the run-time parameters for the AHB master interface port. These parameters are required for the proper functioning of the EIP-101m AHB master adapter."]
    #[inline(always)]
    pub const fn dmac_mst_runparams(&self) -> &DMAC_MST_RUNPARAMS {
        &self.dmac_mst_runparams
    }
    #[doc = "0x7c - DMAC port error raw status register This register provides the actual status of individual port errors. It also indicates which channel is serviced by an external AHB port (which is frozen by a port error). A port error aborts operations on all serviced channels (channel enable bit is forced to 0) and prevents further transfers via that port until the error is cleared by writing to the DMAC_SWRES register."]
    #[inline(always)]
    pub const fn dmac_persr(&self) -> &DMAC_PERSR {
        &self.dmac_persr
    }
    #[doc = "0xf8 - DMAC options register These registers contain information regarding the different options configured in this DMAC."]
    #[inline(always)]
    pub const fn dmac_options(&self) -> &DMAC_OPTIONS {
        &self.dmac_options
    }
    #[doc = "0xfc - DMAC version register This register contains an indication (or signature) of the EIP type of this DMAC, as well as the hardware version/patch numbers."]
    #[inline(always)]
    pub const fn dmac_version(&self) -> &DMAC_VERSION {
        &self.dmac_version
    }
    #[doc = "0x400 - Key store write area register This register defines where the keys should be written in the key store RAM. After writing this register, the key store module is ready to receive the keys through a DMA operation. In case the key data transfer triggered an error in the key store, the error will be available in the interrupt status register after the DMA is finished. The key store write-error is asserted when the programmed/selected area is not completely written. This error is also asserted when the DMA operation writes to ram areas that are not selected. The key store RAM is divided into 8 areas of 128 bits. 192-bit keys written in the key store RAM should start on boundaries of 256 bits. This means that writing a 192-bit key to the key store RAM must be done by writing 256 bits of data with the 64 most-significant bits set to 0. These bits are ignored by the AES engine."]
    #[inline(always)]
    pub const fn key_store_write_area(&self) -> &KEY_STORE_WRITE_AREA {
        &self.key_store_write_area
    }
    #[doc = "0x404 - Key store written area register This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and results in an error."]
    #[inline(always)]
    pub const fn key_store_written_area(&self) -> &KEY_STORE_WRITTEN_AREA {
        &self.key_store_written_area
    }
    #[doc = "0x408 - Key store size register This register defines the size of the keys that are written with DMA. This register should be configured before writing to the KEY_STORE_WRITE_AREA register."]
    #[inline(always)]
    pub const fn key_store_size(&self) -> &KEY_STORE_SIZE {
        &self.key_store_size
    }
    #[doc = "0x40c - Key store read area register This register selects the key store RAM area from where the key needs to be read that will be used for an AES operation. The operation directly starts after writing this register. When the operation is finished, the status of the key store read operation is available in the interrupt status register. Key store read error is asserted when a RAM area is selected which does not contain valid written key."]
    #[inline(always)]
    pub const fn key_store_read_area(&self) -> &KEY_STORE_READ_AREA {
        &self.key_store_read_area
    }
    #[doc = "0x500 - AES_KEY2_0 / AES_GHASH_H_IN_0 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    #[inline(always)]
    pub const fn aes_key2_0(&self) -> &AES_KEY2_0 {
        &self.aes_key2_0
    }
    #[doc = "0x504 - AES_KEY2_1 / AES_GHASH_H_IN_1 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    #[inline(always)]
    pub const fn aes_key2_1(&self) -> &AES_KEY2_1 {
        &self.aes_key2_1
    }
    #[doc = "0x508 - AES_KEY2_2 / AES_GHASH_H_IN_2 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    #[inline(always)]
    pub const fn aes_key2_2(&self) -> &AES_KEY2_2 {
        &self.aes_key2_2
    }
    #[doc = "0x50c - AES_KEY2_3 / AES_GHASH_H_IN_3 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    #[inline(always)]
    pub const fn aes_key2_3(&self) -> &AES_KEY2_3 {
        &self.aes_key2_3
    }
    #[doc = "0x510 - AES_KEY3_0 / AES_KEY2_4 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    #[inline(always)]
    pub const fn aes_key3_0(&self) -> &AES_KEY3_0 {
        &self.aes_key3_0
    }
    #[doc = "0x514 - AES_KEY3_1 / AES_KEY2_5 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    #[inline(always)]
    pub const fn aes_key3_1(&self) -> &AES_KEY3_1 {
        &self.aes_key3_1
    }
    #[doc = "0x518 - AES_KEY3_2 / AES_KEY2_6 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    #[inline(always)]
    pub const fn aes_key3_2(&self) -> &AES_KEY3_2 {
        &self.aes_key3_2
    }
    #[doc = "0x51c - AES_KEY3_3 / AES_KEY2_7 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    #[inline(always)]
    pub const fn aes_key3_3(&self) -> &AES_KEY3_3 {
        &self.aes_key3_3
    }
    #[doc = "0x540 - AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
    #[inline(always)]
    pub const fn aes_iv_0(&self) -> &AES_IV_0 {
        &self.aes_iv_0
    }
    #[doc = "0x544 - AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
    #[inline(always)]
    pub const fn aes_iv_1(&self) -> &AES_IV_1 {
        &self.aes_iv_1
    }
    #[doc = "0x548 - AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
    #[inline(always)]
    pub const fn aes_iv_2(&self) -> &AES_IV_2 {
        &self.aes_iv_2
    }
    #[doc = "0x54c - AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
    #[inline(always)]
    pub const fn aes_iv_3(&self) -> &AES_IV_3 {
        &self.aes_iv_3
    }
    #[doc = "0x550 - AES input/output buffer control and mode register This register specifies the AES mode of operation for the EIP-120t. Electronic codebook (ECB) mode is automatically selected if bits \\[28:5\\]
of this register are all 0."]
    #[inline(always)]
    pub const fn aes_ctrl(&self) -> &AES_CTRL {
        &self.aes_ctrl
    }
    #[doc = "0x554 - AES crypto length registers (LSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM, and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
    #[inline(always)]
    pub const fn aes_c_length_0(&self) -> &AES_C_LENGTH_0 {
        &self.aes_c_length_0
    }
    #[doc = "0x558 - AES crypto length registers (MSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
    #[inline(always)]
    pub const fn aes_c_length_1(&self) -> &AES_C_LENGTH_1 {
        &self.aes_c_length_1
    }
    #[doc = "0x55c - Authentication length register"]
    #[inline(always)]
    pub const fn aes_auth_length(&self) -> &AES_AUTH_LENGTH {
        &self.aes_auth_length
    }
    #[doc = "0x560 - Data input/output registers The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    #[inline(always)]
    pub const fn aes_data_in_out_0(&self) -> &AES_DATA_IN_OUT_0 {
        &self.aes_data_in_out_0
    }
    #[doc = "0x564 - Data Input/Output Registers The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    #[inline(always)]
    pub const fn aes_data_in_out_1(&self) -> &AES_DATA_IN_OUT_1 {
        &self.aes_data_in_out_1
    }
    #[doc = "0x568 - Data Input/Output Registers The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    #[inline(always)]
    pub const fn aes_data_in_out_2(&self) -> &AES_DATA_IN_OUT_2 {
        &self.aes_data_in_out_2
    }
    #[doc = "0x56c - Data Input/Output Registers The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    #[inline(always)]
    pub const fn aes_data_in_out_3(&self) -> &AES_DATA_IN_OUT_3 {
        &self.aes_data_in_out_3
    }
    #[doc = "0x570 - TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order will return the IV twice."]
    #[inline(always)]
    pub const fn aes_tag_out_0(&self) -> &AES_TAG_OUT_0 {
        &self.aes_tag_out_0
    }
    #[doc = "0x574 - TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order returns the IV twice."]
    #[inline(always)]
    pub const fn aes_tag_out_1(&self) -> &AES_TAG_OUT_1 {
        &self.aes_tag_out_1
    }
    #[doc = "0x578 - TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order returns the IV twice."]
    #[inline(always)]
    pub const fn aes_tag_out_2(&self) -> &AES_TAG_OUT_2 {
        &self.aes_tag_out_2
    }
    #[doc = "0x57c - TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order returns the IV twice."]
    #[inline(always)]
    pub const fn aes_tag_out_3(&self) -> &AES_TAG_OUT_3 {
        &self.aes_tag_out_3
    }
    #[doc = "0x600 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hash_data_in_0(&self) -> &HASH_DATA_IN_0 {
        &self.hash_data_in_0
    }
    #[doc = "0x604 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hash_data_in_1(&self) -> &HASH_DATA_IN_1 {
        &self.hash_data_in_1
    }
    #[doc = "0x608 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hash_data_in_2(&self) -> &HASH_DATA_IN_2 {
        &self.hash_data_in_2
    }
    #[doc = "0x60c - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hash_data_in_3(&self) -> &HASH_DATA_IN_3 {
        &self.hash_data_in_3
    }
    #[doc = "0x610 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hash_data_in_4(&self) -> &HASH_DATA_IN_4 {
        &self.hash_data_in_4
    }
    #[doc = "0x614 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hash_data_in_5(&self) -> &HASH_DATA_IN_5 {
        &self.hash_data_in_5
    }
    #[doc = "0x618 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hash_data_in_6(&self) -> &HASH_DATA_IN_6 {
        &self.hash_data_in_6
    }
    #[doc = "0x61c - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hash_data_in_7(&self) -> &HASH_DATA_IN_7 {
        &self.hash_data_in_7
    }
    #[doc = "0x620 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hash_data_in_8(&self) -> &HASH_DATA_IN_8 {
        &self.hash_data_in_8
    }
    #[doc = "0x624 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hash_data_in_9(&self) -> &HASH_DATA_IN_9 {
        &self.hash_data_in_9
    }
    #[doc = "0x628 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hash_data_in_10(&self) -> &HASH_DATA_IN_10 {
        &self.hash_data_in_10
    }
    #[doc = "0x62c - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hash_data_in_11(&self) -> &HASH_DATA_IN_11 {
        &self.hash_data_in_11
    }
    #[doc = "0x630 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hash_data_in_12(&self) -> &HASH_DATA_IN_12 {
        &self.hash_data_in_12
    }
    #[doc = "0x634 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hash_data_in_13(&self) -> &HASH_DATA_IN_13 {
        &self.hash_data_in_13
    }
    #[doc = "0x638 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hash_data_in_14(&self) -> &HASH_DATA_IN_14 {
        &self.hash_data_in_14
    }
    #[doc = "0x63c - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hash_data_in_15(&self) -> &HASH_DATA_IN_15 {
        &self.hash_data_in_15
    }
    #[doc = "0x640 - Input/output buffer control and status register This register pair shares a single address location and contains bits that control and monitor the data flow between the host and the hash engine."]
    #[inline(always)]
    pub const fn hash_io_buf_ctrl(&self) -> &HASH_IO_BUF_CTRL {
        &self.hash_io_buf_ctrl
    }
    #[doc = "0x644 - Hash mode register"]
    #[inline(always)]
    pub const fn hash_mode_in(&self) -> &HASH_MODE_IN {
        &self.hash_mode_in
    }
    #[doc = "0x648 - Hash length register"]
    #[inline(always)]
    pub const fn hash_length_in_l(&self) -> &HASH_LENGTH_IN_L {
        &self.hash_length_in_l
    }
    #[doc = "0x64c - Hash length register"]
    #[inline(always)]
    pub const fn hash_length_in_h(&self) -> &HASH_LENGTH_IN_H {
        &self.hash_length_in_h
    }
    #[doc = "0x650 - Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    #[inline(always)]
    pub const fn hash_digest_a(&self) -> &HASH_DIGEST_A {
        &self.hash_digest_a
    }
    #[doc = "0x654 - Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    #[inline(always)]
    pub const fn hash_digest_b(&self) -> &HASH_DIGEST_B {
        &self.hash_digest_b
    }
    #[doc = "0x658 - Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    #[inline(always)]
    pub const fn hash_digest_c(&self) -> &HASH_DIGEST_C {
        &self.hash_digest_c
    }
    #[doc = "0x65c - Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    #[inline(always)]
    pub const fn hash_digest_d(&self) -> &HASH_DIGEST_D {
        &self.hash_digest_d
    }
    #[doc = "0x660 - Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    #[inline(always)]
    pub const fn hash_digest_e(&self) -> &HASH_DIGEST_E {
        &self.hash_digest_e
    }
    #[doc = "0x664 - Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    #[inline(always)]
    pub const fn hash_digest_f(&self) -> &HASH_DIGEST_F {
        &self.hash_digest_f
    }
    #[doc = "0x668 - Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    #[inline(always)]
    pub const fn hash_digest_g(&self) -> &HASH_DIGEST_G {
        &self.hash_digest_g
    }
    #[doc = "0x66c - Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    #[inline(always)]
    pub const fn hash_digest_h(&self) -> &HASH_DIGEST_H {
        &self.hash_digest_h
    }
    #[doc = "0x700 - Algorithm select This algorithm selection register configures the internal destination of the DMA controller."]
    #[inline(always)]
    pub const fn ctrl_alg_sel(&self) -> &CTRL_ALG_SEL {
        &self.ctrl_alg_sel
    }
    #[doc = "0x704 - Master PROT privileged access enable This register enables the second bit (bit \\[1\\]) of the AHB HPROT bus of the AHB master interface when a read action of key(s) is performed on the AHB master interface for writing keys into the store module."]
    #[inline(always)]
    pub const fn ctrl_prot_en(&self) -> &CTRL_PROT_EN {
        &self.ctrl_prot_en
    }
    #[doc = "0x740 - Software reset"]
    #[inline(always)]
    pub const fn ctrl_sw_reset(&self) -> &CTRL_SW_RESET {
        &self.ctrl_sw_reset
    }
    #[doc = "0x780 - Interrupt configuration"]
    #[inline(always)]
    pub const fn ctrl_int_cfg(&self) -> &CTRL_INT_CFG {
        &self.ctrl_int_cfg
    }
    #[doc = "0x784 - Interrupt enable"]
    #[inline(always)]
    pub const fn ctrl_int_en(&self) -> &CTRL_INT_EN {
        &self.ctrl_int_en
    }
    #[doc = "0x788 - Interrupt clear"]
    #[inline(always)]
    pub const fn ctrl_int_clr(&self) -> &CTRL_INT_CLR {
        &self.ctrl_int_clr
    }
    #[doc = "0x78c - Interrupt set"]
    #[inline(always)]
    pub const fn ctrl_int_set(&self) -> &CTRL_INT_SET {
        &self.ctrl_int_set
    }
    #[doc = "0x790 - Interrupt status"]
    #[inline(always)]
    pub const fn ctrl_int_stat(&self) -> &CTRL_INT_STAT {
        &self.ctrl_int_stat
    }
    #[doc = "0x7f8 - Options register"]
    #[inline(always)]
    pub const fn ctrl_options(&self) -> &CTRL_OPTIONS {
        &self.ctrl_options
    }
    #[doc = "0x7fc - Version register"]
    #[inline(always)]
    pub const fn ctrl_version(&self) -> &CTRL_VERSION {
        &self.ctrl_version
    }
}
#[doc = "DMAC_CH0_CTRL (rw) register accessor: Channel control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_ch0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_ch0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_ch0_ctrl`]
module"]
pub type DMAC_CH0_CTRL = crate::Reg<dmac_ch0_ctrl::DMAC_CH0_CTRL_SPEC>;
#[doc = "Channel control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
pub mod dmac_ch0_ctrl;
#[doc = "DMAC_CH0_EXTADDR (rw) register accessor: Channel external address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_ch0_extaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_ch0_extaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_ch0_extaddr`]
module"]
pub type DMAC_CH0_EXTADDR = crate::Reg<dmac_ch0_extaddr::DMAC_CH0_EXTADDR_SPEC>;
#[doc = "Channel external address"]
pub mod dmac_ch0_extaddr;
#[doc = "DMAC_CH0_DMALENGTH (rw) register accessor: Channel DMA length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_ch0_dmalength::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_ch0_dmalength::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_ch0_dmalength`]
module"]
pub type DMAC_CH0_DMALENGTH = crate::Reg<dmac_ch0_dmalength::DMAC_CH0_DMALENGTH_SPEC>;
#[doc = "Channel DMA length"]
pub mod dmac_ch0_dmalength;
#[doc = "DMAC_STATUS (r) register accessor: DMAC status This register provides the actual state of each DMA channel. It also reports port errors in case these were received by the master interface module during the data transfer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_status`]
module"]
pub type DMAC_STATUS = crate::Reg<dmac_status::DMAC_STATUS_SPEC>;
#[doc = "DMAC status This register provides the actual state of each DMA channel. It also reports port errors in case these were received by the master interface module during the data transfer."]
pub mod dmac_status;
#[doc = "DMAC_SWRES (w) register accessor: DMAC software reset register Software reset is used to reset the DMAC to stop all transfers and clears the port error status register. After the software reset is performed, all the channels are disabled and no new requests are performed by the channels. The DMAC waits for the existing (active) requests to finish and accordingly sets the DMAC status registers.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_swres::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_swres`]
module"]
pub type DMAC_SWRES = crate::Reg<dmac_swres::DMAC_SWRES_SPEC>;
#[doc = "DMAC software reset register Software reset is used to reset the DMAC to stop all transfers and clears the port error status register. After the software reset is performed, all the channels are disabled and no new requests are performed by the channels. The DMAC waits for the existing (active) requests to finish and accordingly sets the DMAC status registers."]
pub mod dmac_swres;
#[doc = "DMAC_CH1_CTRL (rw) register accessor: Channel control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_ch1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_ch1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_ch1_ctrl`]
module"]
pub type DMAC_CH1_CTRL = crate::Reg<dmac_ch1_ctrl::DMAC_CH1_CTRL_SPEC>;
#[doc = "Channel control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
pub mod dmac_ch1_ctrl;
#[doc = "DMAC_CH1_EXTADDR (rw) register accessor: Channel external address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_ch1_extaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_ch1_extaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_ch1_extaddr`]
module"]
pub type DMAC_CH1_EXTADDR = crate::Reg<dmac_ch1_extaddr::DMAC_CH1_EXTADDR_SPEC>;
#[doc = "Channel external address"]
pub mod dmac_ch1_extaddr;
#[doc = "DMAC_CH1_DMALENGTH (rw) register accessor: Channel DMA length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_ch1_dmalength::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_ch1_dmalength::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_ch1_dmalength`]
module"]
pub type DMAC_CH1_DMALENGTH = crate::Reg<dmac_ch1_dmalength::DMAC_CH1_DMALENGTH_SPEC>;
#[doc = "Channel DMA length"]
pub mod dmac_ch1_dmalength;
#[doc = "DMAC_MST_RUNPARAMS (rw) register accessor: DMAC master run-time parameters This register defines all the run-time parameters for the AHB master interface port. These parameters are required for the proper functioning of the EIP-101m AHB master adapter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_mst_runparams::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_mst_runparams::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_mst_runparams`]
module"]
pub type DMAC_MST_RUNPARAMS = crate::Reg<dmac_mst_runparams::DMAC_MST_RUNPARAMS_SPEC>;
#[doc = "DMAC master run-time parameters This register defines all the run-time parameters for the AHB master interface port. These parameters are required for the proper functioning of the EIP-101m AHB master adapter."]
pub mod dmac_mst_runparams;
#[doc = "DMAC_PERSR (r) register accessor: DMAC port error raw status register This register provides the actual status of individual port errors. It also indicates which channel is serviced by an external AHB port (which is frozen by a port error). A port error aborts operations on all serviced channels (channel enable bit is forced to 0) and prevents further transfers via that port until the error is cleared by writing to the DMAC_SWRES register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_persr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_persr`]
module"]
pub type DMAC_PERSR = crate::Reg<dmac_persr::DMAC_PERSR_SPEC>;
#[doc = "DMAC port error raw status register This register provides the actual status of individual port errors. It also indicates which channel is serviced by an external AHB port (which is frozen by a port error). A port error aborts operations on all serviced channels (channel enable bit is forced to 0) and prevents further transfers via that port until the error is cleared by writing to the DMAC_SWRES register."]
pub mod dmac_persr;
#[doc = "DMAC_OPTIONS (r) register accessor: DMAC options register These registers contain information regarding the different options configured in this DMAC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_options::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_options`]
module"]
pub type DMAC_OPTIONS = crate::Reg<dmac_options::DMAC_OPTIONS_SPEC>;
#[doc = "DMAC options register These registers contain information regarding the different options configured in this DMAC."]
pub mod dmac_options;
#[doc = "DMAC_VERSION (r) register accessor: DMAC version register This register contains an indication (or signature) of the EIP type of this DMAC, as well as the hardware version/patch numbers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_version::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_version`]
module"]
pub type DMAC_VERSION = crate::Reg<dmac_version::DMAC_VERSION_SPEC>;
#[doc = "DMAC version register This register contains an indication (or signature) of the EIP type of this DMAC, as well as the hardware version/patch numbers."]
pub mod dmac_version;
#[doc = "KEY_STORE_WRITE_AREA (rw) register accessor: Key store write area register This register defines where the keys should be written in the key store RAM. After writing this register, the key store module is ready to receive the keys through a DMA operation. In case the key data transfer triggered an error in the key store, the error will be available in the interrupt status register after the DMA is finished. The key store write-error is asserted when the programmed/selected area is not completely written. This error is also asserted when the DMA operation writes to ram areas that are not selected. The key store RAM is divided into 8 areas of 128 bits. 192-bit keys written in the key store RAM should start on boundaries of 256 bits. This means that writing a 192-bit key to the key store RAM must be done by writing 256 bits of data with the 64 most-significant bits set to 0. These bits are ignored by the AES engine.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key_store_write_area::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key_store_write_area::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_store_write_area`]
module"]
pub type KEY_STORE_WRITE_AREA = crate::Reg<key_store_write_area::KEY_STORE_WRITE_AREA_SPEC>;
#[doc = "Key store write area register This register defines where the keys should be written in the key store RAM. After writing this register, the key store module is ready to receive the keys through a DMA operation. In case the key data transfer triggered an error in the key store, the error will be available in the interrupt status register after the DMA is finished. The key store write-error is asserted when the programmed/selected area is not completely written. This error is also asserted when the DMA operation writes to ram areas that are not selected. The key store RAM is divided into 8 areas of 128 bits. 192-bit keys written in the key store RAM should start on boundaries of 256 bits. This means that writing a 192-bit key to the key store RAM must be done by writing 256 bits of data with the 64 most-significant bits set to 0. These bits are ignored by the AES engine."]
pub mod key_store_write_area;
#[doc = "KEY_STORE_WRITTEN_AREA (rw) register accessor: Key store written area register This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and results in an error.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key_store_written_area::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key_store_written_area::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_store_written_area`]
module"]
pub type KEY_STORE_WRITTEN_AREA = crate::Reg<key_store_written_area::KEY_STORE_WRITTEN_AREA_SPEC>;
#[doc = "Key store written area register This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and results in an error."]
pub mod key_store_written_area;
#[doc = "KEY_STORE_SIZE (rw) register accessor: Key store size register This register defines the size of the keys that are written with DMA. This register should be configured before writing to the KEY_STORE_WRITE_AREA register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key_store_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key_store_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_store_size`]
module"]
pub type KEY_STORE_SIZE = crate::Reg<key_store_size::KEY_STORE_SIZE_SPEC>;
#[doc = "Key store size register This register defines the size of the keys that are written with DMA. This register should be configured before writing to the KEY_STORE_WRITE_AREA register."]
pub mod key_store_size;
#[doc = "KEY_STORE_READ_AREA (rw) register accessor: Key store read area register This register selects the key store RAM area from where the key needs to be read that will be used for an AES operation. The operation directly starts after writing this register. When the operation is finished, the status of the key store read operation is available in the interrupt status register. Key store read error is asserted when a RAM area is selected which does not contain valid written key.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key_store_read_area::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key_store_read_area::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_store_read_area`]
module"]
pub type KEY_STORE_READ_AREA = crate::Reg<key_store_read_area::KEY_STORE_READ_AREA_SPEC>;
#[doc = "Key store read area register This register selects the key store RAM area from where the key needs to be read that will be used for an AES operation. The operation directly starts after writing this register. When the operation is finished, the status of the key store read operation is available in the interrupt status register. Key store read error is asserted when a RAM area is selected which does not contain valid written key."]
pub mod key_store_read_area;
#[doc = "AES_KEY2_0 (w) register accessor: AES_KEY2_0 / AES_GHASH_H_IN_0 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_key2_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_key2_0`]
module"]
pub type AES_KEY2_0 = crate::Reg<aes_key2_0::AES_KEY2_0_SPEC>;
#[doc = "AES_KEY2_0 / AES_GHASH_H_IN_0 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aes_key2_0;
#[doc = "AES_KEY2_1 (w) register accessor: AES_KEY2_1 / AES_GHASH_H_IN_1 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_key2_1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_key2_1`]
module"]
pub type AES_KEY2_1 = crate::Reg<aes_key2_1::AES_KEY2_1_SPEC>;
#[doc = "AES_KEY2_1 / AES_GHASH_H_IN_1 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aes_key2_1;
#[doc = "AES_KEY2_2 (w) register accessor: AES_KEY2_2 / AES_GHASH_H_IN_2 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_key2_2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_key2_2`]
module"]
pub type AES_KEY2_2 = crate::Reg<aes_key2_2::AES_KEY2_2_SPEC>;
#[doc = "AES_KEY2_2 / AES_GHASH_H_IN_2 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aes_key2_2;
#[doc = "AES_KEY2_3 (w) register accessor: AES_KEY2_3 / AES_GHASH_H_IN_3 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_key2_3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_key2_3`]
module"]
pub type AES_KEY2_3 = crate::Reg<aes_key2_3::AES_KEY2_3_SPEC>;
#[doc = "AES_KEY2_3 / AES_GHASH_H_IN_3 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aes_key2_3;
#[doc = "AES_KEY3_0 (w) register accessor: AES_KEY3_0 / AES_KEY2_4 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_key3_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_key3_0`]
module"]
pub type AES_KEY3_0 = crate::Reg<aes_key3_0::AES_KEY3_0_SPEC>;
#[doc = "AES_KEY3_0 / AES_KEY2_4 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aes_key3_0;
#[doc = "AES_KEY3_1 (w) register accessor: AES_KEY3_1 / AES_KEY2_5 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_key3_1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_key3_1`]
module"]
pub type AES_KEY3_1 = crate::Reg<aes_key3_1::AES_KEY3_1_SPEC>;
#[doc = "AES_KEY3_1 / AES_KEY2_5 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aes_key3_1;
#[doc = "AES_KEY3_2 (w) register accessor: AES_KEY3_2 / AES_KEY2_6 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_key3_2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_key3_2`]
module"]
pub type AES_KEY3_2 = crate::Reg<aes_key3_2::AES_KEY3_2_SPEC>;
#[doc = "AES_KEY3_2 / AES_KEY2_6 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aes_key3_2;
#[doc = "AES_KEY3_3 (w) register accessor: AES_KEY3_3 / AES_KEY2_7 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_key3_3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_key3_3`]
module"]
pub type AES_KEY3_3 = crate::Reg<aes_key3_3::AES_KEY3_3_SPEC>;
#[doc = "AES_KEY3_3 / AES_KEY2_7 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aes_key3_3;
#[doc = "AES_IV_0 (rw) register accessor: AES initialization vector registers These registers are used to provide and read the IV from the AES engine.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_iv_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_iv_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_iv_0`]
module"]
pub type AES_IV_0 = crate::Reg<aes_iv_0::AES_IV_0_SPEC>;
#[doc = "AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
pub mod aes_iv_0;
#[doc = "AES_IV_1 (rw) register accessor: AES initialization vector registers These registers are used to provide and read the IV from the AES engine.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_iv_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_iv_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_iv_1`]
module"]
pub type AES_IV_1 = crate::Reg<aes_iv_1::AES_IV_1_SPEC>;
#[doc = "AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
pub mod aes_iv_1;
#[doc = "AES_IV_2 (rw) register accessor: AES initialization vector registers These registers are used to provide and read the IV from the AES engine.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_iv_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_iv_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_iv_2`]
module"]
pub type AES_IV_2 = crate::Reg<aes_iv_2::AES_IV_2_SPEC>;
#[doc = "AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
pub mod aes_iv_2;
#[doc = "AES_IV_3 (rw) register accessor: AES initialization vector registers These registers are used to provide and read the IV from the AES engine.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_iv_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_iv_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_iv_3`]
module"]
pub type AES_IV_3 = crate::Reg<aes_iv_3::AES_IV_3_SPEC>;
#[doc = "AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
pub mod aes_iv_3;
#[doc = "AES_CTRL (rw) register accessor: AES input/output buffer control and mode register This register specifies the AES mode of operation for the EIP-120t. Electronic codebook (ECB) mode is automatically selected if bits \\[28:5\\]
of this register are all 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_ctrl`]
module"]
pub type AES_CTRL = crate::Reg<aes_ctrl::AES_CTRL_SPEC>;
#[doc = "AES input/output buffer control and mode register This register specifies the AES mode of operation for the EIP-120t. Electronic codebook (ECB) mode is automatically selected if bits \\[28:5\\]
of this register are all 0."]
pub mod aes_ctrl;
#[doc = "AES_C_LENGTH_0 (rw) register accessor: AES crypto length registers (LSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM, and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_c_length_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_c_length_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_c_length_0`]
module"]
pub type AES_C_LENGTH_0 = crate::Reg<aes_c_length_0::AES_C_LENGTH_0_SPEC>;
#[doc = "AES crypto length registers (LSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM, and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
pub mod aes_c_length_0;
#[doc = "AES_C_LENGTH_1 (w) register accessor: AES crypto length registers (MSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_c_length_1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_c_length_1`]
module"]
pub type AES_C_LENGTH_1 = crate::Reg<aes_c_length_1::AES_C_LENGTH_1_SPEC>;
#[doc = "AES crypto length registers (MSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
pub mod aes_c_length_1;
#[doc = "AES_AUTH_LENGTH (w) register accessor: Authentication length register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_auth_length::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_auth_length`]
module"]
pub type AES_AUTH_LENGTH = crate::Reg<aes_auth_length::AES_AUTH_LENGTH_SPEC>;
#[doc = "Authentication length register"]
pub mod aes_auth_length;
#[doc = "AES_DATA_IN_OUT_0 (w) register accessor: Data input/output registers The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_data_in_out_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_data_in_out_0`]
module"]
pub type AES_DATA_IN_OUT_0 = crate::Reg<aes_data_in_out_0::AES_DATA_IN_OUT_0_SPEC>;
#[doc = "Data input/output registers The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aes_data_in_out_0;
#[doc = "AES_DATA_IN_OUT_1 (w) register accessor: Data Input/Output Registers The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_data_in_out_1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_data_in_out_1`]
module"]
pub type AES_DATA_IN_OUT_1 = crate::Reg<aes_data_in_out_1::AES_DATA_IN_OUT_1_SPEC>;
#[doc = "Data Input/Output Registers The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aes_data_in_out_1;
#[doc = "AES_DATA_IN_OUT_2 (w) register accessor: Data Input/Output Registers The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_data_in_out_2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_data_in_out_2`]
module"]
pub type AES_DATA_IN_OUT_2 = crate::Reg<aes_data_in_out_2::AES_DATA_IN_OUT_2_SPEC>;
#[doc = "Data Input/Output Registers The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aes_data_in_out_2;
#[doc = "AES_DATA_IN_OUT_3 (w) register accessor: Data Input/Output Registers The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_data_in_out_3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_data_in_out_3`]
module"]
pub type AES_DATA_IN_OUT_3 = crate::Reg<aes_data_in_out_3::AES_DATA_IN_OUT_3_SPEC>;
#[doc = "Data Input/Output Registers The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aes_data_in_out_3;
#[doc = "AES_TAG_OUT_0 (r) register accessor: TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order will return the IV twice.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_tag_out_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_tag_out_0`]
module"]
pub type AES_TAG_OUT_0 = crate::Reg<aes_tag_out_0::AES_TAG_OUT_0_SPEC>;
#[doc = "TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order will return the IV twice."]
pub mod aes_tag_out_0;
#[doc = "AES_TAG_OUT_1 (r) register accessor: TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order returns the IV twice.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_tag_out_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_tag_out_1`]
module"]
pub type AES_TAG_OUT_1 = crate::Reg<aes_tag_out_1::AES_TAG_OUT_1_SPEC>;
#[doc = "TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order returns the IV twice."]
pub mod aes_tag_out_1;
#[doc = "AES_TAG_OUT_2 (r) register accessor: TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order returns the IV twice.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_tag_out_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_tag_out_2`]
module"]
pub type AES_TAG_OUT_2 = crate::Reg<aes_tag_out_2::AES_TAG_OUT_2_SPEC>;
#[doc = "TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order returns the IV twice."]
pub mod aes_tag_out_2;
#[doc = "AES_TAG_OUT_3 (r) register accessor: TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order returns the IV twice.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_tag_out_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_tag_out_3`]
module"]
pub type AES_TAG_OUT_3 = crate::Reg<aes_tag_out_3::AES_TAG_OUT_3_SPEC>;
#[doc = "TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order returns the IV twice."]
pub mod aes_tag_out_3;
#[doc = "HASH_DATA_IN_0 (w) register accessor: HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_data_in_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_data_in_0`]
module"]
pub type HASH_DATA_IN_0 = crate::Reg<hash_data_in_0::HASH_DATA_IN_0_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_0;
#[doc = "HASH_DATA_IN_1 (w) register accessor: HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_data_in_1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_data_in_1`]
module"]
pub type HASH_DATA_IN_1 = crate::Reg<hash_data_in_1::HASH_DATA_IN_1_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_1;
#[doc = "HASH_DATA_IN_2 (w) register accessor: HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_data_in_2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_data_in_2`]
module"]
pub type HASH_DATA_IN_2 = crate::Reg<hash_data_in_2::HASH_DATA_IN_2_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_2;
#[doc = "HASH_DATA_IN_3 (w) register accessor: HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_data_in_3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_data_in_3`]
module"]
pub type HASH_DATA_IN_3 = crate::Reg<hash_data_in_3::HASH_DATA_IN_3_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_3;
#[doc = "HASH_DATA_IN_4 (w) register accessor: HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_data_in_4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_data_in_4`]
module"]
pub type HASH_DATA_IN_4 = crate::Reg<hash_data_in_4::HASH_DATA_IN_4_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_4;
#[doc = "HASH_DATA_IN_5 (w) register accessor: HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_data_in_5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_data_in_5`]
module"]
pub type HASH_DATA_IN_5 = crate::Reg<hash_data_in_5::HASH_DATA_IN_5_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_5;
#[doc = "HASH_DATA_IN_6 (w) register accessor: HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_data_in_6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_data_in_6`]
module"]
pub type HASH_DATA_IN_6 = crate::Reg<hash_data_in_6::HASH_DATA_IN_6_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_6;
#[doc = "HASH_DATA_IN_7 (w) register accessor: HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_data_in_7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_data_in_7`]
module"]
pub type HASH_DATA_IN_7 = crate::Reg<hash_data_in_7::HASH_DATA_IN_7_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_7;
#[doc = "HASH_DATA_IN_8 (w) register accessor: HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_data_in_8::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_data_in_8`]
module"]
pub type HASH_DATA_IN_8 = crate::Reg<hash_data_in_8::HASH_DATA_IN_8_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_8;
#[doc = "HASH_DATA_IN_9 (w) register accessor: HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_data_in_9::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_data_in_9`]
module"]
pub type HASH_DATA_IN_9 = crate::Reg<hash_data_in_9::HASH_DATA_IN_9_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_9;
#[doc = "HASH_DATA_IN_10 (w) register accessor: HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_data_in_10::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_data_in_10`]
module"]
pub type HASH_DATA_IN_10 = crate::Reg<hash_data_in_10::HASH_DATA_IN_10_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_10;
#[doc = "HASH_DATA_IN_11 (w) register accessor: HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_data_in_11::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_data_in_11`]
module"]
pub type HASH_DATA_IN_11 = crate::Reg<hash_data_in_11::HASH_DATA_IN_11_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_11;
#[doc = "HASH_DATA_IN_12 (w) register accessor: HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_data_in_12::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_data_in_12`]
module"]
pub type HASH_DATA_IN_12 = crate::Reg<hash_data_in_12::HASH_DATA_IN_12_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_12;
#[doc = "HASH_DATA_IN_13 (w) register accessor: HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_data_in_13::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_data_in_13`]
module"]
pub type HASH_DATA_IN_13 = crate::Reg<hash_data_in_13::HASH_DATA_IN_13_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_13;
#[doc = "HASH_DATA_IN_14 (w) register accessor: HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_data_in_14::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_data_in_14`]
module"]
pub type HASH_DATA_IN_14 = crate::Reg<hash_data_in_14::HASH_DATA_IN_14_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_14;
#[doc = "HASH_DATA_IN_15 (w) register accessor: HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_data_in_15::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_data_in_15`]
module"]
pub type HASH_DATA_IN_15 = crate::Reg<hash_data_in_15::HASH_DATA_IN_15_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_15;
#[doc = "HASH_IO_BUF_CTRL (rw) register accessor: Input/output buffer control and status register This register pair shares a single address location and contains bits that control and monitor the data flow between the host and the hash engine.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_io_buf_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_io_buf_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_io_buf_ctrl`]
module"]
pub type HASH_IO_BUF_CTRL = crate::Reg<hash_io_buf_ctrl::HASH_IO_BUF_CTRL_SPEC>;
#[doc = "Input/output buffer control and status register This register pair shares a single address location and contains bits that control and monitor the data flow between the host and the hash engine."]
pub mod hash_io_buf_ctrl;
#[doc = "HASH_MODE_IN (w) register accessor: Hash mode register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_mode_in::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_mode_in`]
module"]
pub type HASH_MODE_IN = crate::Reg<hash_mode_in::HASH_MODE_IN_SPEC>;
#[doc = "Hash mode register"]
pub mod hash_mode_in;
#[doc = "HASH_LENGTH_IN_L (w) register accessor: Hash length register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_length_in_l::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_length_in_l`]
module"]
pub type HASH_LENGTH_IN_L = crate::Reg<hash_length_in_l::HASH_LENGTH_IN_L_SPEC>;
#[doc = "Hash length register"]
pub mod hash_length_in_l;
#[doc = "HASH_LENGTH_IN_H (w) register accessor: Hash length register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_length_in_h::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_length_in_h`]
module"]
pub type HASH_LENGTH_IN_H = crate::Reg<hash_length_in_h::HASH_LENGTH_IN_H_SPEC>;
#[doc = "Hash length register"]
pub mod hash_length_in_h;
#[doc = "HASH_DIGEST_A (rw) register accessor: Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_digest_a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_digest_a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_digest_a`]
module"]
pub type HASH_DIGEST_A = crate::Reg<hash_digest_a::HASH_DIGEST_A_SPEC>;
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hash_digest_a;
#[doc = "HASH_DIGEST_B (rw) register accessor: Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_digest_b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_digest_b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_digest_b`]
module"]
pub type HASH_DIGEST_B = crate::Reg<hash_digest_b::HASH_DIGEST_B_SPEC>;
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hash_digest_b;
#[doc = "HASH_DIGEST_C (rw) register accessor: Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_digest_c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_digest_c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_digest_c`]
module"]
pub type HASH_DIGEST_C = crate::Reg<hash_digest_c::HASH_DIGEST_C_SPEC>;
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hash_digest_c;
#[doc = "HASH_DIGEST_D (rw) register accessor: Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_digest_d::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_digest_d::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_digest_d`]
module"]
pub type HASH_DIGEST_D = crate::Reg<hash_digest_d::HASH_DIGEST_D_SPEC>;
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hash_digest_d;
#[doc = "HASH_DIGEST_E (rw) register accessor: Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_digest_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_digest_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_digest_e`]
module"]
pub type HASH_DIGEST_E = crate::Reg<hash_digest_e::HASH_DIGEST_E_SPEC>;
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hash_digest_e;
#[doc = "HASH_DIGEST_F (rw) register accessor: Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_digest_f::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_digest_f::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_digest_f`]
module"]
pub type HASH_DIGEST_F = crate::Reg<hash_digest_f::HASH_DIGEST_F_SPEC>;
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hash_digest_f;
#[doc = "HASH_DIGEST_G (rw) register accessor: Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_digest_g::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_digest_g::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_digest_g`]
module"]
pub type HASH_DIGEST_G = crate::Reg<hash_digest_g::HASH_DIGEST_G_SPEC>;
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hash_digest_g;
#[doc = "HASH_DIGEST_H (rw) register accessor: Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_digest_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_digest_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_digest_h`]
module"]
pub type HASH_DIGEST_H = crate::Reg<hash_digest_h::HASH_DIGEST_H_SPEC>;
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hash_digest_h;
#[doc = "CTRL_ALG_SEL (rw) register accessor: Algorithm select This algorithm selection register configures the internal destination of the DMA controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_alg_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_alg_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_alg_sel`]
module"]
pub type CTRL_ALG_SEL = crate::Reg<ctrl_alg_sel::CTRL_ALG_SEL_SPEC>;
#[doc = "Algorithm select This algorithm selection register configures the internal destination of the DMA controller."]
pub mod ctrl_alg_sel;
#[doc = "CTRL_PROT_EN (rw) register accessor: Master PROT privileged access enable This register enables the second bit (bit \\[1\\]) of the AHB HPROT bus of the AHB master interface when a read action of key(s) is performed on the AHB master interface for writing keys into the store module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_prot_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_prot_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_prot_en`]
module"]
pub type CTRL_PROT_EN = crate::Reg<ctrl_prot_en::CTRL_PROT_EN_SPEC>;
#[doc = "Master PROT privileged access enable This register enables the second bit (bit \\[1\\]) of the AHB HPROT bus of the AHB master interface when a read action of key(s) is performed on the AHB master interface for writing keys into the store module."]
pub mod ctrl_prot_en;
#[doc = "CTRL_SW_RESET (rw) register accessor: Software reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_sw_reset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_sw_reset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_sw_reset`]
module"]
pub type CTRL_SW_RESET = crate::Reg<ctrl_sw_reset::CTRL_SW_RESET_SPEC>;
#[doc = "Software reset"]
pub mod ctrl_sw_reset;
#[doc = "CTRL_INT_CFG (rw) register accessor: Interrupt configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_int_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_int_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_int_cfg`]
module"]
pub type CTRL_INT_CFG = crate::Reg<ctrl_int_cfg::CTRL_INT_CFG_SPEC>;
#[doc = "Interrupt configuration"]
pub mod ctrl_int_cfg;
#[doc = "CTRL_INT_EN (rw) register accessor: Interrupt enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_int_en`]
module"]
pub type CTRL_INT_EN = crate::Reg<ctrl_int_en::CTRL_INT_EN_SPEC>;
#[doc = "Interrupt enable"]
pub mod ctrl_int_en;
#[doc = "CTRL_INT_CLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_int_clr`]
module"]
pub type CTRL_INT_CLR = crate::Reg<ctrl_int_clr::CTRL_INT_CLR_SPEC>;
#[doc = "Interrupt clear"]
pub mod ctrl_int_clr;
#[doc = "CTRL_INT_SET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_int_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_int_set`]
module"]
pub type CTRL_INT_SET = crate::Reg<ctrl_int_set::CTRL_INT_SET_SPEC>;
#[doc = "Interrupt set"]
pub mod ctrl_int_set;
#[doc = "CTRL_INT_STAT (r) register accessor: Interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_int_stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_int_stat`]
module"]
pub type CTRL_INT_STAT = crate::Reg<ctrl_int_stat::CTRL_INT_STAT_SPEC>;
#[doc = "Interrupt status"]
pub mod ctrl_int_stat;
#[doc = "CTRL_OPTIONS (r) register accessor: Options register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_options::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_options`]
module"]
pub type CTRL_OPTIONS = crate::Reg<ctrl_options::CTRL_OPTIONS_SPEC>;
#[doc = "Options register"]
pub mod ctrl_options;
#[doc = "CTRL_VERSION (r) register accessor: Version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_version::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_version`]
module"]
pub type CTRL_VERSION = crate::Reg<ctrl_version::CTRL_VERSION_SPEC>;
#[doc = "Version register"]
pub mod ctrl_version;
