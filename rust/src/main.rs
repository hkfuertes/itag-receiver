// See the "macOS permissions note" in README.md before running this on macOS
// Big Sur or later.

use btleplug::api::{Central, CharPropFlags, Manager as _, Peripheral, ScanFilter, Characteristic};
use btleplug::platform::Manager;
use futures::stream::StreamExt;
use std::error::Error;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;

const NOTIFY_SERVICE_UUID: Uuid = Uuid::from_u128(0x0000ffe0_0000_1000_8000_00805f9b34fb);
const NOTIFY_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x0000ffe1_0000_1000_8000_00805f9b34fb);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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

                if services.contains(&NOTIFY_SERVICE_UUID){
                    println!("Found matching peripheral {:?}...", &address);

                    if let Err(err) = peripheral.connect().await {
                        eprintln!("Error connecting to peripheral, skipping: {}", err);
                        continue;
                    }

                    if peripheral.is_connected().await? {
                        println!("Now connected to peripheral {:?}.", &address);

                        peripheral.discover_services().await?;
                        
                        let characteristic = Characteristic{
                            uuid: NOTIFY_CHARACTERISTIC_UUID,
                            service_uuid: NOTIFY_SERVICE_UUID,
                            properties: CharPropFlags::READ | CharPropFlags::NOTIFY
                        };
                        peripheral.subscribe(&characteristic).await?;
                        
                        let mut notification_stream = peripheral.notifications().await?;
                        while let Some(data) = notification_stream.next().await {
                            println!(
                                "Received data from {:?} [{:?}]: {:?}",
                                address, data.uuid, data.value
                            );
                        }
                        
                        println!("Disconnecting from peripheral {:?}...", address);
                        peripheral.disconnect().await?;
                    }
                }
            }
        }
    }
    Ok(())
}
