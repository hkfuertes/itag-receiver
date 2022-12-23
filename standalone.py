# -*- coding: utf-8 -*-
import asyncio
from bleak import BleakClient, BleakScanner

class IBleConfiguration:
    def getAddress(self) -> str:
        pass
    def getCharasteristicUuid(self) -> str:
        pass
    def onButtonPressed(self,*_):
        pass

class BleConfiguration(IBleConfiguration):
    def __init__(self, address, charasteristic_uuid):
        self.address = address
        self.charasteristic_uuid = charasteristic_uuid
    
    def getAddress(self) -> str:
        return self.address
    
    def getCharasteristicUuid(self) -> str:
        return self.charasteristic_uuid
    
    def onButtonPressed(self, *_):
        print("[+] Button pressed on %s" % config.getAddress())

async def listen(config: IBleConfiguration):
    device = await BleakScanner.find_device_by_address(config.getAddress(), cb=dict(use_bdaddr=True))
    if device is None:
        print("[!] Could not find device with address '%s'" % config.getAddress())
        return

    disconnected_event = asyncio.Event()

    def disconnected_callback(client):
        print("[!] Device %s disconnected!" % config.getAddress())
        disconnected_event.set()

    async with BleakClient(device, disconnected_callback=disconnected_callback) as client:
        # print("[i] Connected to %s" % config.getAddress())

        await client.start_notify(config.getCharasteristicUuid(), config.onButtonPressed)
        
        try:
            await disconnected_event.wait()
        except asyncio.exceptions.CancelledError:
            await client.stop_notify(config.getCharasteristicUuid())

if __name__ == "__main__":
    config = BleConfiguration(
        address="FF:FF:10:38:20:A3",
        charasteristic_uuid= "0000ffe1-0000-1000-8000-00805f9b34fb"
    )
    asyncio.run(listen(config))
