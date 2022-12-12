// See the "macOS permissions note" in README.md before running this on macOS
// Big Sur or later.

use btleplug::api::{Central, CharPropFlags, Characteristic, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;
use enigo::{Enigo, Key, KeyboardControllable};
use futures::stream::StreamExt;
use std::error::Error;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;

const NOTIFY_SERVICE_UUID: Uuid = Uuid::from_u128(0x0000ffe0_0000_1000_8000_00805f9b34fb);
const NOTIFY_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x0000ffe1_0000_1000_8000_00805f9b34fb);

async fn main_loop() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        eprintln!("No Bluetooth adapters found");
    }

    for adapter in adapter_list.iter() {
        println!("Starting scan...");
        adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("Can't scan BLE adapter for connected devices...");
        time::sleep(Duration::from_secs(2)).await;
        let peripherals = adapter.peripherals().await?;

        if peripherals.is_empty() {
            eprintln!("->>> BLE peripheral devices were not found, sorry. Exiting...");
        } else {
            // All peripheral devices in range.
            for peripheral in peripherals.iter() {
                let properties = peripheral.properties().await?.unwrap();
                let address = properties.address.to_string();
                let services = properties.services;

                if services.contains(&NOTIFY_SERVICE_UUID) {
                    println!("Found matching peripheral {:?}...", &address);

                    if let Err(err) = peripheral.connect().await {
                        eprintln!("Error connecting to peripheral, skipping: {}", err);
                        continue;
                    }

                    if peripheral.is_connected().await? {
                        println!("Now connected to peripheral {:?}.", &address);

                        // We create the keyboard once we are connected.
                        let mut enigo = Enigo::new();

                        peripheral.discover_services().await?;

                        let characteristic = Characteristic {
                            uuid: NOTIFY_CHARACTERISTIC_UUID,
                            service_uuid: NOTIFY_SERVICE_UUID,
                            properties: CharPropFlags::READ | CharPropFlags::NOTIFY,
                        };
                        peripheral.subscribe(&characteristic).await?;

                        // CTRL+C signal for exit
                        /* tokio::spawn(async move {
                            tokio::signal::ctrl_c().await.unwrap();
                            println!("Disconnecting...");
                            peripheral.disconnect().await;
                        }); */

                        // Listen for button presses
                        let mut notification_stream = peripheral.notifications().await?;
                        while let Some(_) = notification_stream.next().await {
                            println!("Button pushed on {:?}", address);
                            enigo.key_down(Key::RightArrow);
                            enigo.key_up(Key::RightArrow);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(main_loop())
}
