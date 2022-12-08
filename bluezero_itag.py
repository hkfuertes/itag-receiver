from bluezero import adapter, central

ITAG_SRV = '0000ffe0-0000-1000-8000-00805f9b34fb'
ITAG_CHAR_UUID = '0000ffe1-0000-1000-8000-00805f9b34fb'

def scan_for_itag(
        adapter_address=None,
        dev_address=None,
        timeout=5.0):
    """
    Called to scan for BLE devices advertising the Heartrate Service UUID
    If there are multiple adapters on your system, this will scan using
    all dongles unless an adapter is specfied through its MAC address
    :param adapter_address: limit scanning to this adapter MAC address
    :param dev_address: scan for a specific peripheral MAC address
    :param timeout: how long to search for devices in seconds
    :return: generator of Devices that match the search parameters
    """
    # If there are multiple adapters on your system, this will scan using
    # all dongles unless an adapter is specified through its MAC address
    for dongle in adapter.Adapter.available():
        # Filter dongles by adapter_address if specified
        if adapter_address and adapter_address.upper() != dongle.address():
            continue

        # Actually listen to nearby advertisements for timeout seconds
        dongle.nearby_discovery(timeout=timeout)

        # Iterate through discovered devices
        for dev in central.Central.available(dongle.address):
            # Filter devices if we specified an address
            if dev_address and dev_address == dev.address:
                yield dev

            # Otherwise, return devices that advertised the Service UUID
            if ITAG_SRV.lower() in dev.uuids:
                yield dev


def on_ITAG_button_pushed(iface, changed_props, invalidated_props):
    """
    Callback used to receive notification events from the device.
    :param iface: dbus advanced data
    :param changed_props: updated properties for this event, contains Value
    :param invalidated_props: dvus advanced data
    """
    print('Pushed!')
    return

    value = changed_props.get('Value', None)
    if not value:
        return


def connect_and_run(dev=None, device_address=None):
    """
    Main function intneded to show usage of central.Central
    :param dev: Device to connect to if scan was performed
    :param device_address: instead, connect to a specific MAC address
    """
    # Create Interface to Central
    if dev:
        monitor = central.Central(
            adapter_addr=dev.adapter,
            device_addr=dev.address)
    else:
        monitor = central.Central(device_addr=device_address)

    # Characteristics that we're interested must be added to the Central
    # before we connect so they automatically resolve BLE properties
    # Heart Rate Measurement - notify
    measurement_char = monitor.add_characteristic(ITAG_SRV, ITAG_CHAR_UUID)

    # Now Connect to the Device
    if dev:
        print("Connecting to " + dev.alias)
    else:
        print("Connecting to " + device_address)

    monitor.connect()

    # Check if Connected Successfully
    if not monitor.connected:
        print("Didn't connect to device!")
        return

    # Enable heart rate notifications
    measurement_char.start_notify()
    measurement_char.add_characteristic_cb(on_ITAG_button_pushed)

    try:
        # Startup in async mode to enable notify, etc
        monitor.run()
    except KeyboardInterrupt:
        print("Disconnecting")

    measurement_char.stop_notify()
    monitor.disconnect()


if __name__ == '__main__':
    devices = scan_for_itag()
    for dev in devices:
        connect_and_run(dev)

        # Only demo the first device found
        break
