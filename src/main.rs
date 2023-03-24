#![no_std]
#![no_main]
#![allow(dead_code)]


// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

/**
 * \brief Resource Table Header
 */
pub struct RpmessageRscHdr {
    // Version Number, set to 1
    ver: u32,
    // Number of Entries, MUST be 2
    num: u32,
    // Reserved for future use, set to 0
    reserved: [u32; 2],
}

/**
 * \brief Structure used for remoteproc trace
 */
pub struct RpmessageRscTrace {
    // Type of trace, MUST be set to TYPE_TRACE | TRACE_INTS_VER0
    trace_type: u32,
    // Device Address, physical address of location of trace buffer in remote side
    da: u32,
    // Length of trace buffer
    len: u32,
    // Reserved for future use, set to 0
    reserved: u32,
    //  Name of the trace
    name: [u8; 32],
}

/**
 * \brief Resource Table Device VRing Structure
 */
pub struct RpmessageRscVring {
     // device address, physical address of VRING, 
     //   set to RPMSG_VRING_ADDR_ANY, updated by linux, with actual address  
     //
     da: u32,
    // Alignment used between AVAIL and USED structures, updated by linux
     align: u32,
    // Number of message buffers, MUST be 256
     num: u32,
    // NotifyId for receive channel, set 1 for TX VRING and 2 for RX VRING
     notifyid: u32,
    // Reserved for future use, set to 0
     reserved: u32,
}

/**
 *  \brief VDEV structure
 */
pub struct RpmessageRscVdev {
    // type of VDEV, set to TYPE_VDEV
    vdev_type: u32,
    // ID of VDEV, set to VIRTIO_ID_RPMSG
    id: u32,
    // Not used, set to 0
    notifyid: u32,
    // Not used, set to 1
    dfeatures: u32,
    // not used, set to 0
    gfeatures: u32,
    // not used, set to 0
    config_len: u32,
    // updated by linux, after linux init, this should be 0x7
    status: u8,
    // number of vrings, set to 2
    num_of_vrings: u8,
    // Reserved for future use, set to 0
    reserved: [u8; 2],
}

/**
 *  \brief IPC Resource Table used by IPC app
 */
pub struct RpmessageResourceTable {
    /**< Header Information */
    base: RpmessageRscHdr,
    /**< offset to VDEV and TRACE entries */
    offset: [u32; 2],
    /**< VDEV entry */
    vdev: RpmessageRscVdev,
    /**< TX VRING  */
    vring0: RpmessageRscVring,
    /**< RX VRING */
    vring1: RpmessageRscVring,
    /**< Trace entry  */
    trace: RpmessageRscTrace,
}

#[link_section = ".resource_table"]
#[no_mangle] pub static G_RPMESSAGE_LINUX_RESOURCE_TABLE : RpmessageResourceTable = RpmessageResourceTable {
	base: RpmessageRscHdr {
		ver: 1,
		num: 2,
		reserved: [0, 0],
	},
	offset: [0, 0],
	vdev: RpmessageRscVdev {
		vdev_type: 3,
		id: 7,
		notifyid: 0,
		dfeatures: 1,
		gfeatures: 1,
		config_len: 0,
		status: 0,
		num_of_vrings: 2,
		reserved: [0, 0],
	},
	vring0: RpmessageRscVring {
		da: 0xFFFFFFFF,
		align: 4096,
		num: 256,
		notifyid: 1,
		reserved: 0
	},
	vring1: RpmessageRscVring {
		da: 0xFFFFFFFF,
		align: 4096,
		num: 256,
		notifyid: 2,
		reserved: 0
	},
	trace: RpmessageRscTrace {
		trace_type: 2,
		da: 0x0,
		len: 0x0,
		reserved: 0,
		name: [b't', b'r', b'a', b'c', b'e', b':', 0, b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', ],
	},
};


#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    loop {
        // your code goes here
    }
}
