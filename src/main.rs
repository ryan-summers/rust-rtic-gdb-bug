#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::exception;

pub struct Container<TypeName> {
    object: TypeName,
}

pub struct Object<'d> {
    object: &'d str,
}

#[rtic::app(device = stm32h7xx_hal::stm32)]
const APP: () = {
    struct Resources {
        // Note: It appears that rustfmt may generate a format that GDB cannot recognize, which
        // results in GDB breakpoints being set improperly. To debug, redefine the following
        // definition to:
        //
        // ```rust
        // test_object: Container<Object<'static>>,
        // ```
        test_object: Container<Object<'static>,>,
    }

    #[init]
    fn init(_c: init::Context) -> init::LateResources {

        let test_object = {
            Container {
                object: Object { object: "test_d" },
            }
        };

        init::LateResources {
            test_object
        }
    }

    #[idle(resources=[])]
    fn idle(_c: idle::Context) -> ! {
        loop {}
    }
};

#[exception]
fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}
