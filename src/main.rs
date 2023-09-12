#![deny(unsafe_code)]
#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

#[rtic::app(device = stm32f1xx_hal::pac)]
mod app {
    use crate::display::SevenSeg;
    use stm32f1xx_hal::{
        gpio::{
            gpiob::{PB12, PB13, PB15},
            gpioe::{PE10, PE11, PE12, PE13, PE14, PE15},
            OpenDrain, Output,
        },
        pac,
        prelude::*,
        timer::{CounterUs, Event},
    };

    #[shared]
    struct Shared {
        timer_handler: CounterUs<pac::TIM2>,
        flag: bool,
    }

    #[local]
    struct Local {
        channel: u8,
        display: Option<
            SevenSeg<
                PB13<Output<OpenDrain>>,
                PE13<Output<OpenDrain>>,
                PE12<Output<OpenDrain>>,
                PB15<Output<OpenDrain>>,
                PE15<Output<OpenDrain>>,
                PE14<Output<OpenDrain>>,
                PB12<Output<OpenDrain>>,
                PE11<Output<OpenDrain>>,
                PE10<Output<OpenDrain>>,
            >,
        >,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut flash = cx.device.FLASH.constrain();
        let rcc = cx.device.RCC.constrain();

        let clocks = rcc
            .cfgr
            .sysclk(36.MHz())
            .pclk1(36.MHz())
            .hclk(8.MHz())
            .freeze(&mut flash.acr);

        let mut gpioe = cx.device.GPIOE.split();
        let mut gpioc = cx.device.GPIOC.split();
        let mut gpiob = cx.device.GPIOB.split();
        
        let seg_a = gpiob.pb13.into_open_drain_output(&mut gpiob.crh);
        let seg_b = gpioe.pe13.into_open_drain_output(&mut gpioe.crh);
        let seg_c = gpioe.pe12.into_open_drain_output(&mut gpioe.crh);
        let seg_d = gpiob.pb15.into_open_drain_output(&mut gpiob.crh);
        let seg_e = gpioe.pe15.into_open_drain_output(&mut gpioe.crh);
        let seg_f = gpioe.pe14.into_open_drain_output(&mut gpioe.crh);
        let seg_g = gpiob.pb12.into_open_drain_output(&mut gpiob.crh);
        let en_t = gpioe.pe10.into_open_drain_output(&mut gpioe.crh);
        let en_u = gpioe.pe11.into_open_drain_output(&mut gpioe.crh);
        let display = SevenSeg::new(
            seg_a, seg_b, seg_c, seg_d, seg_e, seg_f, seg_g, en_u, en_t, false,
        );

        let mut timer = cx.device.TIM2.counter_us(&clocks);
        timer
            .start(1.millis())
            .map_err(|_| ())
            .expect("Timer setup error");
        timer.listen(Event::Update);
        
        let flag: bool = false;
        (
            Shared {
                timer_handler: timer,
                flag: flag,
            },
            Local {
                display: Some(display),
                channel: channel_address,
            },
            init::Monotonics(),
        )
    }

    #[idle(shared=[flag],local=[display,d_flag:u8=0,channel])]
    fn idle(mut cx: idle::Context) -> ! {
        let mut disp = cx
            .local
            .display
            .take()
            .expect("Failed to pass display to function");
        loop {
            {
                if cx.shared.flag.lock(|flag| *flag == true) {
                    let _ = disp.display(*cx.local.channel, *cx.local.d_flag);
                    *cx.local.d_flag += 1;
                    if *cx.local.d_flag == 20 {
                        *cx.local.d_flag = 1
                    };
                    cx.shared.flag.lock(|flag| *flag = false);
                }
            }
        }
    }
    
    #[task(binds=TIM2,priority=1, local=[],shared=[timer_handler,flag])]
    fn encvalget(mut cx: encvalget::Context) {
        cx.shared.flag.lock(|flag| {
            if *flag == false {
                *flag = true;
            }
        });
        cx.shared.timer_handler.lock(|timer| {
            timer.clear_interrupt(Event::Update);
        });
    }
}
